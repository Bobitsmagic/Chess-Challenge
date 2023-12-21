use std::{time::Instant, cmp, collections::HashMap};

use arrayvec::ArrayVec;

use crate::{game::{Game, GameState}, chess_move::{ChessMove, self, NULL_MOVE}, piece_type::PieceType, bit_board::BitBoard, 
    evaluation::*, endgame_table::{self, EndgameTable, UNDEFINED}, bb_settings::{self, BBSettings}};

const MAX_VALUE: f32 =  f32::INFINITY;
const MAX_QUIESCENCE_DEPTH: u8 = 10;
const NULL_MOVE_REDUCTION: u8 = 1;
const DO_NM_PRUNING: bool = false;

pub fn get_best_move(game: &mut Game, table: &EndgameTable, bb_settings: &BBSettings) -> ChessMove{
    if game.get_board().get_all_piece_count() <= 4 {
        //println!("Endgame move");
        return end_game_move(game, table);
    }

    return iterative_deepening(game, table, bb_settings).0; 
}
//r3k2r/1pp1p1bp/p1nqb1p1/5p2/3P4/P1PBQN2/1P1B1PPP/R3K2R b KQkq -

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

pub fn get_relative_endgame_eval(board: &BitBoard, table: &EndgameTable) -> (f32, GameState) {
    if board.get_all_piece_count() <= 4 {
        let score = table.get_score(&board);
        
        let mut res = 0.0;
        let mut gs = GameState::Undecided;
        if score == 0 {
            res = 0.0;
            gs = GameState::InsuffMaterial;
        }
        else {
            gs = if score > 0 { GameState::BlackCheckmate } else { GameState::WhiteCheckmate };

            if board.is_whites_turn() {
                res = if score > 0 { CHECKMATE_VALUE } else { -CHECKMATE_VALUE };
            }
            else {
                res = if score < 0 { CHECKMATE_VALUE } else { -CHECKMATE_VALUE };
            }

            for i in 0..(127 - score.abs()) {
                res *= 0.999;
            }
        }

        return (res, gs);
    }

    return (0.0, GameState::Undecided);
}

pub fn iterative_deepening(game: &mut Game, table: &EndgameTable, bb_settings: &BBSettings) -> (ChessMove, f32) {
    const PRINT: bool = false;
    
    let mut map = HashMap::new();
    
    if PRINT {
        println!("Evaluating: {}", game.get_board().get_fen());
        //static_eval(game, &bb_settings.eval_factors, true);
    }

    let mut start = Instant::now();
    let mut pair: (ChessMove, f32, GameState) = (NULL_MOVE, 0.0, GameState::Undecided);
    for md in 1..(bb_settings.max_depth + 1) {
        pair = alpha_beta_nega_max(game, -MAX_VALUE, MAX_VALUE,  md, table, &mut map, bb_settings);
        //pair = negation_max(game, i);

        let duration = start.elapsed();
        
        if PRINT {
            println!("{:?}", duration);
            print!("Depth: {} Eval: ", md);
            
            print!("{}", pair.2.to_string());
    
            println!(" Move: {}", pair.0.get_uci());
        }

        if pair.2.is_checkmate() {
            break;
        }
    }

    return (pair.0, pair.1);
}

