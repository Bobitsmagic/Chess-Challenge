use core::panic;
use std::collections::HashMap;

use crate::{game::{Game, GameState}, colored_piece_type::ColoredPieceType, piece_type::PieceType, bitboard_helper, endgame_table::EndgameTable, square::{self, Square}};

pub const CHECKMATE_VALUE: i32 = 1_000_000_000;
//                              Pawn, Knight, Bishop, Rook, Queen, King
const PIECE_MOBILITY_SCORE: [i32; 6] = [0, 80, 70, 50, 20, 0];
const PIECE_ATTACK_SCORE: [i32; 6] = [100, 22, 20, 15, 5, 1];
const SQUARE_CONTROL_SCORE: i32 = 50;

pub const PIECE_VALUES: [i32; 6] = [1000, 2800, 3200, 5000, 9000, CHECKMATE_VALUE];
const PAWN_PUSH_BONUS: [i32; 8] = [0, 0, 50, 70, 100, 150, 500, 0];

const EARLY_GAME_SQUARE_ATTACK_FACTOR: [i32; 16] = [
    0, 0, 0, 0, 
    0, 0, 0, 0,
    10, 30, 50, 80,
    20, 30, 100, 150,
];

const PASSED_PAWN_VALUE: i32 = 100;
const DOUBLED_PAWN_PENALTY: i32 = -150;
const ISOLATED_PAWN_PENALTY: i32 = -200;

const PAWN_CENTER_ATTACK_VALUE: i32 = 40;
const PIECE_CENTER_ATTACK_VALUE: i32 = 20;

const KING_ATTACK_PENALTY: i32 = -69;
const KING_DEFENCE_VALUE: i32 = 40;

const ATTACK_FACTOR: i32 = 30;

pub fn static_eval(game: &mut Game, do_print: bool) -> i32 {
    let gs = game.get_game_state();
    
    if do_print {
        println!("Evaluating: {}", game.get_board().get_fen());
        println!("GameState: {}", gs.to_string());
    }

    if gs.is_checkmate() {
        return -CHECKMATE_VALUE;
    }

    if gs.is_draw() {
        return match gs {
            GameState::Stalemate => 0,
            GameState::Repetition => 2, 
            GameState::InsuffMaterial => 3,
            GameState::FiftyMove => 5,
            _ => panic!("Invalid draw state")
        }
    }

    //whites perspective
    let board = game.get_board();

    if board.in_check() {
        println!("Check");
        return 1337;
    }

    let mut sum: i32 = 0;

    //Piece values
    if do_print {
        println!("Piece count: ")
    }

    let mut dif_sum: i32 = 0;
    let mut piece_count = [0_i32; 5];
    for i in 0..5 {
        let dif = board.get_piece_count(ColoredPieceType::from_u8(i * 2 + 0)) as i32 
            - board.get_piece_count(ColoredPieceType::from_u8(i * 2 + 1)) as i32;

        if do_print {
            println!("\t{} -> {} * {}", ColoredPieceType::from_u8(i * 2).get_char(), dif, PIECE_VALUES[i as usize]);
        }
        dif_sum += dif * PIECE_VALUES[i as usize];
    }
    if do_print {
        println!("\tsum: {}", dif_sum);
    }

    sum += dif_sum;
    dif_sum = 0;

    let white_list = board.generate_legal_moves_eval(true);
    let black_list = board.generate_legal_moves_eval(false);

    let mut capture_count = [0_i8; 64];
    let mut attack_score = [0_i32; 64];

    for m in &white_list {
        if m.is_capture(){
            capture_count[m.target_square as usize] += 1;
        }

        if m.is_attack() {
            attack_score[m.target_square as usize] += PIECE_ATTACK_SCORE[PieceType::from_cpt(m.move_piece_type) as usize];
        }
    }
    for m in &black_list {
        if m.is_capture() {
            capture_count[m.target_square as usize] -= 1;
        }

        if m.is_attack() {
            attack_score[m.target_square as usize] -= PIECE_ATTACK_SCORE[PieceType::from_cpt(m.move_piece_type) as usize];
        }
    }

    fn get_square_attack_factor(square: Square) -> i32 {
        let mut x = square.file();
        let mut y = square.rank();

        if x >= 4 {
            x = 7 - x;
        }

        if y >= 4 {
            y = 7 - y;
        }

        return EARLY_GAME_SQUARE_ATTACK_FACTOR[(x + y * 4) as usize];
    }

    for i in 0..64 {        
        sum += attack_score[i].signum() * get_square_attack_factor(Square::from_u8(i as u8)); 
    }


    if do_print {
        print!("{}[", white_list.len());
        for m in &white_list {
            print!("{} ", m.get_board_name(&board));
        }
        println!("]");

        print!("{}[", black_list.len());
        for m in &black_list {
            
            print!("{} ", m.get_board_name(&board));
        }
        println!("]");
    }

    let mut mob_count = [0; 6];
    for m in &white_list {
        mob_count[PieceType::from_cpt(m.move_piece_type) as usize] += 1;
    }

    for m in &black_list {
        mob_count[PieceType::from_cpt(m.move_piece_type) as usize] -= 1;
    }

    if do_print {
        println!("Piece mobility: ");
    }
    for i in 0..6 {
        dif_sum += mob_count[i] * PIECE_MOBILITY_SCORE[i];

        if do_print {
            println!("\t{} -> {} * {} \t= {}", ColoredPieceType::from_u8(i as u8 * 2).get_char(), 
                mob_count[i], PIECE_MOBILITY_SCORE[i as usize], mob_count[i] * PIECE_MOBILITY_SCORE[i]);
        }
    }

    if do_print {
        println!("\tsum: {dif_sum}");
    }

    sum += dif_sum;
    dif_sum = 0;

    sum += eval_pawn_structure(game, do_print);

    if do_print {
        println!("Total: {}", sum);
    }

    if sum.abs() < 7 {
        sum = 7;
    }


    let ret = sum * if game.is_whites_turn() { 1 } else { -1 };

    return ret;
}

