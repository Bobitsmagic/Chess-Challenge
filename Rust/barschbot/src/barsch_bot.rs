use std::{time::Instant, cmp, collections::HashMap};

use arrayvec::ArrayVec;

use crate::{game::{Game, GameState}, chess_move::{ChessMove, self}, piece_type::PieceType, bit_board::BitBoard, 
    evaluation::*, endgame_table::{self, EndgameTable, UNDEFINED}};

const MAX_VALUE: i32 = 2_000_000_000;
const MAX_DEPTH: u8 = 4;
const MAX_QUIESCENCE_DEPTH: u8 = 10;
const NULL_MOVE_REDUCTION: u8 = 1;
const DO_NM_PRUNING: bool = false;

pub fn get_best_move(game: &mut Game, table: &EndgameTable) -> ChessMove{
    if game.get_board().get_all_piece_count() <= 4 {
        println!("Endgame move");
        return end_game_move(game, table);
    }

    return iterative_deepening(game, MAX_DEPTH, table).0; 
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

    let mut map = HashMap::new();

    static_eval(game, true);

    let mut start = Instant::now();
    let mut pair: (ArrayVec<ChessMove, 30>, i32) = (ArrayVec::new(), 0);
    for md in 1..(max_depth + 1) {
        pair = alpha_beta_nega_max(game, -MAX_VALUE, MAX_VALUE,  md, table, &mut map);
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

fn move_sorter(list: &mut ArrayVec<ChessMove, 200>, prev_best: ChessMove) {
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

pub fn alpha_beta_nega_max(game: &mut Game, mut alpha: i32, beta: i32, depth_left: u8, table: &EndgameTable, map: &mut HashMap<u64, (u8, ChessMove, i32)>) -> (ArrayVec<ChessMove, 30>, i32) {        
    if depth_left == 0 {
        //return (chess_move::NULL_MOVE, static_eval(game));
        return quiescence(game, alpha, beta, MAX_QUIESCENCE_DEPTH, table, map);
    }

    if game.get_game_state() != GameState::Undecided {
        return (ArrayVec::new(), static_eval(game, false));
    }
    
    let pair = get_relative_endgame_eval(&game.get_board(), table);
    if pair.0 {
        return (ArrayVec::new(), pair.1);
    }

    let mut hist_depth_left = 0_u8;
    let mut hist_move = chess_move::NULL_MOVE;
    let mut hist_value = i32::MAX;

    let hash = game.get_board().get_zoberist_hash();
    if map.contains_key(&hash) {
        (hist_depth_left, hist_move, hist_value) = map[&hash];

        if hist_depth_left >= depth_left {
            return (ArrayVec::new(), hist_value);
        }
    }

    if DO_NM_PRUNING && !game.get_board().in_check() && !game.last_move_is_null_move() {  
        let mut dl = 0;
        if depth_left > NULL_MOVE_REDUCTION + 1 {
            dl = depth_left - NULL_MOVE_REDUCTION -  1;
        }
        
        game.make_move(chess_move::NULL_MOVE);
        let (_, mut value) = alpha_beta_nega_max(game, -beta, -beta + 1,  dl, table, map);
        game.undo_move();

        value = -value;
        if value >= beta {
            return (ArrayVec::new(), value);
        }
    }

    let mut best_line = ArrayVec::new();

    let mut list = game.get_legal_moves();

    move_sorter(&mut list, hist_move);

    for m in  list {
        
        game.make_move(m);

        let (line, mut value) = alpha_beta_nega_max(game,  -beta, -alpha, depth_left - 1, table, map);

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

    if best_line.len() > 0 {
        if map.contains_key(&hash) {
            *map.get_mut(&hash).unwrap() = (depth_left, *best_line.last().unwrap(), alpha);
        }
        else {
            map.insert(hash, (depth_left, *best_line.last().unwrap(), alpha));
        }
    }
    

    return (best_line, alpha);
}

pub fn quiescence(game: &mut Game, mut alpha: i32, beta: i32, depth_left: u8, table: &EndgameTable, map: &HashMap<u64, (u8, ChessMove, i32)>) -> (ArrayVec<ChessMove, 30>, i32) {
    
    let board = game.get_board();

    let pair = get_relative_endgame_eval(&game.get_board(), table);
    if pair.0 {
        return (ArrayVec::new(), pair.1);
    }

    
    if game.get_game_state() == GameState::Undecided && board.in_check() {
        return check_avoid_search(game, alpha, beta, depth_left, table, map);
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


    let mut hist_depth_left = 0_u8;
    let mut hist_move = chess_move::NULL_MOVE;
    let mut hist_value = i32::MAX;

    let hash = game.get_board().get_zoberist_hash();
    if map.contains_key(&hash) {
        (hist_depth_left, hist_move, hist_value) = map[&hash];

        if hist_depth_left >= depth_left {
            return (ArrayVec::new(), hist_value);
        }
    }
    move_sorter(&mut list, hist_move);

    //[TODO] quiescence search move gen
    for m in  list {
        
        if !(m.is_direct_capture() || m.is_en_passant() || m.is_promotion()) {
            continue;
        }

        game.make_move(m);
        let (line, mut value) = quiescence(game,  -beta, -alpha, depth_left - 1, table, map);
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

pub fn check_avoid_search(game: &mut Game, mut alpha: i32, beta: i32, depth_left: u8, table: &EndgameTable, map: &HashMap<u64, (u8, ChessMove, i32)>) -> (ArrayVec<ChessMove, 30>, i32) {
    
    let pair = get_relative_endgame_eval(&game.get_board(), table);
    if pair.0 {
        return (ArrayVec::new(), pair.1);
    }
    
    let mut best_line = ArrayVec::new();

    let mut list = game.get_legal_moves();

    let mut hist_depth_left = 0_u8;
    let mut hist_move = chess_move::NULL_MOVE;
    let mut hist_value = i32::MAX;

    let hash = game.get_board().get_zoberist_hash();
    if map.contains_key(&hash) {
        (hist_depth_left, hist_move, hist_value) = map[&hash];

        if hist_depth_left >= depth_left {
            return (ArrayVec::new(), hist_value);
        }
    }

    move_sorter(&mut list, hist_move);

    for m in  list {
        game.make_move(m);
        let (line, mut value) = quiescence(game,  -beta, -alpha, depth_left, table, map);
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