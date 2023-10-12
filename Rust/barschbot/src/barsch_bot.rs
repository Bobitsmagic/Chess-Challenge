use std::{time::Instant, cmp};

use arrayvec::ArrayVec;

use crate::{game::{Game, GameState}, chess_move::{ChessMove, self}, board::Board, constants::{BLACK_PAWN, self}, piece_list::{self, PieceList}, bitboard_helper};

const MAX_VALUE: i32 = 2_000_000_000;
const MAX_DEPTH: u8 = 5;
const MAX_QUIESCENCE_DEPTH: u8 = 10;

pub fn get_best_move(game: &mut Game) -> ChessMove{
    return iterative_deepening(game, MAX_DEPTH).0; 
}

pub fn iterative_deepening(game: &mut Game, max_depth: u8) -> (ChessMove, i32) {
    let mut start = Instant::now();

    let mut pair: (ArrayVec<ChessMove, 30>, i32) = (ArrayVec::new(), 0);
    for md in 1..(max_depth + 1) {
        pair = alpha_beta_nega_max(game, -MAX_VALUE, MAX_VALUE,  md);
        //pair = negation_max(game, i);

        let duration = start.elapsed();
        println!("{:?}", duration);
        print!("Depth: {} Eval: {} Line: ", md, pair.1);

        let mut list = pair.0.clone();
        list.reverse();
        
        for i in 0..(cmp::min(list.len(), md as usize)) {
            list[i].print();
            print!(" ");
        }
        print!(" | ");
        for i in (cmp::min(list.len(), md as usize))..list.len() {
            list[i].print();
            print!(" ");
        }
        println!();
    }

    return (pair.0[0], pair.1);
}

pub fn negation_max(game: &mut Game, depth_left: u8) -> (ChessMove, i32) {
    if depth_left == 0 {
        return (chess_move::NULL_MOVE, 
            static_eval(game));
    }
    
    let mut best_value = i32::MIN;
    let mut best_move = chess_move::NULL_MOVE;

    for m in game.get_legal_moves() {
        game.make_move(m);

        let value = -negation_max(game,  depth_left - 1).1;
        if value > best_value {
            best_value = value;
            best_move = m;
        }
        
        game.undo_move();
    }    

    return (best_move, best_value);
}

fn move_sorter(list: &mut ArrayVec<ChessMove, 200>) {
    list.sort_unstable_by(|a, b| {
        if a.is_direct_capture() != b.is_direct_capture() {
            return b.is_direct_capture().cmp(&a.is_direct_capture());
        }

        if a.is_direct_capture() && b.is_direct_capture() {
            return (PIECE_VALUES[(b.capture_piece_type >> 1) as usize] - PIECE_VALUES[(b.move_piece_type >> 1) as usize]).cmp(
                &(PIECE_VALUES[(a.capture_piece_type >> 1) as usize] - PIECE_VALUES[(a.move_piece_type >> 1) as usize]));
        }

        if a.is_promotion() != b.is_promotion() {
            return b.is_promotion().cmp(&a.is_promotion());
        }

        if a.is_promotion() && b.is_promotion() {
            return b.promotion_piece_type.cmp(&a.promotion_piece_type);
        }

        return std::cmp::Ordering::Equal;
    });
}

pub fn alpha_beta_nega_max(game: &mut Game, mut alpha: i32, beta: i32, depth_left: u8) -> (ArrayVec<ChessMove, 30>, i32) {
    if depth_left == 0 {
        //return (chess_move::NULL_MOVE, static_eval(game));
        return quiescence(game, alpha, beta, MAX_QUIESCENCE_DEPTH);
    }
    
    let mut best_line = ArrayVec::new();

    let mut list = game.get_legal_moves();

    move_sorter(&mut list);

    for m in  list {
        
        game.make_move(m);

        let (line, mut value) = alpha_beta_nega_max(game,  -beta, -alpha, depth_left - 1);

        game.undo_move();

        value = -value;

        if value >= beta {
            return (ArrayVec::new(), beta);
        }

        if value > alpha {
            alpha = value;

            best_line = line;
            best_line.push(m);
        }
    }    

    return (best_line, alpha);
}


