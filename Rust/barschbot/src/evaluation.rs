use core::panic;
use std::{collections::HashMap, ops::Deref};

use crate::{game::{Game, GameState}, colored_piece_type::ColoredPieceType, piece_type::PieceType, bitboard_helper, endgame_table::EndgameTable, square::{self, Square}, zoberist_hash, bb_settings::EvalFactors, bit_board::BitBoard, constants};

pub const CHECKMATE_VALUE: f32 = f32::MAX;
//                              Pawn, Knight, Bishop, Rook, Queen, King

pub struct EvalAttributes {
    pub piece_dif: [i32; 5], 
    pub safe_mobility_dif: [i32; 6], 
    pub unsafe_mobility_dif: [i32; 6],

    pub material_sum: i32,
    pub sq_control_dif: i32,

    pub pawn_push_dif: [i32; 6], 
    pub passed_pawn_dif: i32, 
    pub doubled_pawn_dif: i32, 
    pub isolated_pawn_dif: i32, 

    pub knight_outpost_dif: i32,

    //Number of moves a Queen and Knight could do at the king pos
    pub king_qn_moves_dif: i32,
    //Number of controlled squares by the opponent the king can move to
    pub king_control_dif: i32,
    //Number of safe moves to a square the opponent king can move to
    pub safe_check_dif: i32,
    //Number of unsafe moves to a square the opponent king can move to
    pub unsafe_check_dif: i32,
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

        if self.king_control_dif != 0 {
            println!("King attack dif: {}", self.king_control_dif);
        }
    }

    pub fn get_vector(&self) -> Vec<f32> {
        let mut list = Vec::new();

        for v in self.piece_dif {
            list.push(v as f32);
        }

        for v in self.safe_mobility_dif {
            list.push(v as f32);
        }

        for v in self.unsafe_mobility_dif {
            list.push(v as f32);
        }

        for v in self.pawn_push_dif {
            list.push(v as f32);
        }

        list.push(self.passed_pawn_dif as f32);
        list.push(self.doubled_pawn_dif as f32);
        list.push(self.isolated_pawn_dif as f32);

        list.push(self.knight_outpost_dif as f32);

        list.push(self.king_qn_moves_dif as f32);
        list.push(self.king_control_dif as f32);
        list.push(self.safe_check_dif as f32);
        list.push(self.unsafe_check_dif as f32);

        return list;
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

        if do_print {
            println!("Check eval!!!!!!!!!!!")
        }
        else {
            panic!("In check");
        }
    }

    if do_print {
        board.print_local_moves(&board.generate_legal_moves_eval(true));
        board.print_local_moves(&board.generate_legal_moves_eval(false));
    } 

    let attributes = generate_eval_attributes(&board);

    if do_print {
        attributes.print();
    }

    let sum = factors.evaluate(&attributes);
     
    return (sum * if game.is_whites_turn() { 1.0 } else { -1.0 }, gs);
}