pub fn eval_pawn_structure(game: &Game, do_print: bool) -> i32 {

    let board = game.get_board();
    let mut sum = 0;

        //Pawn structure
    let white_pawns_bitboard = board.get_piece_bitboard(ColoredPieceType::WhitePawn);
    let black_pawns_bitboard = board.get_piece_bitboard(ColoredPieceType::BlackPawn);

    let white_passed_pawns = count_passed_pawns(white_pawns_bitboard, black_pawns_bitboard, bitboard_helper::WHITE_PASSED_PAWN_MASK);
    let black_passed_pawns = count_passed_pawns(black_pawns_bitboard, white_pawns_bitboard, bitboard_helper::BLACK_PASSED_PAWN_MASK);

    sum += (white_passed_pawns - black_passed_pawns) * PASSED_PAWN_VALUE;

    let white_doubled_pawns = count_doubled_pawns(white_pawns_bitboard);
    let black_doubled_pawns = count_doubled_pawns(black_pawns_bitboard);

    sum += (white_doubled_pawns - black_doubled_pawns) * DOUBLED_PAWN_PENALTY;

    let white_isolated_pawns = count_isolated_pawns(white_pawns_bitboard);
    let black_isolated_pawns = count_isolated_pawns(black_pawns_bitboard);

    sum += (white_isolated_pawns - black_isolated_pawns) * ISOLATED_PAWN_PENALTY;

    let mut pawn_ranks = [0; 8];
    //dont have to check last or first rank
    for i in 1..7 {
        pawn_ranks[i] += (white_pawns_bitboard & bitboard_helper::RANK_MASKS[i]).count_ones() as i32;
        pawn_ranks[7 - i] -= (black_pawns_bitboard & bitboard_helper::RANK_MASKS[i]).count_ones() as i32;
    }   

    for i in 1..7 {
        sum += pawn_ranks[i] * PAWN_PUSH_BONUS[i];
    }

    if do_print {
        println!("Pawn structure: ");
        println!("\tPassed pawns: ({} - {}) * {}", white_passed_pawns, black_passed_pawns, PASSED_PAWN_VALUE);
        println!("\tDoubled pawns: ({} - {}) * {}", white_doubled_pawns, black_doubled_pawns, DOUBLED_PAWN_PENALTY);
        println!("\tIsolated pawns: ({} - {}) * {}", white_isolated_pawns, black_isolated_pawns, ISOLATED_PAWN_PENALTY);

        println!("Pawn push bonus: ");
        for i in 1..7 {
            println!("\tPawn rank {} dif: {} * {}", i + 1, pawn_ranks[i], PAWN_PUSH_BONUS[i]);
        }

        println!("Pawn sum: {}", sum);
    }
    
    return sum;

    fn count_passed_pawns(allied_pawns: u64, opponent_pawns: u64, pawn_mask: [u64; 64]) -> i32 {
        let mut count = 0;

        for i in bitboard_helper::iterate_set_bits(allied_pawns) {
            if opponent_pawns & pawn_mask[i as usize] == 0 {
                count += 1;
            }
        }

        return count;
    }

    fn count_doubled_pawns(pawn_bitboard: u64) -> i32 {
        let mut buffer = pawn_bitboard;
        let mut count = 0;
        for i in 0..8 {
            buffer <<= 8;

            count += (pawn_bitboard & buffer).count_ones();
        }

        return count as i32;
    }

    fn count_isolated_pawns(pawn_bitboard: u64) -> i32 {
        let mut count = 0;

        for i in bitboard_helper::iterate_set_bits(pawn_bitboard) {
            if pawn_bitboard & bitboard_helper::NEIGHBOUR_FILES[(i % 8)  as usize] == 0 {
                count += 1;
            }
        }

        return count;
    }    
}