use std::time::Instant;

use crate::{game::{Game, GameState}, chess_move::{ChessMove, self}, board::Board, constants::{BLACK_PAWN, self}, piece_list::{self, PieceList}, bitboard_helper};

pub fn iterative_deepening(game: &mut Game, max_depth: u8) -> (ChessMove, i32) {
    let mut start = Instant::now();

    let mut pair: (ChessMove, i32) = (chess_move::NULL_MOVE, 0);
    for i in 1..(max_depth + 1) {
        pair = negation_max(game, i);
        
        let duration = start.elapsed();
        println!("{:?}", duration);
        println!("Depth: {} Move: {} Eval: {}", i, pair.0.get_uci(), pair.1);
    }

    return pair;
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

        //m.print();
        //println!(" -> {}", value);

        if value > best_value {
            best_value = value;
            best_move = m;
        }
        
        game.undo_move();
    }    

    return (best_move, best_value);
}
//                              Pawn, Knight, Bishop, Rook, Queen
const PIECE_VALUES: [i32; 5] = [1000, 2800, 3200, 5000, 9000];
const CHECK_MATE_VALUE: i32 = 1_000_000_000;
const PASSED_PAWN_VALUE: i32 = 70;
const DOUBLED_PAWN_VALUE: i32 = -50;
const PAWN_CENTER_ATTACK_VALUE: i32 = 20;

pub fn static_eval(game: &mut Game) -> i32 {
    
    match game.get_game_state() {
        GameState::Checkmate => return CHECK_MATE_VALUE,
        GameState::Draw => return 0,
        GameState::Undecided => ()
    }
    
    //whites perspective
    let board = game.get_board();
    let mut sum: i32 = 0;

    for i in 0..5 {
        sum += (board.piece_lists[i * 2 + 0].count() as i32 - board.piece_lists[i * 2 + 1].count() as i32) 
            * PIECE_VALUES[i];
    }
    
    sum += eval_pawn_structure(game);

    return  sum * if game.is_whites_turn() { 1 } else { -1 };
}


pub fn eval_pawn_structure(game: &Game) -> i32 {

    let board = game.get_board();
    let white_pawn_list = board.piece_lists[constants::WHITE_PAWN as usize];
    let black_pawn_list = board.piece_lists[constants::BLACK_PAWN as usize];

    let mut sum: i32 = 0;

    sum += (count_passed_pawns(white_pawn_list, game.black_pawns_bitboard, bitboard_helper::WHITE_PASSED_PAWN_MASK) - 
        count_passed_pawns(black_pawn_list, game.white_pawns_bitboard, bitboard_helper::BLACK_PASSED_PAWN_MASK)) * PASSED_PAWN_VALUE;


    sum += (count_doubled_pawns(game.white_pawns_bitboard) - 
        count_doubled_pawns(game.black_pawns_bitboard)) * DOUBLED_PAWN_VALUE;

    sum += (2 * (game.white_pawns_bitboard & bitboard_helper::DOUBLE_PAWN_CENTER_ATTACK_WHITE).count_ones() as i32 
        + (game.white_pawns_bitboard & bitboard_helper::PAWN_CENTER_ATTACK_WHITE).count_ones() as i32
        - 2 * (game.black_pawns_bitboard & bitboard_helper::DOUBLE_PAWN_CENTER_ATTACK_BLACK).count_ones() as i32  
        - (game.white_pawns_bitboard & bitboard_helper::PAWN_CENTER_ATTACK_BLACK).count_ones() as i32) * PAWN_CENTER_ATTACK_VALUE;
    
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