pub fn quiescence(game: &mut Game, mut alpha: i32, beta: i32, depth_left: u8) -> (ArrayVec<ChessMove, 30>, i32) {
    let stand_pat = static_eval(game);
    
    if stand_pat >= beta {
        return (ArrayVec::new(), beta);
    }

    //only for quiescence search
    if stand_pat > alpha {
        alpha = stand_pat;
    }


    if depth_left == 0 {
        //println!("Could not finish quiescence search");
        return (ArrayVec::new(), stand_pat);
    }
    
    let mut best_line = ArrayVec::new();

    let mut list = game.get_legal_moves();

    move_sorter(&mut list);

    for m in  list {
        
        if !m.is_direct_capture() {
            continue;
        }

        game.make_move(m);

        let (line, mut value) = quiescence(game,  -beta, -alpha, depth_left - 1);
        game.undo_move();
        
        value = -value;
        
        if value >= beta {
            return (ArrayVec::new(), beta);
        }

        if value > alpha {
            alpha = value;
            best_line = line;

            best_line.push(m);
        }
    }    

    return (best_line, alpha);
}
//                              Pawn, Knight, Bishop, Rook, Queen
const PIECE_VALUES: [i32; 6] = [1000, 2800, 3200, 5000, 9000, MAX_VALUE];
const MAX_PIECE_MOBILITY: [i32; 6] = [4, 8, 13, 14, 27, 8];
const PIECE_MOBILITY_SCORE: [i32; 6] = [0, 100, 100, 50, 20, 200];

const PAWN_PUSH_BONUS: [i32; 8] = [0, 0, 30, 50, 80, 150, 500, 0];
const CHECK_MATE_VALUE: i32 = 1_000_000_000;
const PASSED_PAWN_VALUE: i32 = 70;
const DOUBLED_PAWN_PENALTY: i32 = -50;
const PAWN_CENTER_ATTACK_VALUE: i32 = 30;
const PIECE_CENTER_ATTACK_VALUE: i32 = 20;

const KING_ATTACK_PENALTY: i32 = -69;
const KING_DEFENCE_VALUE: i32 = 40;


pub fn static_eval(game: &mut Game) -> i32 {
    
    match game.get_game_state() {
        GameState::Checkmate => return CHECK_MATE_VALUE,
        GameState::Draw => return 0,
        GameState::Undecided => ()
    }
    
    //whites perspective
    let board = game.get_board();
    let pl = board.piece_lists;
    let mut sum: i32 = 0;

    for i in 0..5 {
        sum += (pl[i * 2 + 0].count() as i32 - pl[i * 2 + 1].count() as i32) 
            * PIECE_VALUES[i];
    }

    //Center attack
    let mut white_center_attacks = 0;
    let mut black_center_attacks = 0;
    for i in 0..4 {
        white_center_attacks += board.get_square_attack_count(true, i + constants::C4);
        white_center_attacks += board.get_square_attack_count(true, i + constants::C5);

        black_center_attacks += board.get_square_attack_count(true, i + constants::C4);
        black_center_attacks += board.get_square_attack_count(true, i + constants::C5);
    }

    sum += (white_center_attacks as i32 - black_center_attacks as i32) * PIECE_CENTER_ATTACK_VALUE;

    //King safety
    let mut attacks_on_white_king = 0;
    let mut defence_on_white_king = 0;
    let mut attacks_on_black_king = 0;
    let mut defence_on_black_king = 0;
    for i in 0..4 {
        for sq in constants::KING_MOVES[board.get_king_square(true) as usize] {
            defence_on_white_king += board.get_square_attack_count(true, *sq);
            attacks_on_white_king += board.get_square_attack_count(false, *sq);
        }
        
        for sq in constants::KING_MOVES[board.get_king_square(false) as usize] {
            defence_on_black_king += board.get_square_attack_count(false, *sq);
            attacks_on_black_king += board.get_square_attack_count(true, *sq);
        }
    }

    sum += (attacks_on_white_king as i32 - attacks_on_black_king as i32) * KING_ATTACK_PENALTY;
    //sum += (defence_on_white_king as i32 - defence_on_black_king as i32) * KING_DEFENCE_VALUE;

    //Mobility
    for piece_type in 1..5 {
        let mut list = pl[piece_type * 2 + 0];
        for index in 0..list.count(){
            sum += (board.get_piece_attack_count(list.get_occupied_square(index)) as i32) * PIECE_MOBILITY_SCORE[piece_type] / MAX_PIECE_MOBILITY[piece_type];
        }

        list = pl[piece_type * 2 + 1];
        for index in 0..list.count(){
            sum -= (board.get_piece_attack_count(list.get_occupied_square(index)) as i32) * PIECE_MOBILITY_SCORE[piece_type] / MAX_PIECE_MOBILITY[piece_type];
        }
    }
    
    sum += eval_pawn_structure(game);

    return  sum * if game.is_whites_turn() { 1 } else { -1 };
}