pub fn generate_eval_attributes(board: &BitBoard) -> EvalAttributes {
    let mut piece_count = [0; 5];
    const MAT_SUM_VAL: [i32; 5] = [0, 3, 3, 5, 9];
    let mut material_sum = 0;
    for i in 0..5 {
        let pt = PieceType::from_u8(i as u8);

        let wc = board.get_piece_count(ColoredPieceType::from_pt(pt, true)) as i32;
        let bc = board.get_piece_count(ColoredPieceType::from_pt(pt, false)) as i32;
        piece_count[i as usize] =  wc - bc; 

        material_sum += MAT_SUM_VAL[pt as usize] * (wc + bc);        
    }

    let mut white_list = board.generate_legal_moves_eval(true);
    let mut black_list = board.generate_legal_moves_eval(false);


    //white_list.sort_unstable_by(|a, b| { return a.get_uci().cmp(&b.get_uci())});
    //black_list.sort_unstable_by(|a, b| { return a.get_uci().cmp(&b.get_uci())});
    //board.print_local_moves(&white_list);
    //board.print_local_moves(&black_list);

    let mut least_valueable_attacker_white = [PieceType::None; 64];
    let mut least_valueable_attacker_black = [PieceType::None; 64];

    let mut static_exchange_evaluation = [0; 64];
        
    //Compute controlled squares (SEE) and LVA
    const PIECE_ATTACK_SCORE: [i32; 7] = [10000, 1000, 1000, 100, 10, 1, 0];
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


    let mut white_controlled_sq = 0;
    let mut black_controlled_sq = 0;
    for i in 0..64 {
        bitboard_helper::set_bit(&mut white_controlled_sq, Square::from_u8(i), 
            static_exchange_evaluation[i as usize] > 50);

        bitboard_helper::set_bit(&mut black_controlled_sq, Square::from_u8(i), 
            static_exchange_evaluation[i as usize] < -50);
    }

    let mut sq_control_count = white_controlled_sq.count_ones() as i32 - black_controlled_sq.count_ones() as i32;

    let mut safe_mobility_count = [0; 6];
    let mut unsafe_mobility_count = [0; 6];

    let white_king_pos = board.get_king_square(true);
    let black_king_pos = board.get_king_square(false);

    let white_king_queen_mask = board.get_queen_moves(white_king_pos);
    let black_king_queen_mask = board.get_queen_moves(black_king_pos);

    let white_king_knight_mask = bitboard_helper::KNIGHT_ATTACKS[white_king_pos as usize];
    let black_king_knight_mask = bitboard_helper::KNIGHT_ATTACKS[black_king_pos as usize];

    //Amount of moves a Queen and Knight could do at the position of the King
    let king_qn_move_count = (white_king_queen_mask.count_ones() as i32 - black_king_queen_mask.count_ones() as i32) 
                            + (white_king_knight_mask.count_ones() as i32 - black_king_knight_mask.count_ones() as i32);
    
    //Every check one side can give without loosing material
    let mut safe_king_attacks = 0;
    //Every check one side can give that would loose material
    let mut unsafe_king_attacks = 0;

    //Compute safe mobility
    for m in &white_list {
        let mpt = PieceType::from_cpt(m.move_piece_type);
        let capture_pt = PieceType::from_cpt(board.get_piece_type(m.target_square));

        //if attacking opponent piece SEE can be increased
        //if attacking move (not forward pawn move) SEE has to be lowerd
        let see = static_exchange_evaluation[m.target_square as usize] 
            + if m.is_defence() { 0 } else { PIECE_ATTACK_SCORE[capture_pt as usize] } 
            - if m.is_attack() { PIECE_ATTACK_SCORE[mpt as usize] } else { 0 };

        let is_safe = see >= 0;
        
        //println!("{} is {}", m.get_board_name(board), if is_safe {"safe"} else {"unsafe"});

        if is_safe {
            safe_mobility_count[mpt as usize] += 1;
        }
        else {
            unsafe_mobility_count[mpt as usize] += 1;
        }

        if bitboard_helper::get_bit(black_king_queen_mask, m.target_square) && 
            (m.target_square.is_orthogonal_to(black_king_pos) && mpt.is_orthogonal_slider() ||
            !m.target_square.is_orthogonal_to(black_king_pos) && mpt.is_diagonal_slider()) {
                
            if is_safe {
                safe_king_attacks += 1;
            }
            else {
                unsafe_king_attacks += 1;
            }
        }

        if bitboard_helper::get_bit(black_king_knight_mask, m.target_square) && 
            mpt == PieceType::Knight {
            
            if is_safe {
                safe_king_attacks += 1;
            }
            else {
                unsafe_king_attacks += 1;
            }
        }
    }

    for m in &black_list {
        let mpt = PieceType::from_cpt(m.move_piece_type);
        let capture_pt = PieceType::from_cpt(board.get_piece_type(m.target_square));
        
        let see = static_exchange_evaluation[m.target_square as usize] 
            - if m.is_defence() { 0 } else { PIECE_ATTACK_SCORE[capture_pt as usize] } 
            + if m.is_attack() { PIECE_ATTACK_SCORE[mpt as usize] } else { 0 };

        let is_safe = see <= 0;
        
        if is_safe {
            safe_mobility_count[mpt as usize] -= 1;
        }
        else {
            unsafe_mobility_count[mpt as usize] -= 1;
        }

        if bitboard_helper::get_bit(white_king_queen_mask, m.target_square) && 
            (m.target_square.is_orthogonal_to(white_king_pos) && mpt.is_orthogonal_slider() ||
            !m.target_square.is_orthogonal_to(white_king_pos) && mpt.is_diagonal_slider()) {
                
            if is_safe {
                safe_king_attacks -= 1;
            }
            else {
                unsafe_king_attacks -= 1;
            }
        }

        if bitboard_helper::get_bit(white_king_knight_mask, m.target_square) && 
            mpt == PieceType::Knight {
            
            if is_safe {
                safe_king_attacks -= 1;
            }
            else {
                unsafe_king_attacks -= 1;
            }
        }
    }

    //Knight outposts
    let mut knight_outposts = 0;
    for i in bitboard_helper::iterate_set_bits(board.get_piece_bitboard(ColoredPieceType::WhiteKnight)) {
        if bitboard_helper::NEIGHBOUR_FILES[(i % 8) as usize] 
            & bitboard_helper::WHITE_PASSED_PAWN_MASK[i as usize] 
            & board.get_piece_bitboard(ColoredPieceType::BlackPawn) == 0 {

            knight_outposts += 1;
        }
    }

    for i in bitboard_helper::iterate_set_bits(board.get_piece_bitboard(ColoredPieceType::BlackKnight)) {
        if bitboard_helper::NEIGHBOUR_FILES[(i % 8) as usize] 
            & bitboard_helper::BLACK_PASSED_PAWN_MASK[i as usize] 
            & board.get_piece_bitboard(ColoredPieceType::WhitePawn) == 0 {

            knight_outposts -= 1;
        }
    }

    let (passed_pawns, doubled_pawns, isolated_pawns, pawn_ranks) 
        = eval_pawn_structure(board);
    
    //King safety
    //Black king moves
    let mut king_control = 0;
    for s in constants::KING_MOVES[board.get_king_square(false) as usize] {
        if static_exchange_evaluation[*s as usize] > 0 {
            king_control += 1;
        }
    }
    for s in constants::KING_MOVES[board.get_king_square(true) as usize] {
        if static_exchange_evaluation[*s as usize] < 0 {
            king_control -= 1;
        }
    }
    
    return EvalAttributes {
        piece_dif: piece_count,
        safe_mobility_dif: safe_mobility_count,
        unsafe_mobility_dif: unsafe_mobility_count,

        material_sum: material_sum,
        sq_control_dif: sq_control_count,

        pawn_push_dif: pawn_ranks,
        passed_pawn_dif: passed_pawns,
        doubled_pawn_dif: doubled_pawns,
        isolated_pawn_dif: isolated_pawns,

        knight_outpost_dif: knight_outposts,

        king_qn_moves_dif: king_qn_move_count,
        king_control_dif: king_control, 
        safe_check_dif: safe_king_attacks,
        unsafe_check_dif: unsafe_king_attacks,
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
            //println!("I: {}", i);
            if opponent_pawns & pawn_mask[i as usize] == 0 {
                count += 1;

                //println!("Passed pawn: {}", Square::from_u8(i as u8).to_string());
                //println!("Opponents: ");
                //bitboard_helper::print_bitboard(opponent_pawns);
                //bitboard_helper::print_bitboard(pawn_mask[i as usize]);
            }
        }

        return count;
    }

    fn count_doubled_pawns(pawn_bitboard: u64) -> i32 {
        let mut buffer = pawn_bitboard << 8;
        let mut count = 0;

        buffer |= buffer << 8;
        buffer |= buffer << 16;
        buffer |= buffer << 32;

        return (pawn_bitboard & buffer).count_ones() as i32;
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

#[cfg(test)]
mod tests {
    use crate::bb_settings;

    use super::*;

    #[test]
    fn test_generate_eval_attributes_start_board() {
        let board = BitBoard::start_position();
        let attributes = generate_eval_attributes(&board);

        // Check that all attributes are zero for an empty board
        assert_eq!(attributes.pawn_push_dif, [0_i32; 6]);
        assert_eq!(attributes.piece_dif, [0_i32; 5]);
        assert_eq!(attributes.safe_mobility_dif, [0_i32; 6]);
        assert_eq!(attributes.unsafe_mobility_dif, [0_i32; 6]);
        assert_eq!(attributes.passed_pawn_dif, 0);
        assert_eq!(attributes.doubled_pawn_dif, 0);
        assert_eq!(attributes.isolated_pawn_dif, 0);
        assert_eq!(attributes.king_qn_moves_dif, 0);
        assert_eq!(attributes.king_control_dif, 0);
        assert_eq!(attributes.safe_check_dif, 0);
        assert_eq!(attributes.unsafe_check_dif, 0);
        assert_eq!(attributes.material_sum, bb_settings::MAX_MATERIAL_SUM)
    }
    #[test]
    fn test_generate_eval_attributes_symmetric_full_board() {
        let board = BitBoard::from_fen("2k1rb1r/pbpnqpp1/1p3n2/3pp2p/3PP2P/1P3N2/PBPNQPP1/2K1RB1R w Kk - 0 1");
        let attributes = generate_eval_attributes(&board);

        // Check that all attributes are zero for an empty board
        assert_eq!(attributes.pawn_push_dif, [0_i32; 6]);
        assert_eq!(attributes.piece_dif, [0_i32; 5]);
        assert_eq!(attributes.safe_mobility_dif, [0_i32; 6]);
        assert_eq!(attributes.unsafe_mobility_dif, [0_i32; 6]);
        assert_eq!(attributes.passed_pawn_dif, 0);
        assert_eq!(attributes.doubled_pawn_dif, 0);
        assert_eq!(attributes.isolated_pawn_dif, 0);
        assert_eq!(attributes.king_qn_moves_dif, 0);
        assert_eq!(attributes.king_control_dif, 0);
        assert_eq!(attributes.safe_check_dif, 0);
        assert_eq!(attributes.unsafe_check_dif, 0);
        assert_eq!(attributes.material_sum, bb_settings::MAX_MATERIAL_SUM)
    }

    #[test]
    fn test_eval_pawn_structure_uneaven() {
        let board = BitBoard::from_fen("7k/1p6/8/4pP2/3p4/1P1P1P2/P5P1/7K w - - 0 1");

        //Passed pawns 2 - 0
        //Doubled pawns 1 - 0
        //Isolated pawns 1 - 1
        //Pawn ranks 
        //2 - 1
        //3 - 0
        //0 - 1
        //1 - 1
        //0 - 0
        //0 - 0

        let (passed_pawns, doubled_pawns, isolated_pawns, pawn_ranks) 
        = eval_pawn_structure(&board);

        assert_eq!(passed_pawns, 2);
        assert_eq!(doubled_pawns, 1);
        assert_eq!(isolated_pawns, 0);
        assert_eq!(pawn_ranks, [1, 3, -1, 0, 0, 0]);
    }


    #[test]
    fn test_knight_outpost() {
        //https://lichess.org/editor/k7/4p2p/p1Np4/NppN3N/1N6/8/8/K5N1_w_-_-_0_1?color=white
        let board = BitBoard::from_fen("k7/4p2p/p1Np4/NppN3N/1N6/8/8/K5N1 w - - 0 1");

        let attr = generate_eval_attributes(&board);

        assert_eq!(attr.knight_outpost_dif, 3);
    }

    #[test]
    fn test_eval_pawn_structure_uneven_2() {
        let board = BitBoard::from_fen("7k/1p3P2/1P2p1P1/1Pp2p2/1P6/1P1p4/1P6/7K w - - 0 1");

        
        let (passed_pawns, doubled_pawns, isolated_pawns, pawn_ranks) 
        = eval_pawn_structure(&board);
        
        //Passed pawns 2 - 3
        assert_eq!(passed_pawns, -1);
        //Doubled pawns 4 - 0
        assert_eq!(doubled_pawns, 4);
        //Isolated pawns 5 - 0
        assert_eq!(isolated_pawns, 5);
        //Pawn ranks 
        //1 - 1
        //1 - 1
        //1 - 2
        //1 - 0
        //2 - 1
        //1 - 0
        assert_eq!(pawn_ranks, [0, 0, -1, 1, 1, 1]);
    }

    
    #[test]
    fn test_king_safety() {
        //https://lichess.org/editor/1kp5/1pp5/8/2br2n1/8/8/5P1P/6K1_w_-_-_0_1?color=white
        let board = BitBoard::from_fen("1kp5/1pp5/8/2br2n1/8/8/5P1P/6K1 w - - 0 1");
        let attributes = generate_eval_attributes(&board);

        //3n + 11r + 2b - (3n + 3r + 2b)
        assert_eq!(attributes.king_qn_moves_dif, 8);
        //-1
        assert_eq!(attributes.king_control_dif, -1);
        //2n + 1b + 2r
        assert_eq!(attributes.safe_check_dif, -4);
        //0
        assert_eq!(attributes.unsafe_check_dif, -1);
    }

    #[test]
    fn test_pawn_moblility() {
        //https://lichess.org/editor/8/8/7r/P1PP2k1/6P1/8/5PP1/4K3_w_-_-_0_1?color=white
        let board = BitBoard::from_fen("8/8/7r/P1PP2k1/6P1/8/5PP1/4K3 w - - 0 1");
        let attributes = generate_eval_attributes(&board);

        assert_eq!(attributes.safe_mobility_dif[0], 9);
        assert_eq!(attributes.unsafe_mobility_dif[0], 8);
    }
    
}