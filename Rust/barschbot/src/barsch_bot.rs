use std::{time::Instant, cmp, collections::HashMap};

use arrayvec::ArrayVec;
use rand::seq::SliceRandom;

use crate::{game::{Game, GameState}, chess_move::{ChessMove, self, NULL_MOVE}, piece_type::PieceType, bit_board::{BitBoard, self}, 
    evaluation::*, endgame_table::{self, EndgameTable, UNDEFINED, BoardState}, bb_settings::{self, BBSettings}, opening_book::OpeningBook};

const MAX_VALUE: f32 =  f32::INFINITY;

pub struct Stats {
    pub nodes: u64,
    pub qs: u64,
    pub best_move_hits: u64,
    pub not_best_move_hits: u64
}

impl Stats {
    pub fn new() -> Stats {
        return Stats { nodes: 0, qs: 0, best_move_hits: 0, not_best_move_hits: 0 };
    }
    pub fn print(&self) {
        println!("Nodes: {} Qs: {} BMFM ratio: {}", self.nodes, self.qs, self.best_move_hits as f32 / (self.not_best_move_hits + self.best_move_hits) as f32);
    }
}

pub fn get_best_move(game: &mut Game, table: &EndgameTable, bb_settings: &BBSettings, book: &OpeningBook) -> ChessMove{
    println!("Looking for best move");
    let om = book.get_move(game.get_board().get_zoberist_hash());

    if om != NULL_MOVE {
        println!("Book move");
        return om;
    }

    if bb_settings.end_game_table && game.get_board().get_all_piece_count() <= table.max_piece_count as u32 {
        //println!("Endgame move");
        return end_game_move(game, table);
    }

    if bb_settings.eval_factors.piece_value[0] == 0.0 {
        let list = game.get_legal_moves();

        return list[game.get_board().get_zoberist_hash() as usize % list.len()];
    }

    return iterative_deepening(game, table, bb_settings).0; 
}

pub fn end_game_move(game: &mut Game, table: &EndgameTable) -> ChessMove {
    if game.get_board().get_all_piece_count() > table.max_piece_count as u32 {
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
    if board.get_all_piece_count() <= table.max_piece_count as u32 {

        //println!("This should not happen {}", table.max_piece_count);
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
    const PRINT: bool = true;
    
    let mut map = HashMap::new();
    
    if PRINT {
        println!("Evaluating: {}", game.get_board().get_fen());
        static_eval(game, &bb_settings.eval_factors, true);
    }

    let mut start = Instant::now();
    let mut pair: (ChessMove, f32, GameState) = (NULL_MOVE, 0.0, GameState::Undecided);
    let mut stats = Stats::new();

    for md in 1..(bb_settings.max_depth + 1) {
        pair = alpha_beta_nega_max(game, -MAX_VALUE, MAX_VALUE,  md, table, &mut map, bb_settings, &mut stats);
        //pair = negation_max(game, i);

        let duration = start.elapsed();
        
        if PRINT {
            println!("{:?}", duration);
            print!("Depth: {} Eval: ", md);
            
            
            if pair.2 == GameState::Undecided {
                print!("{:.3}", pair.1);
            }
            else {
                print!("{:.3}", pair.2.to_string());
            }
            
            println!(" Move: {}", pair.0.get_uci());
        }

        if pair.2.is_checkmate() {
            break;
        }
    }

    if PRINT {
        stats.print();
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

pub fn better_move_sorter(list: &mut ArrayVec<ChessMove, 200>, board: &BitBoard, prev_best: ChessMove) {
    const PIECE_VALUES: [i32; 7] = [10, 28, 32, 50, 90, 100, 0];

    //board.print();            
    list.sort_by_cached_key(|cm| {
        if *cm == prev_best {
            return i32::MIN;
        }
        
        let mut sum = 0;

        if cm.is_direct_capture() {
            sum += PIECE_VALUES[PieceType::from_cpt(cm.capture_piece_type) as usize] 
            - PIECE_VALUES[PieceType::from_cpt(cm.move_piece_type) as usize] 
            + 200;
        }

        if cm.is_en_passant() {
            sum += 200;
        }

        sum *= 1000;

        sum += PIECE_VALUES[PieceType::from_cpt(cm.promotion_piece_type) as usize];

        sum *= 1000;

        sum += board.get_piece_captures_at(cm.move_piece_type, cm.target_square).iter()
            .map(|x| PIECE_VALUES[*x as usize]).sum::<i32>();

        //println!("Move: {} sum: {}", cm.get_board_name(&board), sum);

        return -sum;
    });

    //board.print_local_moves(&list);
}

pub fn alpha_beta_nega_max(game: &mut Game, mut alpha: f32, beta: f32, depth_left: u8, table: &EndgameTable, map: &mut HashMap<u64, (u8, ChessMove, f32, GameState)>, settings: &BBSettings, stats: &mut Stats) -> (ChessMove, f32, GameState) {        
    stats.nodes += 1;
    
    if depth_left == 0 {
        stats.qs += 1;
        return quiescence(game, alpha, beta, settings.max_quiescence_depth, table, map, settings);
    }

    if game.get_game_state() != GameState::Undecided {
        let pair = static_eval(game, &settings.eval_factors, false);
        return (chess_move::NULL_MOVE, pair.0, pair.1);
    }
    
    if settings.end_game_table {
        let pair = get_relative_endgame_eval(&game.get_board(), table);
        if pair.1 != GameState::Undecided {
            return (chess_move::NULL_MOVE, pair.0, pair.1);
        }
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

    //move_sorter(&mut list, hist_move);
    better_move_sorter(&mut list, &game.get_board(), hist_move);

    let fm = list[0];
    let mut sm = NULL_MOVE;

    if list.len() > 1 {
        sm = list[1];
    }

    for m in  list {
        
        game.make_move(m);
        
        let (line, mut value, gs) = alpha_beta_nega_max(game,  -beta, -alpha, depth_left - 1, table, map, settings, stats);
        
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

    if best_move == fm || best_move == sm {
        stats.best_move_hits += 1;
    }
    else {
        stats.not_best_move_hits += 1;
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
    
    if settings.end_game_table {
        let pair = get_relative_endgame_eval(&game.get_board(), table);
        if pair.1 != GameState::Undecided {
            return (chess_move::NULL_MOVE, pair.0, pair.1);
        }
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

pub fn is_quiet_pos(board: &mut BitBoard) -> bool {
    if board.in_check() {
        return false;
    }

    return true;

    let mut list = board.get_legal_moves();

    for m in  list {
        if (PieceType::from_cpt(m.move_piece_type) as u8) < (PieceType::from_cpt(m.capture_piece_type) as u8) 
            || m.is_promotion() {
            return false;
        }
    }    

    return true;
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