fn move_sorter(list: &mut ArrayVec<ChessMove, 200>, prev_best: ChessMove) {
    const PIECE_VALUES: [i32; 6] = [100, 280, 320, 500, 900, 100000];

    list.sort_unstable_by(|a, b| {
        if *a == prev_best {
            return std::cmp::Ordering::Less;
        }

        if *b == prev_best {
            return std::cmp::Ordering::Greater;
        }

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

pub fn alpha_beta_nega_max(game: &mut Game, mut alpha: f32, beta: f32, depth_left: u8, table: &EndgameTable, map: &mut HashMap<u64, (u8, ChessMove, f32, GameState)>, settings: &BBSettings) -> (ChessMove, f32, GameState) {        
    if depth_left == 0 {
        return quiescence(game, alpha, beta, settings.max_quiescence_depth, table, map, settings);
    }

    if game.get_game_state() != GameState::Undecided {
        let pair = static_eval(game, &settings.eval_factors, false);
        return (chess_move::NULL_MOVE, pair.0, pair.1);
    }
    
    let pair = get_relative_endgame_eval(&game.get_board(), table);
    if pair.1 != GameState::Undecided {
        return (chess_move::NULL_MOVE, pair.0, pair.1);
    }

    let hash = game.get_board().get_zoberist_hash();
    let mut hist_move = chess_move::NULL_MOVE;
    if map.contains_key(&hash) {
        let (hist_depth_left, m , hist_value, hist_gs) = map[&hash];

        hist_move = m;
        if hist_depth_left >= depth_left {
            return (hist_move, hist_value, hist_gs);
        }
    }

    let mut best_move = NULL_MOVE;
    let mut best_gs = GameState::Undecided;

    let mut list = game.get_legal_moves();

    move_sorter(&mut list, hist_move);

    for m in  list {
        game.make_move(m);
        
        let (line, mut value, gs) = alpha_beta_nega_max(game,  -beta, -alpha, depth_left - 1, table, map, settings);
        
        game.undo_move();
        
        value = -value;
        
        if value >= beta {
            //println!("Beta cutoff");
            return (NULL_MOVE, beta, GameState::Undecided);
        }
        
        //println!("Trying move: {} val {}, alpha {}", m.get_uci(), value, alpha);
        if value > alpha {
            alpha = value;

            best_move = m;
            best_gs = gs;
        }
    }    

    if !best_move.is_null_move() {
        if map.contains_key(&hash) {
            *map.get_mut(&hash).unwrap() = (depth_left, best_move, alpha, best_gs);
        }
        else {
            map.insert(hash, (depth_left, best_move, alpha, best_gs));
        }
    }
    
    //println!("Returning: {}", best_move.get_uci());
    return (best_move, alpha, best_gs);
}

pub fn quiescence(game: &mut Game, mut alpha: f32, beta: f32, depth_left: u8, table: &EndgameTable, map: &HashMap<u64, (u8, ChessMove, f32, GameState)>, settings: &BBSettings) -> (ChessMove, f32, GameState) {
    if game.get_game_state() != GameState::Undecided {
        let pair = static_eval(game, &settings.eval_factors, false);
        return (chess_move::NULL_MOVE, pair.0, pair.1);
    }
    
    let pair = get_relative_endgame_eval(&game.get_board(), table);
    if pair.1 != GameState::Undecided {
        return (chess_move::NULL_MOVE, pair.0, pair.1);
    }
    
    if game.get_board().in_check() {
        return check_avoid_search(game, alpha, beta, depth_left, table, map, settings);
    }
    
    let (stand_pat, sp_gs) = static_eval(game, &settings.eval_factors, false);

    if stand_pat >= beta {
        return (NULL_MOVE, beta, GameState::Undecided);
    }

    //only for quiescence search
    if stand_pat > alpha {
        alpha = stand_pat;
    }


    if depth_left <= 0 {
        //println!("Could not finish quiescence search");
        return (NULL_MOVE, stand_pat, sp_gs);
    }
    
    let hash = game.get_board().get_zoberist_hash();
    let mut hist_move = chess_move::NULL_MOVE;
    if map.contains_key(&hash) {
        let (hist_depth_left, m , hist_value, hist_gs) = map[&hash];

        hist_move = m;
        if hist_depth_left >= depth_left {
            return (hist_move, hist_value, hist_gs);
        }
    }

    let mut best_move = NULL_MOVE;
    let mut best_gs = GameState::Undecided;

    let mut list = game.get_legal_moves();

    move_sorter(&mut list, hist_move);

    for m in  list {
        if !(m.is_direct_capture() || m.is_en_passant() || m.is_promotion()) {
            continue;
        }

        game.make_move(m);

        let (line, mut value, gs) = quiescence(game,  -beta, -alpha, depth_left - 1, table, map, settings);

        game.undo_move();

        value = -value;

        if value >= beta {
            return (NULL_MOVE, beta, GameState::Undecided);
        }

        if value > alpha {
            alpha = value;

            best_move = m;
            best_gs = gs;
        }
    }    

    return (best_move, alpha, best_gs);
}

pub fn check_avoid_search(game: &mut Game, mut alpha: f32, beta: f32, depth_left: u8, table: &EndgameTable, map: &HashMap<u64, (u8, ChessMove, f32, GameState)>, settings: &BBSettings) -> (ChessMove, f32, GameState) {
    let mut best_move = NULL_MOVE;
    let mut best_gs = GameState::Undecided;

    let mut list = game.get_legal_moves();

    move_sorter(&mut list, chess_move::NULL_MOVE);
    //println!("Check seach {} depth {}", game.get_board().get_fen(), depth_left);
    for m in  list {
        game.make_move(m);

        let (line, mut value, gs) = quiescence(game,  -beta, -alpha, depth_left - if depth_left > 0 { 1 } else { 0 } , table, map, settings);

        game.undo_move();

        value = -value;

        if value >= beta {
            return (NULL_MOVE, beta, GameState::Undecided);
        }

        if value > alpha {
            alpha = value;

            best_move = line;
            best_gs = gs;
        }
    }    

    return (best_move, alpha, best_gs);
}