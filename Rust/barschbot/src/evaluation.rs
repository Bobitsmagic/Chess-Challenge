use core::panic;
use std::collections::HashMap;

use crate::{game::{Game, GameState}, colored_piece_type::ColoredPieceType, piece_type::PieceType, bitboard_helper, endgame_table::EndgameTable, square::{self, Square}, zoberist_hash, bb_settings::EvalFactors, bit_board::BitBoard};

pub const CHECKMATE_VALUE: f32 = f32::MAX;
//                              Pawn, Knight, Bishop, Rook, Queen, King

pub struct EvalAttributes {
    pub piece_dif: [i32; 5], 
    pub safe_mobility_dif: [i32; 6], 
    pub unsafe_mobility_dif: [i32; 6], 
    
    pub square_control_dif: i32, 

    pub pawn_push_dif: [i32; 6], 
    pub passed_pawn_dif: i32, 
    pub doubled_pawn_dif: i32, 
    pub isolated_pawn_dif: i32, 

    pub king_attack_dif: i32
}

impl EvalAttributes {
    pub fn print(&self) {

        println!("Piece dif: ");
        for i in 0..5 {
            if self.piece_dif[i] != 0 {
                let pt = PieceType::from_u8(i as u8);
                
                println!("\t{} {}", PieceType::from_u8(i as u8).get_char(), self.piece_dif[i]);
            }
        }

        println!("Mobility dif:");
        for i in 0..6 {
            if self.safe_mobility_dif[i] != 0 {
                println!("\tsafe: {} -> {}", PieceType::from_u8(i as u8).get_char(), self.safe_mobility_dif[i]);
            }

            if self.unsafe_mobility_dif[i] != 0 {
                println!("\tunsafe: {} -> {}", PieceType::from_u8(i as u8).get_char(), self.unsafe_mobility_dif[i]);
            }
        }
        
        println!("Square control dif: {}", self.square_control_dif);

        println!("Pawn push dif: ");
        for i in 0..6 {
            if self.pawn_push_dif[i] != 0 {
                println!("\tRank {} -> {}", i, self.pawn_push_dif[i]);
            }
        }

        if self.passed_pawn_dif != 0 {
            println!("Passed pawn dif: {}", self.passed_pawn_dif);
        }

        if self.doubled_pawn_dif != 0 {
            println!("Doubled pawn dif: {}", self.doubled_pawn_dif);
        }

        if self.isolated_pawn_dif != 0 {
            println!("Isolated pawn dif: {}", self.isolated_pawn_dif);
        }

        if self.king_attack_dif != 0 {
            println!("King attack dif: {}", self.king_attack_dif);
        }
    }
}

pub fn static_eval(game: &mut Game, factors: &EvalFactors, do_print: bool) -> (f32, GameState) {
    let gs = game.get_game_state();

    //whites perspective
    let board = game.get_board();

    if do_print {
        println!("Evaluating: ");
        board.print();
        println!("Gs: {}", gs.to_string());
    }

    if gs.is_checkmate() {
        return (-CHECKMATE_VALUE, gs);
    }

    if gs.is_draw() {
        return (0.0, gs);
    }


    if board.in_check() {
        board.print();
        panic!("In check");
    } 

    let attributes = generate_eval_attributes(&board);

    if do_print {
        attributes.print();
    }

    let sum = factors.evaluate(&attributes);
     
    return (sum, gs);
}

