use std::cmp;
use std::time::Instant;

use arrayvec::ArrayVec;
use neuroflow::FeedForward;
use neuroflow::data::DataSet;
use neuroflow::io;
use neuroflow::activators::Type::Tanh;

use crate::bit_board::BitBoard;
use crate::chess_move::ChessMove;
use crate::colored_piece_type::ColoredPieceType;
use crate::{constants, bitboard_helper};
use crate::game::{Game, GameState};
use crate::piece_type::PieceType;
use crate::square::Square;

const MAX_VALUE: f64 = 2_000_000_000.0;
const MAX_DEPTH: u8 = 5;
const MAX_QUIESCENCE_DEPTH: u8 = 5;

pub fn get_best_move(game: &mut Game) -> ChessMove{
    let path = "C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\barschbot\\target\\release\\Error0.021.flow";

    let mut new_nn: FeedForward = io::load(path).unwrap();

    return iterative_deepening(game, MAX_DEPTH, &mut new_nn).0; 
}

pub fn iterative_deepening(game: &mut Game, max_depth: u8, nn: &mut FeedForward) -> (ChessMove, f64) {
    let mut start = Instant::now();

    let mut pair: (ArrayVec<ChessMove, 30>, f64) = (ArrayVec::new(), 0.0);
    for md in 1..(max_depth + 1) {
        pair = alpha_beta_nega_max(game, -MAX_VALUE, MAX_VALUE,  md, nn);
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

    return (*pair.0.last().unwrap(), pair.1);
}

fn move_sorter(list: &mut ArrayVec<ChessMove, 200>) {
    const PIECE_VALUES: [i32; 6] = [
        100, 280, 320, 500, 900, 1337_42,
    ];

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

pub fn alpha_beta_nega_max(game: &mut Game, mut alpha: f64, beta: f64, depth_left: u8, nn: &mut  FeedForward) -> (ArrayVec<ChessMove, 30>, f64) {
    if depth_left == 0 {
        //return (chess_move::NULL_MOVE, static_eval(game));
        return quiescence(game, alpha, beta, MAX_QUIESCENCE_DEPTH, nn);
    }
    
    let mut best_line = ArrayVec::new();

    let mut list = game.get_legal_moves();

    move_sorter(&mut list);

    for m in  list {
        
        game.make_move(m);

        let (line, mut value) = alpha_beta_nega_max(game,  -beta, -alpha, depth_left - 1, nn);

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

pub fn quiescence(game: &mut Game, mut alpha: f64, beta: f64, depth_left: u8, nn: &mut FeedForward) -> (ArrayVec<ChessMove, 30>, f64) {
    let stand_pat = static_eval(game, nn);
    
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

        let (line, mut value) = quiescence(game,  -beta, -alpha, depth_left - 1, nn);
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

const CHECK_MATE_VALUE: f64 = 1_000_000_000.0;

//Piece value [Pt] 
//Mobillity [Pt]
//Center attacks [Pt]
//King attacks
//King defences
//Passed pawns
//Doubled pawns
//Pawns on rank [2..6]
pub fn get_eval_vector(board: &BitBoard) -> Vec<f64> {
    let mut list = Vec::new();
    for i in 0..6 {
        list.push(board.get_piece_count(ColoredPieceType::from_u8(i * 2 + 0)) as f64 
        - board.get_piece_count(ColoredPieceType::from_u8(i * 2 + 1)) as f64);
    }
    
    //whites perspective
    let white_list = board.get_pseudo_legal_moves(true);
    let black_list = board.get_pseudo_legal_moves(false);

    //Mobility
    let mut mob_count = [0; 6];
    for m in &white_list {
        mob_count[PieceType::from_cpt(m.move_piece_type) as usize] += 1;
    }

    for m in &black_list {
        mob_count[PieceType::from_cpt(m.move_piece_type) as usize] -= 1;
    }

    for m in mob_count {
        list.push(m as f64);
    }


    //Center attack
    let mut center_score = [0; 6];

    fn is_center_square(square: Square) -> bool {
        const CENTER_SQUARES: [Square; 8] = [
            Square::C4, Square::D4, Square::E4, Square::F4, 
            Square::C5, Square::D5, Square::E5, Square::F5, 
        ];
        return CENTER_SQUARES.contains(&square);
    }

    for m in &white_list {
        if is_center_square(m.target_square) {
            center_score[PieceType::from_cpt(m.move_piece_type) as usize] += 1;
        }   
    }

    for m in &black_list {
        if is_center_square(m.target_square) {
            center_score[PieceType::from_cpt(m.move_piece_type) as usize] -= 1;
        }   
    }

    for pc in center_score {
        list.push(pc as f64);
    }

    //King safety
    let mut attacks_on_white_king = 0;
    let mut defence_on_white_king = 0;
    let mut attacks_on_black_king = 0;
    let mut defence_on_black_king = 0;

    for m in &white_list {
        //defences on white king 
        if constants::KING_MOVES[board.get_king_square(true) as usize].contains(&(m.target_square as u8)) {
            defence_on_white_king += 1;
        }
        //attacks on black king
        if constants::KING_MOVES[board.get_king_square(false) as usize].contains(&(m.target_square as u8)) {
            attacks_on_black_king += 1;
        }   
    }

    for m in &black_list {
        //defences on white king 
        if constants::KING_MOVES[board.get_king_square(false) as usize].contains(&(m.target_square as u8)) {
            defence_on_black_king += 1;
        }
        //attacks on black king
        if constants::KING_MOVES[board.get_king_square(true) as usize].contains(&(m.target_square as u8)) {
            attacks_on_white_king += 1;
        }   
    }

    list.push(attacks_on_white_king as f64 - attacks_on_black_king as f64);
    list.push(defence_on_white_king as f64 - defence_on_black_king as f64);
    

    //Pawn structure
    let white_pawns_bitboard = board.get_piece_bitboard(ColoredPieceType::WhitePawn);
    let black_pawns_bitboard = board.get_piece_bitboard(ColoredPieceType::BlackPawn);

    let white_passed_pawns = count_passed_pawns(white_pawns_bitboard, black_pawns_bitboard, bitboard_helper::WHITE_PASSED_PAWN_MASK);
    let black_passed_pawns = count_passed_pawns(black_pawns_bitboard, white_pawns_bitboard, bitboard_helper::BLACK_PASSED_PAWN_MASK);

    list.push(white_passed_pawns as f64 - black_passed_pawns as f64);

    let white_doubled_pawns = count_doubled_pawns(white_pawns_bitboard);
    let black_doubled_pawns = count_doubled_pawns(black_pawns_bitboard);

    list.push(white_doubled_pawns as f64 - black_doubled_pawns as f64);
    
    let mut pawn_ranks = [0.0; 8];
    //dont have to check last or first rank
    for i in 1..7 {
        pawn_ranks[i] += (white_pawns_bitboard & bitboard_helper::RANK_MASKS[i]).count_ones() as f64;
        pawn_ranks[7 - i] -= (black_pawns_bitboard & bitboard_helper::RANK_MASKS[i]).count_ones() as f64;
    }

    for i in 1..7 {
        list.push(pawn_ranks[i] as f64);
    }
    
    return list;

    fn count_passed_pawns(allied_pawns: u64, opponent_pawns: u64, pawn_mask: [u64; 64]) -> u32 {
        let mut count = 0;

        for i in bitboard_helper::iterate_set_bits(allied_pawns) {
            if opponent_pawns & pawn_mask[i as usize] == 0 {
                count += 1;
            }
        }

        return count;
    }

    fn count_doubled_pawns(pawn_bitboard: u64) -> u32 {
        let mut buffer = pawn_bitboard;
        let mut count = 0;
        for i in 0..8 {
            buffer <<= 8;

            count += (pawn_bitboard & buffer).count_ones();
        }

        return count;
    }
}

const VECTOR_LENGTH: usize = 12 * 64 + 5;

pub fn get_neutral_vector(board: &BitBoard) -> Vec<f64> {
    let mut list = vec![0.0; VECTOR_LENGTH];

    let type_field = board.type_field;

    for i in 0..64 {
        let pt = type_field[i];

        if pt != ColoredPieceType::None {
            list[i * 12 + (pt as usize)] = 1.0;
        }
    }

    let offset = 12 * 64;
    if board.white_queen_castle {
        list[offset + 0] = 1.0;
    }
    if board.white_king_castle {
        list[offset + 1] = 1.0;
    }
    if board.black_queen_castle {
        list[offset + 2] = 1.0;
    }
    if board.black_king_castle {
        list[offset + 3] = 1.0;
    }

    if board.is_whites_turn() {
        list[offset + 4] = 1.0;
    }

    return list;
}



pub fn eval_board(board: &BitBoard, nn: &mut FeedForward) -> f64 {
    let v = get_eval_vector(board);

    return nn.calc(&v)[0];
}

pub fn static_eval(game: &mut Game, nn: &mut FeedForward) -> f64 {
    
    let factor = (if game.is_whites_turn() { 1 } else { -1 }) as f64;

    let gs = game.get_game_state();
    if gs.is_checkmate() {
        return CHECK_MATE_VALUE * factor;
    }

    if gs.is_draw() {
        return 0.0;
    }

    
    return eval_board(&game.get_board(), nn) * -factor;
}

