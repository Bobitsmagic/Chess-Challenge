use std::{time::Instant, cmp, array};

use arrayvec::ArrayVec;

use crate::{game::{Game, GameState}, chess_move::{ChessMove, self}, constants::{BLACK_PAWN, self}, piece_list::{self, PieceList}, bitboard_helper, piece_type::PieceType, colored_piece_type::ColoredPieceType, square::Square, bit_board::BitBoard, perceptron::Perceptron, 
    evaluation::*, endgame_table::{self, EndgameTable, UNDEFINED}};

const MAX_VALUE: i32 = 2_000_000_000;
const MAX_DEPTH: u8 = 8;
const MAX_QUIESCENCE_DEPTH: u8 = 5;

pub fn get_best_move(game: &mut Game, table: &EndgameTable) -> ChessMove{
    if game.get_board().get_all_piece_count() <= 4 {
        println!("Endgame move");
        return end_game_move(game, table);
    }

    return iterative_deepening(game, MAX_DEPTH, table).0; 
}

pub fn end_game_move(game: &mut Game, table: &EndgameTable) -> ChessMove {
    if game.get_board().get_all_piece_count() > 4 {
        panic!("To many pieces for endgame table");
    }

    let mut best_score =  endgame_table::UNDEFINED;

    let mut best_move = chess_move::NULL_MOVE;

    for m in game.get_legal_moves() {
        game.make_move(m);

        let s = table.get_score(&game.get_board());

        game.undo_move();

        //m.print();
        //println!(" -> {s}");

        if best_score == UNDEFINED {
            best_score = s;
            best_move = m;
        }

        if game.is_whites_turn() {
            if s > best_score {
                best_score = s;
                best_move = m;
            }
        }
        else {
            if s < best_score {
                best_score = s;
                best_move = m;
            }
        }

    }

    return best_move;
}

pub fn get_relative_endgame_eval(board: &BitBoard, table: &EndgameTable) -> (bool, i32) {
    if board.get_all_piece_count() <= 4 {
        let score = table.get_score(&board);
        
        let mut res = 0;
        if score == 0 {
            res = 0;
        }
        else {
            if board.is_whites_turn() {
                res = if score > 0 { CHECKMATE_VALUE } else { -CHECKMATE_VALUE };
            }
            else {
                res = if score < 0 { CHECKMATE_VALUE } else { -CHECKMATE_VALUE };
            }

            res += (score.signum() * (127 - score.abs())) as i32;
        }

        return (true, res);
    }

    return (false, 0);
}

pub fn iterative_deepening(game: &mut Game, max_depth: u8, table: &EndgameTable) -> (ChessMove, i32) {
    println!("Evaluating: {}", game.get_board().get_fen());

    static_eval(game, true);

    let mut start = Instant::now();
    let mut pair: (ArrayVec<ChessMove, 30>, i32) = (ArrayVec::new(), 0);
    for md in 1..(max_depth + 1) {
        pair = alpha_beta_nega_max(game, -MAX_VALUE, MAX_VALUE,  md, table);
        //pair = negation_max(game, i);

        let duration = start.elapsed();
        println!("{:?}", duration);

        print!("Depth: {} Eval: ", md);

        match pair.1.abs() {
            0 => print!("Stalemate"),     
            2 => print!("Repetition"),     
            3 => print!("InsuffMaterial"),     
            5 => print!("50MoveRule"),
            7 => print!("Even"),
            _ => print!("{}", pair.1)     
        }

        print!(" Line: ");

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

        if pair.1.abs() > CHECKMATE_VALUE - 1000 {
            break;
        }
    }

    return (*pair.0.last().unwrap(), pair.1);
}

pub fn negation_max(game: &mut Game, depth_left: u8) -> (ChessMove, i32) {
    if depth_left == 0 {
        return (chess_move::NULL_MOVE, 
            static_eval(game, false));
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
            return (PIECE_VALUES[PieceType::from_cpt(b.capture_piece_type) as usize] 
                    - PIECE_VALUES[PieceType::from_cpt(b.move_piece_type) as usize])
                    .cmp(&(PIECE_VALUES[PieceType::from_cpt(a.move_piece_type) as usize] 
                    - PIECE_VALUES[PieceType::from_cpt(a.move_piece_type) as usize]));
        }

        if a.is_promotion() != b.is_promotion() {
            return b.is_promotion().cmp(&a.is_promotion());
        }

        if a.is_promotion() && b.is_promotion() {
            return  PIECE_VALUES[PieceType::from_cpt(b.promotion_piece_type) as usize]
                .cmp(&PIECE_VALUES[PieceType::from_cpt(a.promotion_piece_type) as usize]);
        }

        return std::cmp::Ordering::Equal;
    });
}

pub fn alpha_beta_nega_max(game: &mut Game, mut alpha: i32, beta: i32, depth_left: u8, table: &EndgameTable) -> (ArrayVec<ChessMove, 30>, i32) {        


    if depth_left == 0 {
        //return (chess_move::NULL_MOVE, static_eval(game));
        return quiescence(game, alpha, beta, MAX_QUIESCENCE_DEPTH, table);
    }

    if game.get_game_state() != GameState::Undecided {
        return (ArrayVec::new(), static_eval(game, false));
    }
    
    let pair = get_relative_endgame_eval(&game.get_board(), table);
    if pair.0 {
        return (ArrayVec::new(), pair.1);
    }

    let mut best_line = ArrayVec::new();

    let mut list = game.get_legal_moves();

    move_sorter(&mut list);

    for m in  list {
        
        game.make_move(m);

        let (line, mut value) = alpha_beta_nega_max(game,  -beta, -alpha, depth_left - 1, table);

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

pub fn quiescence(game: &mut Game, mut alpha: i32, beta: i32, depth_left: u8, table: &EndgameTable) -> (ArrayVec<ChessMove, 30>, i32) {
    
    let board = game.get_board();

    let pair = get_relative_endgame_eval(&game.get_board(), table);
    if pair.0 {
        return (ArrayVec::new(), pair.1);
    }
    
    let stand_pat = static_eval(game, false);
    
    if stand_pat.abs() < 7 {
        return (ArrayVec::new(), stand_pat);
    }

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

    //[TODO] quiescence search move gen
    for m in  list {
        
        if !(m.is_direct_capture() || m.is_en_passant() || m.is_promotion()) {
            continue;
        }

        game.make_move(m);
        let (line, mut value) = quiescence(game,  -beta, -alpha, depth_left - 1, table);
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