pub fn eval_pawn_structure(game: &Game) -> i32 {

    let board = game.get_board();
    let white_pawn_list = board.piece_lists[constants::WHITE_PAWN as usize];
    let black_pawn_list = board.piece_lists[constants::BLACK_PAWN as usize];

    let white_pawns_bitboard = game.white_pawns_bitboard();
    let black_pawns_bitboard = game.black_pawns_bitboard();
    
    let mut sum: i32 = 0;

    let white_passed_pawns = count_passed_pawns(white_pawn_list, black_pawns_bitboard, bitboard_helper::WHITE_PASSED_PAWN_MASK);
    let black_passed_pawns = count_passed_pawns(black_pawn_list, white_pawns_bitboard, bitboard_helper::BLACK_PASSED_PAWN_MASK);

    sum += (white_passed_pawns - black_passed_pawns) * PASSED_PAWN_VALUE;

    let white_doubled_pawns = count_doubled_pawns(white_pawns_bitboard);
    let black_doubled_pawns = count_doubled_pawns(black_pawns_bitboard);

    sum += (white_doubled_pawns - black_doubled_pawns) * DOUBLED_PAWN_PENALTY;

    let white_dca = (white_pawns_bitboard & bitboard_helper::DOUBLE_PAWN_CENTER_ATTACK_WHITE).count_ones() as i32;
    let white_ca = (white_pawns_bitboard & bitboard_helper::PAWN_CENTER_ATTACK_WHITE).count_ones() as i32;

    let black_dca = (black_pawns_bitboard & bitboard_helper::DOUBLE_PAWN_CENTER_ATTACK_BLACK).count_ones() as i32;
    let black_ca = (black_pawns_bitboard & bitboard_helper::PAWN_CENTER_ATTACK_BLACK).count_ones() as i32;

    //sum += (2 * white_dca + white_ca - 2 * black_dca - black_ca) * PAWN_CENTER_ATTACK_VALUE;
    
    
    //println!("White pawns: ");
    //bitboard_helper::print_bitboard(white_pawns_bitboard);
    //println!("Black pawns: ");
    //bitboard_helper::print_bitboard(black_pawns_bitboard);
    //println!(" Passed pawns: {white_passed_pawns} - {black_passed_pawns}\n Doubled Pawns: {white_doubled_pawns} - {black_doubled_pawns}\n Center attack: 2 * {white_dca} + {white_ca} - 2 * {black_dca} - {black_ca}\n Eval: {sum}");
    
    //dont have to check last or first rank
    for i in 1..7 {
        //println!("Rank {}: {} - {}", i + 1, (white_pawns_bitboard & bitboard_helper::RANK_MASKS[i]).count_ones(), (black_pawns_bitboard & bitboard_helper::RANK_MASKS[i]).count_ones());

        sum += PAWN_PUSH_BONUS[i] * (white_pawns_bitboard & bitboard_helper::RANK_MASKS[i]).count_ones() as i32;
        sum -= PAWN_PUSH_BONUS[7 - i] * (black_pawns_bitboard & bitboard_helper::RANK_MASKS[i]).count_ones() as i32;

        //println!("{}", sum);
    }

    return sum;

    fn count_passed_pawns(piece_list: PieceList, opponent_pawns: u64, pawn_mask: [u64; 64]) -> i32 {
        let mut count = 0;

        for i in 0..piece_list.count() {
            let start_square = piece_list.get_occupied_square(i);

            if opponent_pawns & pawn_mask[start_square as usize] == 0 {
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

            count += (pawn_bitboard & buffer).count_ones() as i32;
        }

        return count;
    }
}