pub fn generate_eval_attributes(board: &BitBoard) -> EvalAttributes {
    let mut piece_count = [0; 5];

    for i in 0..5 {
        piece_count[i as usize] = board.get_piece_count(ColoredPieceType::from_u8(i * 2 + 0)) as i32 
            - board.get_piece_count(ColoredPieceType::from_u8(i * 2 + 1)) as i32;
    }

    let white_list = board.generate_legal_moves_eval(true);
    let black_list = board.generate_legal_moves_eval(false);

    let mut least_valueable_attacker_white = [PieceType::None; 64];
    let mut least_valueable_attacker_black = [PieceType::None; 64];

    let mut static_exchange_evaluation = [0; 64];
        
    //Compute controlled squares (SEE) and LVA
    const PIECE_ATTACK_SCORE: [i32; 6] = [10000, 1000, 1000, 100, 10, 1];
    for m in &white_list {
        if m.is_attack() {
            least_valueable_attacker_white[m.target_square as usize] 
                = least_valueable_attacker_white[m.target_square as usize].min(PieceType::from_cpt(m.move_piece_type));

            static_exchange_evaluation[m.target_square as usize] += PIECE_ATTACK_SCORE[PieceType::from_cpt(m.move_piece_type) as usize];
        }
    }
    for m in &black_list {
        if m.is_attack() {
            least_valueable_attacker_black[m.target_square as usize] 
                = least_valueable_attacker_black[m.target_square as usize].min(PieceType::from_cpt(m.move_piece_type));

            static_exchange_evaluation[m.target_square as usize] -= PIECE_ATTACK_SCORE[PieceType::from_cpt(m.move_piece_type) as usize];
        }
    }

    let mut safe_mobility_count = [0; 6];
    let mut unsafe_mobility_count = [0; 6];
    //Compute safe mobility
    for m in &white_list {
        let mpt = PieceType::from_cpt(m.move_piece_type);
        if mpt <= PieceType::from_cpt(board.get_piece_type(m.target_square)) ||
                mpt <= least_valueable_attacker_black[m.target_square as usize] {
            safe_mobility_count[mpt as usize] += 1;
        }
        else {
            unsafe_mobility_count[mpt as usize] += 1;
        }
    }

    for m in &black_list {
        let mpt = PieceType::from_cpt(m.move_piece_type);
        if mpt <= PieceType::from_cpt(board.get_piece_type(m.target_square)) ||
                mpt <= least_valueable_attacker_white[m.target_square as usize] {
            safe_mobility_count[mpt as usize] -= 1;
        }
        else {
            unsafe_mobility_count[mpt as usize] -= 1;
        }
    }

    let mut controlled_squares = 0;
    for s in 0..64 {
        controlled_squares += static_exchange_evaluation[s].signum()
    }
    
    let (white_passed_pawns, white_doubled_pawns, white_isolated_pawns, white_pawn_ranks) 
        = eval_pawn_structure(board);
    
    
    return EvalAttributes {
        piece_dif: piece_count,
        safe_mobility_dif: safe_mobility_count,
        unsafe_mobility_dif: unsafe_mobility_count,
        square_control_dif: controlled_squares,
        pawn_push_dif: white_pawn_ranks,
        passed_pawn_dif: white_passed_pawns,
        doubled_pawn_dif: white_doubled_pawns,
        isolated_pawn_dif: white_isolated_pawns,
        king_attack_dif: 0
    };
}

pub fn eval_pawn_structure(board: &BitBoard) -> (i32, i32, i32, [i32; 6]) {
    let mut sum = 0;

        //Pawn structure
    let white_pawns_bitboard = board.get_piece_bitboard(ColoredPieceType::WhitePawn);
    let black_pawns_bitboard = board.get_piece_bitboard(ColoredPieceType::BlackPawn);

    let white_passed_pawns = count_passed_pawns(white_pawns_bitboard, black_pawns_bitboard, bitboard_helper::WHITE_PASSED_PAWN_MASK);
    let black_passed_pawns = count_passed_pawns(black_pawns_bitboard, white_pawns_bitboard, bitboard_helper::BLACK_PASSED_PAWN_MASK);

    let white_doubled_pawns = count_doubled_pawns(white_pawns_bitboard);
    let black_doubled_pawns = count_doubled_pawns(black_pawns_bitboard);

    let white_isolated_pawns = count_isolated_pawns(white_pawns_bitboard);
    let black_isolated_pawns = count_isolated_pawns(black_pawns_bitboard);

    let mut pawn_ranks = [0; 6];
    //dont have to check last or first rank
    for i in 1..7 {
        pawn_ranks[i - 1] += (white_pawns_bitboard & bitboard_helper::RANK_MASKS[i]).count_ones() as i32;
        pawn_ranks[7 - i - 1] -= (black_pawns_bitboard & bitboard_helper::RANK_MASKS[i]).count_ones() as i32;
    }   

    return (white_passed_pawns - black_passed_pawns, 
        white_doubled_pawns - black_doubled_pawns, 
        white_isolated_pawns - black_isolated_pawns, 
        pawn_ranks);

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