use core::panic;
use std::char;
use arrayvec::ArrayVec;

use crate::{bitboard_helper::{self, toggle_bit}, chess_move::ChessMove, square::Square, colored_piece_type::ColoredPieceType, piece_type::PieceType, endgame_table::BoardState, zoberist_hash::ZoberistHash64};



//const DEBUG: bool = std::cfg!(debug_assertions);


#[derive(Clone, Copy)]
pub struct BitBoard {
    whites_turn: bool,

    pub white_king_castle: bool,
    pub black_king_castle: bool,
    pub white_queen_castle: bool,
    pub black_queen_castle: bool,

    en_passant_square: Square,

    white_pieces: u64,
    black_pieces: u64,
    pawns: u64,
    knights: u64,
    orthogonal_sliders: u64,
    diagonal_sliders: u64,
    kings: u64,

    pub type_field: [ColoredPieceType; 64]
}

impl BitBoard {
    pub fn empty() -> Self {
        return BitBoard { whites_turn: true, white_queen_castle: false, white_king_castle: false, black_queen_castle: false, black_king_castle: false,
            en_passant_square: Square::None, 
            white_pieces: 0, black_pieces: 0, pawns: 0, knights: 0, orthogonal_sliders: 0, diagonal_sliders: 0, kings: 0, type_field: [ColoredPieceType::None; 64] };
    }

    pub fn start_position() -> Self {
        return Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - -");
    }

    pub fn is_whites_turn(&self) -> bool {
        return self.whites_turn;
    }

    pub fn set_whites_turn(&mut self, whites_turn: bool) {
        self.whites_turn = whites_turn;
    }

    pub fn from_type_field(type_field: [ColoredPieceType; 64]) -> Self {
        let mut board = BitBoard::empty(); 
        
        for i in 0..64 {
            if type_field[i] != ColoredPieceType::None {
                board.place_piece(type_field[i], Square::from_u8(i as u8));
            }
        }

        return board;
    }

    pub fn from_board_state(bs: &BoardState) -> Self {
        let mut board = BitBoard::empty(); 
        
        for i in 0..64 {
            if bs.type_field[i] != ColoredPieceType::None {
                board.place_piece(bs.type_field[i], Square::from_u8(i as u8));
            }
        }

        board.set_whites_turn(bs.whites_turn);
        board.en_passant_square = bs.ep_square;

        return board;
    }

    pub fn get_board_state(&self) -> BoardState {
        return BoardState { type_field: self.type_field.clone(), ep_square: self.en_passant_square, whites_turn: self.whites_turn }
    }

    pub fn get_all_piece_count(&self) -> u32 {
        return (self.white_pieces | self.black_pieces).count_ones();
    }
    //What kind of pizza should i orderd

    pub fn get_valid_mover(&mut self) -> (bool, bool) {

        let attacks_on_white = self.get_square_attacker(false, self.get_king_square(true));
        let attacks_on_black = self.get_square_attacker(true, self.get_king_square(false));

        //both in check
        if attacks_on_white.len() > 0 && attacks_on_black.len() > 0 {
            return (false, false);
        }

        if attacks_on_white.len() > 2 || attacks_on_black.len() > 2 {
            return (false, false); 
        }



        //if attacks_on_white.len() == 2 {
        //    if !is_valid_double_check(self.type_field[attacks_on_white[0] as usize] as u8,
        //        self.type_field[attacks_on_white[1] as usize] as u8) {
        //        return (false, false);
        //    }
        //}
//
        //if attacks_on_black.len() == 2 {
        //    if !is_valid_double_check(self.type_field[attacks_on_black[0] as usize] as u8,
        //        self.type_field[attacks_on_black[1] as usize] as u8) {
        //        return (false, false);
        //    }
        //}  

        if attacks_on_white.len() > 0 {
            return (true, false);
        }

        if attacks_on_black.len() > 0 {
            return (false, true);
        }

        return (true, true);
        
        fn is_valid_double_check(cpt1: u8, cpt2: u8) -> bool {
            let min = PieceType::from_u8(cpt1.min(cpt2) / 2);
            let max = PieceType::from_u8(cpt1.max(cpt2) / 2);

            return match (min, max) {
                (PieceType::Knight, PieceType::Bishop) => true,
                (PieceType::Knight, PieceType::Rook) => true,
                (PieceType::Knight, PieceType::Queen) => true,
                (PieceType::Bishop, PieceType::Rook) => true,
                (PieceType::Bishop, PieceType::Queen) => true,
                (PieceType::Rook, PieceType::Queen) => true,

                _ => false
            }
        }
    }

    pub fn get_valid_ep_squares(&mut self) -> Vec<Square> {
        let mut rank_5 = bitboard_helper::RANK_MASKS[if self.whites_turn { 4 } else { 3 }];
        let allied_pawns = self.pawns & rank_5 & if self.whites_turn { self.white_pieces } else { self.black_pieces };
        let opponent_pawns = self.pawns & rank_5 & if !self.whites_turn { self.white_pieces } else { self.black_pieces };

        let pawns_with_neighbours = bitboard_helper::shift_board(allied_pawns, 1, 0) 
            | bitboard_helper::shift_board(allied_pawns, -1, 0);
            
        let shift_direction = if self.whites_turn { -1 } else { 1 };

        let all_pieces = self.white_pieces | self.black_pieces;
        let occupied_squares = bitboard_helper::shift_board(all_pieces, 0, shift_direction) 
            | bitboard_helper::shift_board(all_pieces, 0, shift_direction * 2);
        
        
        let mut list = vec![Square::None];

        for index in bitboard_helper::iterate_set_bits(opponent_pawns & pawns_with_neighbours & !occupied_squares) {
            list.push(Square::from_u8(index as u8 - (shift_direction * 8) as u8 ));    
        }

        return list;
    }

    pub fn is_valid_position(&mut self) -> bool {

        let attacks_on_white = self.get_square_attacker(true, self.get_king_square(false));
        let attacks_on_black = self.get_square_attacker(false, self.get_king_square(true));

        //both in check
        if attacks_on_white.len() > 0 && attacks_on_black.len() > 0 {
            return false;
        }

        if attacks_on_white.len() > 2 || attacks_on_black.len() > 2 {
            return false 
        }

        if attacks_on_white.len() == 2 {
            if !is_valid_double_check(self.type_field[attacks_on_white[0] as usize] as u8,
                self.type_field[attacks_on_white[1] as usize] as u8) {
                return false;
            }
        }

        if attacks_on_black.len() == 2 {
            if !is_valid_double_check(self.type_field[attacks_on_black[0] as usize] as u8,
                self.type_field[attacks_on_black[1] as usize] as u8) {
                return false;
            }
        }   

        return true;
        
        fn is_valid_double_check(cpt1: u8, cpt2: u8) -> bool {
            let min = PieceType::from_u8(cpt1.min(cpt2) / 2);
            let max = PieceType::from_u8(cpt1.max(cpt2) / 2);

            return match (min, max) {
                (PieceType::Knight, PieceType::Bishop) => true,
                (PieceType::Knight, PieceType::Rook) => true,
                (PieceType::Knight, PieceType::Queen) => true,
                (PieceType::Bishop, PieceType::Rook) => true,
                (PieceType::Bishop, PieceType::Queen) => true,
                (PieceType::Rook, PieceType::Queen) => true,

                _ => false
            }
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        let parts = fen.split(" ").collect::<Vec<_>>();
        let mut board = BitBoard::empty();
        let mut square = 64 - 8;

        const ALL_CHARS: &str = "rnbqkpRNBQKP12345678/";

        for c in parts[0].chars() {
            if ALL_CHARS.find(c) == None {
                continue;
            }
            if c == '/' {
                square -= 16;
                continue;
            }
            
            let piece = ColoredPieceType::from_char(c);
            
            if piece != ColoredPieceType::None {
                board.place_piece(piece,  Square::from_u8(square));
                
                square += 1;
            }
            else {
                square += c.to_string().parse::<u8>().unwrap();
            }
        }

        board.whites_turn = parts[1] == "w";

        for c in parts[2].chars() {
            match c {
                'K' => board.white_king_castle = true,
                'Q' => board.white_queen_castle = true,
                'k' => board.black_king_castle = true,
                'q' => board.black_queen_castle = true,
                _ => ()
            }
        }

        if parts[3] != "-" {
            board.en_passant_square = Square::from_str(parts[3]);
        }

        //println!("Loaded FEN {}", fen);

        return board;
    }

    pub fn get_fen(&self) -> String {
        let mut s = "".to_owned();
        for y in (0..8).rev() {
            let mut empty_count = 0;
            for x in 0..8 {
                let square = x + y * 8;
                let pt = self.type_field[square as usize];
                if pt != ColoredPieceType::None {
                    if empty_count > 0 {
                        s += &empty_count.to_string();
                        empty_count = 0;
                    }

                    s += &pt.get_char().to_string();
                }
                else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                s += &empty_count.to_string();
            }

            if y != 0 {
                s += "/";
            }
        }


        s += " ";
        if self.whites_turn {
            s += "w";
        }
        else {
            s += "b";
        }
        
        s += " ";
        if self.white_king_castle || self.white_queen_castle || self.black_king_castle || self.black_queen_castle {
            if self.white_king_castle {
                s += "K";
            }

            if self.white_queen_castle {
                s += "Q";
            }

            if self.black_king_castle {
                s += "k";
            }

            if self.black_queen_castle {
                s += "q";
            }
        }
        else {
            s += "-";
        }

        s += " ";
        if self.en_passant_square == Square::None {
            s += "-";
        }
        else {
            s += &self.en_passant_square.to_string();
        }

        return s;
    }

    pub fn get_zoberist_hash(&self) -> u64 {
        return ZoberistHash64::calculate_hash(&self.type_field, self.whites_turn, self.en_passant_square, 
            self.white_queen_castle, self.white_king_castle, self.black_queen_castle, self.black_king_castle);
    }

    fn toggle_piece_bitboards(&mut self, colored_piece_type: ColoredPieceType, square: Square) {
        match PieceType::from_cpt(colored_piece_type) {
            PieceType::Pawn     => toggle_bit(&mut self.pawns, square),
            PieceType::Knight   => toggle_bit(&mut self.knights, square),
            PieceType::Bishop   => toggle_bit(&mut self.diagonal_sliders, square),
            PieceType::Rook     => toggle_bit(&mut self.orthogonal_sliders, square),
            PieceType::Queen    => { toggle_bit(&mut self.orthogonal_sliders, square); 
                                  toggle_bit(&mut self.diagonal_sliders, square) },
            PieceType::King     => toggle_bit(&mut self.kings, square),
            PieceType::None     => panic!("Toggling \"None\" piece")
        }

        if colored_piece_type.is_white() {
            toggle_bit(&mut self.white_pieces, square);
        }
        else {
            toggle_bit(&mut self.black_pieces, square);
        }
    }

    fn place_piece(&mut self, colored_piece_type: ColoredPieceType, square: Square) {
        self.toggle_piece_bitboards(colored_piece_type, square);
        self.type_field[square as usize] = colored_piece_type;
    }

    fn remove_piece(&mut self, square: Square) {
        let cpt = self.type_field[square as usize];
        self.toggle_piece_bitboards(cpt, square);
        self.type_field[square as usize] = ColoredPieceType::None;
    }

    fn move_piece(&mut self, start_square: Square, target_square: Square) {
        let cpt = self.type_field[start_square as usize];
        self.remove_piece(start_square);
        self.place_piece(cpt, target_square);
    }

    fn capture_piece(&mut self, start_square: Square, target_square: Square) {
        self.remove_piece(target_square);
        self.move_piece(start_square, target_square);
    }

    pub fn get_king_square(&self, white: bool) -> Square {
        let bitboard = self.kings & if white { self.white_pieces } else { self.black_pieces };
        return Square::from_u8(bitboard.trailing_zeros() as u8);
    }

    fn square_is_attacked_by(&self, white: bool, target_square: Square) -> bool {
        let color_mask = if white { self.white_pieces } else { self.black_pieces };
        
        let pawn_mask = if white { bitboard_helper::BLACK_PAWN_ATTACKS[target_square as usize] } else {  bitboard_helper::WHITE_PAWN_ATTACKS[target_square as usize] };

        if (self.kings & bitboard_helper::KING_ATTACKS[target_square as usize] 
            | self.knights & bitboard_helper::KNIGHT_ATTACKS[target_square as usize] 
            | self.pawns & pawn_mask) & color_mask != 0 {
                return true;
        }
       
        let all_pieces = self.white_pieces | self.black_pieces;
        
        let diagonal_attackers = self.diagonal_sliders & color_mask & bitboard_helper::DIAGONAL_ATTACKS[target_square as usize];
        if sliders_have_attack(target_square, diagonal_attackers, all_pieces) {
            return true;
        }

        let orthogonal_attackers = self.orthogonal_sliders & color_mask & bitboard_helper::ORTHOGONAL_ATTACKS[target_square as usize];
        if sliders_have_attack(target_square, orthogonal_attackers, all_pieces) {
            return true;
        }
        
        fn sliders_have_attack(target_square: Square, mut sliders: u64, blockers: u64) -> bool {
            for index in bitboard_helper::iterate_set_bits(sliders) {
                let in_between = bitboard_helper::get_in_between(target_square, Square::from_u8(index as u8));

                if blockers & in_between == 0 {
                    return true;
                }
            }

            return false;
        }

        return false;
    }

    pub fn get_piece_count(&self, colored_piece_type: ColoredPieceType) -> u32 {
        let color_mask = if colored_piece_type.is_white() { self.white_pieces } else { self.black_pieces };

        let bits = color_mask & match PieceType::from_cpt(colored_piece_type) {
            PieceType::Pawn => self.pawns,
            PieceType::Knight => self.knights,
            PieceType::Bishop => self.diagonal_sliders & !self.orthogonal_sliders,
            PieceType::Rook => !self.diagonal_sliders & self.orthogonal_sliders,
            PieceType::Queen => self.diagonal_sliders & self.orthogonal_sliders,
            PieceType::King => self.kings,
            _ => 0
        };

        return bits.count_ones();
    }

    pub fn get_piece_bitboard(&self, colored_piece_type: ColoredPieceType) -> u64 {
        let color_mask = if colored_piece_type.is_white() { self.white_pieces } else { self.black_pieces };

        return color_mask & match PieceType::from_cpt(colored_piece_type) {
            PieceType::Pawn => self.pawns,
            PieceType::Knight => self.knights,
            PieceType::Bishop => self.diagonal_sliders & !self.orthogonal_sliders,
            PieceType::Rook => !self.diagonal_sliders & self.orthogonal_sliders,
            PieceType::Queen => self.diagonal_sliders & self.orthogonal_sliders,
            PieceType::King => self.kings,
            _ => 0
        };
    }

    pub fn get_pawn_attacks (&self, white: bool) -> u64 {
        let color_mask = if white { self.white_pieces } else { self.black_pieces };
        
        if white {
            return bitboard_helper::capture_up_left(self.pawns & color_mask, u64::MAX) | 
                bitboard_helper::capture_up_right(self.pawns & color_mask, u64::MAX); 
        }
        else {
            return bitboard_helper::capture_down_left(self.pawns & color_mask, u64::MAX) | 
                bitboard_helper::capture_down_right(self.pawns & color_mask, u64::MAX); 
        }
    }

    pub fn get_piece_captures_at(&self, colored_piece_type: ColoredPieceType, square: Square) -> ArrayVec<PieceType, 10> {
        let opponent_mask = if colored_piece_type.is_white() { self.black_pieces } else { self.white_pieces };
        
        
        let mut res = 0_u64;
        match PieceType::from_cpt(colored_piece_type) {
            PieceType::Pawn => {
                if colored_piece_type.is_white() {
                    res = bitboard_helper::WHITE_PAWN_ATTACKS[square as usize] & opponent_mask;
                }
                else {
                    res = bitboard_helper::BLACK_PAWN_ATTACKS[square as usize] & opponent_mask;
                }
            },
            PieceType::Knight => {
                res = bitboard_helper::KNIGHT_ATTACKS[square as usize] & opponent_mask;
            },
            PieceType::King => {
                res = bitboard_helper::KING_ATTACKS[square as usize] & opponent_mask;
            },
            _ => ()
        }


        let all_mask = self.white_pieces | self.black_pieces;
        if colored_piece_type.is_diagonal_slider() {
            for index in bitboard_helper::iterate_set_bits(bitboard_helper::DIAGONAL_ATTACKS[square as usize] & opponent_mask) {
                let in_between = bitboard_helper::get_in_between(square, Square::from_u8(index as u8));

                if in_between & all_mask == 0 {
                    res |= 1_u64 << index;
                }
            }
        }

        if colored_piece_type.is_orthogonal_slider() {
            for index in bitboard_helper::iterate_set_bits(bitboard_helper::ORTHOGONAL_ATTACKS[square as usize] & opponent_mask) {
                let in_between = bitboard_helper::get_in_between(square, Square::from_u8(index as u8));

                if in_between & all_mask == 0 {
                    res |= 1_u64 << index;
                }
            }
        }

        let mut list = ArrayVec::new();
        for index in bitboard_helper::iterate_set_bits(res) {
            list.push(PieceType::from_cpt(self.type_field[index as usize]));
        }

        return list;
    }
    
    pub fn get_piece_moves_at(&self, piece_type: PieceType, square: Square) -> u8 {
        let all_mask = self.black_pieces | self.white_pieces;
        
        
        let mut res = 0_u64;
        match piece_type {
            PieceType::Pawn => {
                return 3;
            },
            PieceType::Knight => {
                return bitboard_helper::KNIGHT_ATTACKS[square as usize].count_ones() as u8;
            },
            PieceType::King => {
                return bitboard_helper::KING_ATTACKS[square as usize].count_ones() as u8;
            },
            _ => ()
        }

        let all_mask = self.white_pieces | self.black_pieces;
        if piece_type.is_diagonal_slider() {
            for index in bitboard_helper::iterate_set_bits(bitboard_helper::DIAGONAL_ATTACKS[square as usize]) {
                let in_between = bitboard_helper::get_in_between(square, Square::from_u8(index as u8));

                if in_between & all_mask == 0 {
                    res |= 1_u64 << index;
                }
            }
        }

        if piece_type.is_orthogonal_slider() {
            for index in bitboard_helper::iterate_set_bits(bitboard_helper::ORTHOGONAL_ATTACKS[square as usize]) {
                let in_between = bitboard_helper::get_in_between(square, Square::from_u8(index as u8));

                if in_between & all_mask == 0 {
                    res |= 1_u64 << index;
                }
            }
        }

        return res.count_ones() as u8;
    }

    fn square_is_attacked_by_ignore_king(&self, white: bool, target_square: Square) -> bool {
        let color_mask = if white { self.white_pieces } else { self.black_pieces };

        let pawn_mask = if white { bitboard_helper::BLACK_PAWN_ATTACKS[target_square as usize] } else {  bitboard_helper::WHITE_PAWN_ATTACKS[target_square as usize] };

        if (self.kings & bitboard_helper::KING_ATTACKS[target_square as usize] 
            | self.knights & bitboard_helper::KNIGHT_ATTACKS[target_square as usize] 
            | self.pawns & pawn_mask) & color_mask != 0 {
                return true;
        }
       
        let all_pieces = (self.white_pieces | self.black_pieces) & !(1_u64 << self.get_king_square(!white) as u8);
        
        let diagonal_attackers = self.diagonal_sliders & color_mask & bitboard_helper::DIAGONAL_ATTACKS[target_square as usize];
        if sliders_have_attack(target_square, diagonal_attackers, all_pieces) {
            return true;
        }

        let orthogonal_attackers = self.orthogonal_sliders & color_mask & bitboard_helper::ORTHOGONAL_ATTACKS[target_square as usize];
        if sliders_have_attack(target_square, orthogonal_attackers, all_pieces) {
            return true;
        }
        
        fn sliders_have_attack(target_square: Square, mut sliders: u64, blockers: u64) -> bool {
            for index in bitboard_helper::iterate_set_bits(sliders) {
                let in_between = bitboard_helper::get_in_between(target_square, Square::from_u8(index as u8));

                if blockers & in_between == 0 {
                    return true;
                }
            }

            return false;
        }

        return false;
    }

    pub fn get_piece_type(&self, target_square: Square) -> ColoredPieceType {
        return self.type_field[target_square as usize];
    }
    pub fn get_square_attacker(&self, white: bool, target_square: Square) -> ArrayVec<Square, 16> {
        let color_mask = if white { self.white_pieces } else { self.black_pieces };
        
        let pawn_mask = if white { bitboard_helper::BLACK_PAWN_ATTACKS[target_square as usize] } else {  bitboard_helper::WHITE_PAWN_ATTACKS[target_square as usize] };

        let mut list = ArrayVec::new();

        let res = (self.kings & bitboard_helper::KING_ATTACKS[target_square as usize] 
            | self.knights & bitboard_helper::KNIGHT_ATTACKS[target_square as usize] 
            | self.pawns & pawn_mask) & color_mask;
        
        for index in bitboard_helper::iterate_set_bits(res) {
            list.push(Square::from_u8(index as u8));
        }
        
        let all_pieces = self.white_pieces | self.black_pieces;
        
        let diagonal_attackers = self.diagonal_sliders & color_mask & bitboard_helper::DIAGONAL_ATTACKS[target_square as usize];

        add_slide_attacks(target_square, diagonal_attackers, all_pieces, &mut list); 

        let orthogonal_attackers = self.orthogonal_sliders & color_mask & bitboard_helper::ORTHOGONAL_ATTACKS[target_square as usize];

        add_slide_attacks(target_square, orthogonal_attackers, all_pieces, &mut list); 

        fn add_slide_attacks(target_square: Square, mut sliders: u64, blockers: u64, list: &mut ArrayVec<Square, 16>) {
            for index in bitboard_helper::iterate_set_bits(sliders) {
                let in_between = bitboard_helper::get_in_between(target_square, Square::from_u8(index as u8));

                if blockers & in_between == 0 {
                    list.push(Square::from_u8(index as u8));
                }
            }
        }

        return list;
    }

    fn get_ray_attackers(&self, white: bool, target_square: Square) -> ArrayVec<Square, 16> {
        let color_mask = if white { self.white_pieces } else { self.black_pieces };
        
        let mut list = ArrayVec::new();
        let mut res = self.kings & color_mask & bitboard_helper::KING_ATTACKS[target_square as usize];
        
        let diagonal_attackers = self.diagonal_sliders & color_mask & bitboard_helper::DIAGONAL_ATTACKS[target_square as usize];

        add_slide_attacks(target_square, diagonal_attackers, &mut list); 

        let orthogonal_attackers = self.orthogonal_sliders & color_mask & bitboard_helper::ORTHOGONAL_ATTACKS[target_square as usize];

        add_slide_attacks(target_square, orthogonal_attackers, &mut list); 

        fn add_slide_attacks(target_square: Square, mut sliders: u64, list: &mut ArrayVec<Square, 16>) {
            for index in bitboard_helper::iterate_set_bits(sliders) {
                let in_between = bitboard_helper::get_in_between(target_square, Square::from_u8(index as u8));

                if in_between != 0 {
                    list.push(Square::from_u8(index as u8));
                }
            }
        }

        return list;
    }
    
    pub fn in_check(&self) -> bool {
        return self.square_is_attacked_by(!self.whites_turn, self.get_king_square(self.whites_turn));
    }

    fn in_double_check(&self) -> bool {
        return self.get_square_attacker(!self.whites_turn, self.get_king_square(self.whites_turn)).len() == 2;
    }

        //Diagonal, Orthogonal
    fn get_pin_info(&self, white: bool) -> (u64, u64) {           
        let opposing_pieces = if !white { self.white_pieces } else { self.black_pieces };
        let allied_pieces = if white { self.white_pieces } else { self.black_pieces };
        let all_pieces = opposing_pieces | allied_pieces;
        let king_square = self.get_king_square(white);
        
        let diagonal_attackers = self.diagonal_sliders & opposing_pieces & bitboard_helper::DIAGONAL_ATTACKS[king_square as usize];
        let orthogonal_attackers = self.orthogonal_sliders & opposing_pieces & bitboard_helper::ORTHOGONAL_ATTACKS[king_square as usize];

        let d_res = get_slide_pins(king_square, diagonal_attackers, all_pieces);
        let o_res = get_slide_pins(king_square, orthogonal_attackers, all_pieces);
        return (d_res & allied_pieces, 
             o_res & allied_pieces); 

        fn get_slide_pins(target_square: Square, mut sliders: u64, all_pieces: u64) -> u64 {
            let mut pinned_pieces = 0;
            for index in bitboard_helper::iterate_set_bits(sliders) {

                let in_between = bitboard_helper::get_in_between(target_square, Square::from_u8(index as u8));

                let piece = in_between & all_pieces;
                if piece.count_ones() == 1 {
                    pinned_pieces |= piece;
                }
            }

            return  pinned_pieces;
        }
    }

    fn get_pin_full_pin_info(&self, white: bool) -> (u64, u64, u64, u64) {
        let opposing_pieces = if !white { self.white_pieces } else { self.black_pieces };
        let allied_pieces = if white { self.white_pieces } else { self.black_pieces };
        let all_pieces = opposing_pieces | allied_pieces;
        let king_square = self.get_king_square(white);
        
        //Diagonal
        
        let diagonal_attackers = self.diagonal_sliders & opposing_pieces & bitboard_helper::DIAGONAL_ATTACKS[king_square as usize];
        let mut pos_diagonal = 0;
        let mut neg_diagonal = 0;
        for index in bitboard_helper::iterate_set_bits(diagonal_attackers) {
            
            let square = Square::from_u8(index as u8);
            let in_between = bitboard_helper::get_in_between(king_square, square);
            
            let piece = in_between & all_pieces;
            if piece.count_ones() == 1 {
                //x0 - y0 = x1 - y1
                if square.file() + king_square.rank() == square.rank() + king_square.file() {
                    pos_diagonal = piece;
                }
                else {
                    neg_diagonal = piece;
                }
            }
        }
        
        let orthogonal_attackers = self.orthogonal_sliders & opposing_pieces & bitboard_helper::ORTHOGONAL_ATTACKS[king_square as usize];
        let mut horizontal = 0;
        let mut vertical = 0;
        for index in bitboard_helper::iterate_set_bits(orthogonal_attackers) {
            
            let square = Square::from_u8(index as u8);
            let in_between = bitboard_helper::get_in_between(king_square, square);
            
            let piece = in_between & all_pieces;
            if piece.count_ones() == 1 {
                
                if square.file() != king_square.file() {
                    horizontal = piece;
                }
                else {
                    vertical = piece;
                }
            }
        }

        return (horizontal, vertical, pos_diagonal, neg_diagonal);
    }

    pub fn generate_legal_moves(&self, whites_turn: bool) -> ArrayVec<ChessMove, 200> {
        let attacker_list = self.get_square_attacker(!whites_turn, self.get_king_square(whites_turn));
        
        //Double check
        if attacker_list.len() == 2 {
            //println!("Double check");
            return self.generate_legal_king_moves(whites_turn);
        }
        
        
        if attacker_list.len() == 1 {
            //println!("Single check");
            return self.generate_legal_moves_in_check(attacker_list[0], whites_turn);
        }

        return self.generate_legal_moves_no_check(whites_turn);
        
        //let mut list = self.generate_legal_moves_no_check(whites_turn);
        //let mut fast = self.generate_legal_moves_no_check_fast(whites_turn);
        //
        //if list.len() != fast.len() {
        //    self.print();
        //    
        //    let pair = self.get_pin_full_pin_info(whites_turn);
        //    
        //    println!("hori");
        //    bitboard_helper::print_bitboard(pair.0);
        //    println!("vert");
        //    bitboard_helper::print_bitboard(pair.1);
        //    println!("pos diag");
        //    bitboard_helper::print_bitboard(pair.2);
        //    println!("neg diag");
        //    bitboard_helper::print_bitboard(pair.3);
        //    
        //    
        //    list.sort_by(|a ,b| a.get_board_name(&self).cmp(&b.get_board_name(&self)));
        //    fast.sort_by(|a ,b| a.get_board_name(&self).cmp(&b.get_board_name(&self)));
        //    
        //    for m in list {
        //        print!("{} ", m.get_board_name(&self));
        //    }
        //    
        //    println!();
        //    for m in fast {
        //        print!("{} ", m.get_board_name(&self));
        //    }
        //    println!();
        //    
        //    panic!("Oh no");
        //}
        //
        //return self.generate_legal_moves_no_check_fast(whites_turn);
    }   
    
    //Only king moves
    fn generate_legal_king_moves(&self, whites_turn: bool) -> ArrayVec<ChessMove, 200> {
        let mut list = ArrayVec::new();
        
        let moving_color_mask = if whites_turn { self.white_pieces } else { self.black_pieces };
        
        let king_queen_mask = bitboard_helper::QUEEN_ATTACKS[self.get_king_square(whites_turn) as usize];
        let in_check = self.in_check();

        //King moves
        let move_piece_type = ColoredPieceType::from_pt(PieceType::King, whites_turn);
        let king_square = self.get_king_square(whites_turn);
                
        for target_index in bitboard_helper::iterate_set_bits(
            bitboard_helper::KING_ATTACKS[king_square as usize] & !moving_color_mask) {

            let target_square = Square::from_u8(target_index as u8);

            let target_piece_type = self.type_field[target_square as usize];
            
            if !self.square_is_attacked_by_ignore_king(!whites_turn, target_square) {
                list.push(ChessMove::new_move(king_square, target_square, move_piece_type, target_piece_type))
            }
        }

        return list;
    } 

    fn generate_legal_moves_in_check(&self, attacker_square: Square, whites_turn: bool) -> ArrayVec<ChessMove, 200> {
        let mut list = ArrayVec::new();

        let attacker_type = PieceType::from_cpt(self.type_field[attacker_square as usize]);
        let king_square = self.get_king_square(whites_turn);
        let pin_info = self.get_pin_info(whites_turn);
        let pinned_pieces = pin_info.0 | pin_info.1;

        let mut  possible_move_mask = 0;
        possible_move_mask = 1_u64 << attacker_square as u8;

        if attacker_type.is_slider() {
            possible_move_mask |= bitboard_helper::get_in_between(attacker_square, king_square);
        }

        //Pinned pieces cant block check
        let moving_unpinned_pieces = !pinned_pieces & if whites_turn { self.white_pieces } else { self.black_pieces };
        let opponent_mask = if !whites_turn { self.white_pieces } else { self.black_pieces };
        let all_mask = self.white_pieces | self.black_pieces;
        
        //Pawns
        fn add_pawn_move(start_square: Square, target_square: Square, move_piece_type: ColoredPieceType, target_piece_type: ColoredPieceType, promotion_rank: u8, is_white: bool, list: &mut ArrayVec<ChessMove, 200>) {
            if(target_square.rank() == promotion_rank) {
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Queen,  is_white)));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Rook,   is_white)));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Bishop, is_white)));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Knight, is_white)));
            }
            else {
                list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
            }
        }

        //forward
        let promotion_rank: u8 = if whites_turn { 7 } else { 0 };      
        let pawn_direction: i32 = if whites_turn { 1 } else { -1 };

        let pawns = self.pawns & moving_unpinned_pieces;
        let ep_mask = if self.en_passant_square == Square::None { 0 } else { self.en_passant_square.bit_board() };

        let mut move_piece_type = ColoredPieceType::from_pt(PieceType::Pawn, whites_turn);

        let mut res = !all_mask & bitboard_helper::shift_board(pawns, 0,  pawn_direction);
        
        for index in bitboard_helper::iterate_set_bits(possible_move_mask & res) {
            let target_square = Square::from_u8(index as u8);
            let start_square = Square::from_u8((index as i32 - pawn_direction * 8) as u8);

            add_pawn_move(start_square, target_square, move_piece_type, ColoredPieceType::None, promotion_rank, whites_turn, &mut list);
        }   

        let double_move_mask = bitboard_helper::RANK_MASKS[if whites_turn { 3  } else { 4 }];      
        //double move                                                            only successfull first moves
        res = double_move_mask & possible_move_mask & !all_mask & bitboard_helper::shift_board(res, 0,  pawn_direction);

        for index in bitboard_helper::iterate_set_bits(res) {
            let target_square = Square::from_u8(index as u8);
            let start_square = Square::from_u8((index as i32 - 2 * pawn_direction * 8) as u8);
            list.push(ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None));
        }   

        //attack right
        res = (possible_move_mask & opponent_mask | ep_mask) & bitboard_helper::shift_board(pawns, 1,  pawn_direction);
        for index in bitboard_helper::iterate_set_bits(res) {
            let target_square = Square::from_u8(index as u8);
            let start_square = Square::from_u8((index as i32 - pawn_direction * 8 - 1) as u8);

            if target_square == self.en_passant_square {
                let m = ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None); 
                if self.move_is_legal(m) {
                    list.push(m);
                }
            }
            else {
                add_pawn_move(start_square, target_square, move_piece_type, self.type_field[target_square as usize],promotion_rank, whites_turn, &mut list);
            }
        }

        //attack left
        res = (possible_move_mask & opponent_mask | ep_mask) & bitboard_helper::shift_board(pawns, -1,  pawn_direction);
        for index in bitboard_helper::iterate_set_bits(res) {
            let target_square = Square::from_u8(index as u8);
            let start_square = Square::from_u8((index as i32 - pawn_direction * 8 + 1) as u8);

            if target_square == self.en_passant_square {
                let m = ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None); 
                if self.move_is_legal(m) {
                    list.push(m);
                }
            }
            else {
                add_pawn_move(start_square, target_square, move_piece_type, self.type_field[target_square as usize],promotion_rank, whites_turn, &mut list);
            }
        }

        //en passant


        //Knights
        move_piece_type = ColoredPieceType::from_pt(PieceType::Knight, whites_turn);   
        for start_index in bitboard_helper::iterate_set_bits(self.knights & moving_unpinned_pieces) {
            let start_square = Square::from_u8(start_index as u8);

            for target_index in bitboard_helper::iterate_set_bits(
                bitboard_helper::KNIGHT_ATTACKS[start_square as usize] & possible_move_mask) {

                let target_square = Square::from_u8(target_index as u8);

                let target_piece_type = self.type_field[target_square as usize];
                list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
            }
        }


        //Diagonal attackers
        let diagonal_attackers = self.diagonal_sliders & moving_unpinned_pieces;
        for start_index in bitboard_helper::iterate_set_bits(diagonal_attackers) {
            let intersection = possible_move_mask & bitboard_helper::DIAGONAL_ATTACKS[start_index as usize];
            let start_square = Square::from_u8(start_index as u8);

            for target_index in bitboard_helper::iterate_set_bits(intersection) {
                let target_square = Square::from_u8(target_index as u8);
                let in_between = bitboard_helper::get_in_between(start_square, target_square);
                if  in_between & all_mask == 0 {
                    list.push(ChessMove::new_move(start_square, target_square, 
                        self.type_field[start_square as usize], self.type_field[target_square as usize]));
                }
            }
        }

        let orthogonal_attackers = self.orthogonal_sliders & moving_unpinned_pieces;
        //Orthogonal attackers
        for start_index in bitboard_helper::iterate_set_bits(orthogonal_attackers) {
            let intersection = possible_move_mask & bitboard_helper::ORTHOGONAL_ATTACKS[start_index as usize];
            let start_square = Square::from_u8(start_index as u8);

            for target_index in bitboard_helper::iterate_set_bits(intersection) {
                let target_square = Square::from_u8(target_index as u8);
                let in_between = bitboard_helper::get_in_between(start_square, target_square);
                if  in_between & all_mask == 0 {
                    list.push(ChessMove::new_move(start_square, target_square, 
                        self.type_field[start_square as usize], self.type_field[target_square as usize]));
                }
            }
        }

        for m in self.generate_legal_king_moves(whites_turn) {
            list.push(m);
        }

        return list;
    }

    fn generate_legal_moves_no_check(&self, whites_turn: bool) -> ArrayVec<ChessMove, 200> {
        let mut list = ArrayVec::new();

        let king_square = self.get_king_square(whites_turn);

        let (d_pins, o_pins) = self.get_pin_info(whites_turn);

        //Pinned pieces cant block check
        let moving_color = if whites_turn { self.white_pieces } else { self.black_pieces };
        let opponent_mask = if !whites_turn { self.white_pieces } else { self.black_pieces };
        let all_mask = self.white_pieces | self.black_pieces;
        
        //Pawns
        fn add_pawn_move(start_square: Square, target_square: Square, move_piece_type: ColoredPieceType, target_piece_type: ColoredPieceType, promotion_rank: u8, is_white: bool, list: &mut ArrayVec<ChessMove, 200>) {
            if(target_square.rank() == promotion_rank) {
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Queen,  is_white)));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Rook,   is_white)));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Bishop, is_white)));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Knight, is_white)));
            }
            else {
                list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
            }
        }

        //forward
        let promotion_rank: u8 = if whites_turn { 7 } else { 0 };      
        let double_move_mask = bitboard_helper::RANK_MASKS[if whites_turn { 3  } else { 4 }];      

        let pawn_direction: i32 = if whites_turn { 1 } else { -1 };
        let ep_mask = if self.en_passant_square == Square::None { 0 } else { self.en_passant_square.bit_board() };

        let pawns = self.pawns & moving_color;
        let forward_movable_pawns = pawns & !d_pins & !(o_pins & bitboard_helper::RANK_MASKS[king_square.rank() as usize]);

        //println!("Diagonal pins: ");
        //bitboard_helper::print_bitboard(d_pins);

        let mut move_piece_type = ColoredPieceType::from_pt(PieceType::Pawn, whites_turn);

        let mut res = !all_mask & bitboard_helper::shift_board(forward_movable_pawns, 0,  pawn_direction);
        
        for index in bitboard_helper::iterate_set_bits(res) {
            let target_square = Square::from_u8(index as u8);
            let start_square = Square::from_u8((index as i32 - pawn_direction * 8) as u8);

            add_pawn_move(start_square, target_square, move_piece_type, ColoredPieceType::None, promotion_rank, whites_turn, &mut list);
        }   
        
        //double move                                                            only successfull first moves
        res = double_move_mask & !all_mask & bitboard_helper::shift_board(res, 0,  pawn_direction);

        for index in bitboard_helper::iterate_set_bits(res) {
            let target_square = Square::from_u8(index as u8);
            let start_square = Square::from_u8((index as i32 - 2 * pawn_direction * 8) as u8);

            list.push(ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None));
        }   

        //attack right
        let diagonal_movable_pawns = pawns & !o_pins;

        res = (opponent_mask | ep_mask) & bitboard_helper::shift_board(diagonal_movable_pawns, 1,  pawn_direction);
        for index in bitboard_helper::iterate_set_bits(res) {
            let target_square = Square::from_u8(index as u8);
            let start_square = Square::from_u8((index as i32 - pawn_direction * 8 - 1) as u8);

            if bitboard_helper::get_bit(d_pins, start_square) {
                let intersection = bitboard_helper::DIAGONAL_ATTACKS[start_square as usize] & 
                    bitboard_helper::DIAGONAL_ATTACKS[king_square as usize];

                if !bitboard_helper::get_bit(intersection, target_square) {
                    continue;
                }
            } 

            if target_square == self.en_passant_square {
                let m = ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None); 
                if self.move_is_legal(m) {
                    list.push(m);
                }
            }
            else {
                add_pawn_move(start_square, target_square, move_piece_type, self.type_field[target_square as usize],promotion_rank, whites_turn, &mut list);
            }
        }

        //attack left
        res = (opponent_mask | ep_mask) & bitboard_helper::shift_board(diagonal_movable_pawns, -1,  pawn_direction);
        for index in bitboard_helper::iterate_set_bits(res) {
            let target_square = Square::from_u8(index as u8);
            let start_square = Square::from_u8((index as i32 - pawn_direction * 8 + 1) as u8);

            if bitboard_helper::get_bit(d_pins, start_square) {
                let intersection = bitboard_helper::DIAGONAL_ATTACKS[start_square as usize] & 
                    bitboard_helper::DIAGONAL_ATTACKS[king_square as usize];

                if !bitboard_helper::get_bit(intersection, target_square) {
                    continue;
                }
            } 

            if target_square == self.en_passant_square {
                let m = ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None); 
                if self.move_is_legal(m) {
                    list.push(m);
                }
            }
            else {
                add_pawn_move(start_square, target_square, move_piece_type, self.type_field[target_square as usize],promotion_rank, whites_turn, &mut list);
            }
        }

        //Knights
        move_piece_type = ColoredPieceType::from_pt(PieceType::Knight, whites_turn);   
        for start_index in bitboard_helper::iterate_set_bits(self.knights & moving_color & !(o_pins | d_pins)) {
            let start_square = Square::from_u8(start_index as u8);

            for target_index in bitboard_helper::iterate_set_bits(
                bitboard_helper::KNIGHT_ATTACKS[start_square as usize] & !moving_color) {

                let target_square = Square::from_u8(target_index as u8);

                let target_piece_type = self.type_field[target_square as usize];
                list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
            }
        }

        fn add_slide_moves(start_square: Square, move_piece_type: ColoredPieceType, dx: i32, dy: i32, type_field: [ColoredPieceType; 64], white: bool, list: &mut ArrayVec<ChessMove, 200>) {
            let mut x = start_square.file() as i32 + dx; 
            let mut y = start_square.rank() as i32 + dy;

            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                let target_square = Square::from_u8((x + y * 8) as u8);
                let target_piece_type = type_field[target_square as usize];

                if target_piece_type == ColoredPieceType::None || target_piece_type.is_white() != move_piece_type.is_white() {
                    list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
                }

                if target_piece_type != ColoredPieceType::None {
                    break;
                }

                x += dx;
                y += dy;
            }
        }

        //diagonal moves
        const DIAGONAL_DIRECTIONS: [(i32, i32); 4] = [
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        for start_index in bitboard_helper::iterate_set_bits(self.diagonal_sliders & moving_color & !o_pins) {
            let start_square = Square::from_u8(start_index as u8);
            move_piece_type = self.type_field[start_square as usize];
            
            if bitboard_helper::get_bit(d_pins, start_square) {
                let x = start_square.file() as i32 - king_square.file() as i32;
                let y = start_square.rank() as i32 - king_square.rank() as i32;

                for (dx, dy) in DIAGONAL_DIRECTIONS {
                    //Dot product zero -> vertical to pin direction
                    if dx * x + dy * y == 0 {
                        continue;
                    }

                    add_slide_moves(start_square, move_piece_type, dx, dy, self.type_field, whites_turn, &mut list);
                }
            } 
            else {
                for (dx, dy) in DIAGONAL_DIRECTIONS {
                    add_slide_moves(start_square, move_piece_type, dx, dy, self.type_field, whites_turn, &mut list);
                }
            }
        }

        const ORTHOGONAL_DIRECTIONS: [(i32, i32); 4] = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
        ];

        for start_index in bitboard_helper::iterate_set_bits(self.orthogonal_sliders & moving_color & !d_pins) {
            let start_square = Square::from_u8(start_index as u8);
            move_piece_type = self.type_field[start_square as usize];
            
            if bitboard_helper::get_bit(o_pins, start_square) {
                let x = start_square.file() as i32 - king_square.file() as i32;
                let y = start_square.rank() as i32 - king_square.rank() as i32;

                for (dx, dy) in ORTHOGONAL_DIRECTIONS {
                    //Dot product zero -> vertical to pin direction
                    if dx * x + dy * y == 0 {
                        continue;
                    }

                    add_slide_moves(start_square, move_piece_type, dx, dy, self.type_field, whites_turn, &mut list);
                }
            } 
            else {
                for (dx, dy) in ORTHOGONAL_DIRECTIONS {
                    add_slide_moves(start_square, move_piece_type, dx, dy, self.type_field, whites_turn, &mut list);
                }
            }
        }

        for m in self.generate_legal_king_moves(whites_turn) {
            list.push(m);
        }

        move_piece_type = ColoredPieceType::from_pt(PieceType::King, whites_turn);   

        //Castles
        //not in check
        if whites_turn {
            if self.white_queen_castle {
                if  bitboard_helper::WHITE_QUEEN_CASTLE_MASK & all_mask == 0 && 
                    !self.square_is_attacked_by(!whites_turn, Square::D1) && 
                    !self.square_is_attacked_by(!whites_turn, Square::C1) {
                    list.push(ChessMove::new_move(king_square, Square::C1, move_piece_type, ColoredPieceType::None));
                }
            }

            if self.white_king_castle {
                if  bitboard_helper::WHITE_KING_CASTLE_MASK & all_mask == 0 && 
                    !self.square_is_attacked_by(!whites_turn, Square::F1) && 
                    !self.square_is_attacked_by(!whites_turn, Square::G1) {
                    list.push(ChessMove::new_move(king_square, Square::G1, move_piece_type, ColoredPieceType::None));
                }
            }
        }
        else {
            if self.black_queen_castle {
                if  bitboard_helper::BLACK_QUEEN_CASTLE_MASK & all_mask == 0 && 
                    !self.square_is_attacked_by(!whites_turn, Square::D8) && 
                    !self.square_is_attacked_by(!whites_turn, Square::C8) {
                    list.push(ChessMove::new_move(king_square, Square::C8, move_piece_type, ColoredPieceType::None));
                }
            }

            if self.black_king_castle {
                if  bitboard_helper::BLACK_KING_CASTLE_MASK & all_mask == 0 && 
                    !self.square_is_attacked_by(!whites_turn, Square::F8) && 
                    !self.square_is_attacked_by(!whites_turn, Square::G8) {
                    list.push(ChessMove::new_move(king_square, Square::G8, move_piece_type, ColoredPieceType::None));
                }
            }
        }
        

        return list;
    }

    pub fn generate_legal_moves_eval(&self, whites_turn: bool) -> ArrayVec<ChessMove, 200> {
        let mut list = ArrayVec::new();

        let king_square = self.get_king_square(whites_turn);

        let (d_pins, o_pins) = self.get_pin_info(whites_turn);

        //Pinned pieces cant block check
        let moving_color = if whites_turn { self.white_pieces } else { self.black_pieces };
        let all_mask = self.white_pieces | self.black_pieces;
        
        //Pawns
        fn add_pawn_move(start_square: Square, target_square: Square, move_piece_type: ColoredPieceType, target_piece_type: ColoredPieceType, promotion_rank: u8, is_white: bool, list: &mut ArrayVec<ChessMove, 200>) {
            if(target_square.rank() == promotion_rank) {
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Queen,  is_white)));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Rook,   is_white)));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Bishop, is_white)));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Knight, is_white)));
            }
            else {
                list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
            }
        }

        //forward
        let promotion_rank: u8 = if whites_turn { 7 } else { 0 };      
        let double_move_mask = bitboard_helper::RANK_MASKS[if whites_turn { 3  } else { 4 }];      

        let pawn_direction: i32 = if whites_turn { 1 } else { -1 };
        let ep_mask = if self.en_passant_square == Square::None { 0 } else { self.en_passant_square.bit_board() };

        let pawns = self.pawns & moving_color;
        let forward_movable_pawns = pawns & !d_pins & !(o_pins & bitboard_helper::RANK_MASKS[king_square.rank() as usize]);

        //println!("Diagonal pins: ");
        //bitboard_helper::print_bitboard(d_pins);

        let mut move_piece_type = ColoredPieceType::from_pt(PieceType::Pawn, whites_turn);

        let mut res = !all_mask & bitboard_helper::shift_board(forward_movable_pawns, 0,  pawn_direction);
        
        for index in bitboard_helper::iterate_set_bits(res) {
            let target_square = Square::from_u8(index as u8);
            let start_square = Square::from_u8((index as i32 - pawn_direction * 8) as u8);

            add_pawn_move(start_square, target_square, move_piece_type, ColoredPieceType::None, promotion_rank, whites_turn, &mut list);
        }   
        
        //double move                                                            only successfull first moves
        res = double_move_mask & !all_mask & bitboard_helper::shift_board(res, 0,  pawn_direction);

        for index in bitboard_helper::iterate_set_bits(res) {
            let target_square = Square::from_u8(index as u8);
            let start_square = Square::from_u8((index as i32 - 2 * pawn_direction * 8) as u8);

            list.push(ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None));
        }   

        //ignore capture piece requirement
        //attack right
        let diagonal_movable_pawns = pawns & !o_pins;

        res = bitboard_helper::shift_board(diagonal_movable_pawns, 1,  pawn_direction);
        for index in bitboard_helper::iterate_set_bits(res) {
            let target_square = Square::from_u8(index as u8);
            let start_square = Square::from_u8((index as i32 - pawn_direction * 8 - 1) as u8);

            if bitboard_helper::get_bit(d_pins, start_square) {
                let intersection = bitboard_helper::DIAGONAL_ATTACKS[start_square as usize] & 
                    bitboard_helper::DIAGONAL_ATTACKS[king_square as usize];

                if !bitboard_helper::get_bit(intersection, target_square) {
                    continue;
                }
            } 

            if target_square == self.en_passant_square {
                if !(start_square.rank() == 4 || 
                    start_square.rank() == 3) {
                    continue;
                }

                let m = ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None); 
                if self.move_is_legal(m) {
                    list.push(m);
                }
            }
            else {
                add_pawn_move(start_square, target_square, move_piece_type, self.type_field[target_square as usize],promotion_rank, whites_turn, &mut list);
            }
        }

        //attack left
        res = bitboard_helper::shift_board(diagonal_movable_pawns, -1,  pawn_direction);
        for index in bitboard_helper::iterate_set_bits(res) {
            let target_square = Square::from_u8(index as u8);
            let start_square = Square::from_u8((index as i32 - pawn_direction * 8 + 1) as u8);

            if bitboard_helper::get_bit(d_pins, start_square) {
                let intersection = bitboard_helper::DIAGONAL_ATTACKS[start_square as usize] & 
                    bitboard_helper::DIAGONAL_ATTACKS[king_square as usize];

                if !bitboard_helper::get_bit(intersection, target_square) {
                    continue;
                }
            } 

            if target_square == self.en_passant_square {
                if !(start_square.rank() == 4 || 
                    start_square.rank() == 3) {
                    continue;
                }

                let m = ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None); 
                if self.move_is_legal(m) {
                    list.push(m);
                }
            }
            else {
                add_pawn_move(start_square, target_square, move_piece_type, self.type_field[target_square as usize],promotion_rank, whites_turn, &mut list);
            }
        }

        //Knights
        move_piece_type = ColoredPieceType::from_pt(PieceType::Knight, whites_turn);   
        for start_index in bitboard_helper::iterate_set_bits(self.knights & moving_color & !(o_pins | d_pins)) {
            let start_square = Square::from_u8(start_index as u8);

            for target_index in bitboard_helper::iterate_set_bits(
                bitboard_helper::KNIGHT_ATTACKS[start_square as usize]) {

                let target_square = Square::from_u8(target_index as u8);

                let target_piece_type = self.type_field[target_square as usize];
                list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
            }
        }

        fn add_slide_moves(start_square: Square, move_piece_type: ColoredPieceType, dx: i32, dy: i32, type_field: [ColoredPieceType; 64], white: bool, list: &mut ArrayVec<ChessMove, 200>) {
            let mut x = start_square.file() as i32 + dx; 
            let mut y = start_square.rank() as i32 + dy;

            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                let target_square = Square::from_u8((x + y * 8) as u8);
                let target_piece_type = type_field[target_square as usize];

                list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
                
                if target_piece_type != ColoredPieceType::None && 
                        //Both diagonal sliders
                    !(  (move_piece_type.is_diagonal_slider() && target_piece_type.is_diagonal_slider() 
                        //Both are orthogonal sliders 
                            || move_piece_type.is_orthogonal_slider() && target_piece_type.is_orthogonal_slider()) 
                            && target_piece_type.is_white() == move_piece_type.is_white()
                    ) {
                
                    break;
                }

                x += dx;
                y += dy;
            }
        }

        //diagonal moves
        const DIAGONAL_DIRECTIONS: [(i32, i32); 4] = [
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        for start_index in bitboard_helper::iterate_set_bits(self.diagonal_sliders & moving_color & !o_pins) {
            let start_square = Square::from_u8(start_index as u8);
            move_piece_type = self.type_field[start_square as usize];
            
            if bitboard_helper::get_bit(d_pins, start_square) {
                let x = start_square.file() as i32 - king_square.file() as i32;
                let y = start_square.rank() as i32 - king_square.rank() as i32;

                for (dx, dy) in DIAGONAL_DIRECTIONS {
                    //Dot product zero -> vertical to pin direction
                    if dx * x + dy * y == 0 {
                        continue;
                    }

                    add_slide_moves(start_square, move_piece_type, dx, dy, self.type_field, whites_turn, &mut list);
                }
            } 
            else {
                for (dx, dy) in DIAGONAL_DIRECTIONS {
                    add_slide_moves(start_square, move_piece_type, dx, dy, self.type_field, whites_turn, &mut list);
                }
            }
        }

        const ORTHOGONAL_DIRECTIONS: [(i32, i32); 4] = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
        ];

        for start_index in bitboard_helper::iterate_set_bits(self.orthogonal_sliders & moving_color & !d_pins) {
            let start_square = Square::from_u8(start_index as u8);
            move_piece_type = self.type_field[start_square as usize];
            
            if bitboard_helper::get_bit(o_pins, start_square) {
                let x = start_square.file() as i32 - king_square.file() as i32;
                let y = start_square.rank() as i32 - king_square.rank() as i32;

                for (dx, dy) in ORTHOGONAL_DIRECTIONS {
                    //Dot product zero -> vertical to pin direction
                    if dx * x + dy * y == 0 {
                        continue;
                    }

                    add_slide_moves(start_square, move_piece_type, dx, dy, self.type_field, whites_turn, &mut list);
                }
            } 
            else {
                for (dx, dy) in ORTHOGONAL_DIRECTIONS {
                    add_slide_moves(start_square, move_piece_type, dx, dy, self.type_field, whites_turn, &mut list);
                }
            }
        }

        move_piece_type = ColoredPieceType::from_pt(PieceType::King, whites_turn);   

        let start_square = Square::from_u8(self.get_king_square(whites_turn) as u8);

        for target_index in bitboard_helper::iterate_set_bits(
            bitboard_helper::KING_ATTACKS[start_square as usize]) {

            let target_square = Square::from_u8(target_index as u8);

            let target_piece_type = self.type_field[target_square as usize];
            list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
        }
        
        //Castles
        //not in check
        if whites_turn {
            if self.white_queen_castle {
                if  bitboard_helper::WHITE_QUEEN_CASTLE_MASK & all_mask == 0 && 
                    !self.square_is_attacked_by(!whites_turn, Square::D1) && 
                    !self.square_is_attacked_by(!whites_turn, Square::C1) {
                    list.push(ChessMove::new_move(king_square, Square::C1, move_piece_type, ColoredPieceType::None));
                }
            }

            if self.white_king_castle {
                if  bitboard_helper::WHITE_KING_CASTLE_MASK & all_mask == 0 && 
                    !self.square_is_attacked_by(!whites_turn, Square::F1) && 
                    !self.square_is_attacked_by(!whites_turn, Square::G1) {
                    list.push(ChessMove::new_move(king_square, Square::G1, move_piece_type, ColoredPieceType::None));
                }
            }
        }
        else {
            if self.black_queen_castle {
                if  bitboard_helper::BLACK_QUEEN_CASTLE_MASK & all_mask == 0 && 
                    !self.square_is_attacked_by(!whites_turn, Square::D8) && 
                    !self.square_is_attacked_by(!whites_turn, Square::C8) {
                    list.push(ChessMove::new_move(king_square, Square::C8, move_piece_type, ColoredPieceType::None));
                }
            }

            if self.black_king_castle {
                if  bitboard_helper::BLACK_KING_CASTLE_MASK & all_mask == 0 && 
                    !self.square_is_attacked_by(!whites_turn, Square::F8) && 
                    !self.square_is_attacked_by(!whites_turn, Square::G8) {
                    list.push(ChessMove::new_move(king_square, Square::G8, move_piece_type, ColoredPieceType::None));
                }
            }
        }
        
        return list;
    }

    pub fn get_diagonal_moves(&self, start_square: Square) -> ArrayVec<Square, 16>
    {
        //diagonal moves
        const DIAGONAL_DIRECTIONS: [(i32, i32); 4] = [
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];

        let mut list = ArrayVec::new();
        for (dx, dy) in DIAGONAL_DIRECTIONS {
            let mut x = start_square.file() as i32 + dx; 
            let mut y = start_square.rank() as i32 + dy;

            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                let target_square = Square::from_u8((x + y * 8) as u8);
                let target_piece_type = self.type_field[target_square as usize];

                list.push(target_square);
                
                if target_piece_type != ColoredPieceType::None {
                    break;
                }

                x += dx;
                y += dy;
            }
        }

        return list;
    }

    pub fn get_queen_moves(&self, start_square: Square) -> u64
    {
        //diagonal moves
        const QUEEN_DIRECTIONS: [(i32, i32); 8] = [
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),

            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
        ];

        let mut ret = 0;
        for (dx, dy) in QUEEN_DIRECTIONS {
            let mut x = start_square.file() as i32 + dx; 
            let mut y = start_square.rank() as i32 + dy;

            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                let target_square = Square::from_u8((x + y * 8) as u8);
                let target_piece_type = self.type_field[target_square as usize];

                bitboard_helper::set_bit(&mut ret, target_square, true);
                
                if target_piece_type != ColoredPieceType::None {
                    break;
                }

                x += dx;
                y += dy;
            }
        }

        return ret;
    }

    pub fn make_move(&mut self, m: ChessMove) {
        if m.is_null_move() {
            self.en_passant_square = Square::None;
            self.whites_turn = !self.whites_turn;

            //println!("kek");

            return;
        }

        if m.move_piece_type == ColoredPieceType::WhiteKing {
            self.white_queen_castle = false;
            self.white_king_castle = false;
        }
        
        if m.move_piece_type == ColoredPieceType::BlackKing {
            self.black_queen_castle = false;
            self.black_king_castle = false;
        }
        
        if m.start_square == Square::A1 || m.target_square == Square::A1 {
            self.white_queen_castle = false;
        }
        if m.start_square == Square::H1 || m.target_square == Square::H1 {
            self.white_king_castle = false;
        }
        if m.start_square == Square::A8 || m.target_square == Square::A8 {
            self.black_queen_castle = false;
        }
        if m.start_square == Square::H8 || m.target_square == Square::H8 {
            self.black_king_castle = false;
        }

        //Update en passant square
        let pawn_direction: i32 = if self.whites_turn { 1 } else { -1 };
        if PieceType::from_cpt(m.move_piece_type) == PieceType::Pawn && 
            (m.start_square as u8).abs_diff(m.target_square as u8) == 16 {

            let x = m.target_square.file();
            let y = m.target_square.rank();

            let mut has_neighbour = false;
            if x > 0 {
                if self.type_field[(x - 1 + y * 8) as usize] == m.move_piece_type.get_opposite_color() {
                    has_neighbour = true;
                }
            }

            if x < 7 {
                if self.type_field[(x + 1 + y * 8) as usize] == m.move_piece_type.get_opposite_color() {
                    has_neighbour = true;
                }
            }

            if has_neighbour {
                self.en_passant_square = Square::from_u8((m.target_square as i32 - pawn_direction * 8) as u8);
            }
            else {
                self.en_passant_square = Square::None;
            }
        }
        else {
            self.en_passant_square = Square::None;
        }

        //Moves the rooks
        if m.is_castle() {
            let king_rank = m.start_square.rank();
            
            //left castle
            if m.start_square as u8 > m.target_square as u8 {
                self.move_piece(Square::from_u8(0 + king_rank * 8), Square::from_u8(m.target_square as u8 + 1));
            }
            //right castle
            else {
                self.move_piece(Square::from_u8(7 + king_rank * 8), Square::from_u8(m.target_square as u8 - 1));      
            }
        }

        if m.is_direct_capture() {
            self.capture_piece(m.start_square, m.target_square);
        }
        else {
            self.move_piece(m.start_square, m.target_square);
        }

        if m.is_en_passant() {
            self.remove_piece(Square::from_u8((m.target_square as i32 - pawn_direction * 8) as u8));
        }
        
        if m.is_promotion() {
            self.remove_piece(m.target_square);
            self.place_piece(m.promotion_piece_type, m.target_square);
        }

        self.whites_turn = !self.whites_turn;
    }

    //Does not check castle move square and start square
    pub fn move_is_legal(&self, m: ChessMove) -> bool {
        let mut res = true;
        let mut buffer = self.clone();

        buffer.make_move(m);

        let mut king_square = buffer.get_king_square(self.whites_turn);

        return !buffer.square_is_attacked_by(!self.whites_turn, king_square);   
        
        //let list = buffer.get_square_attacker(!self.whites_turn, king_square);
        //return list.len() == 0;
        
    }

    pub fn get_legal_moves(&self) -> ArrayVec<ChessMove, 200> {
        return self.generate_legal_moves(self.whites_turn);        
    }

    pub fn print_type_field(type_field: &[ColoredPieceType; 64]) {
        const PIECE_CHAR: [char; 13] = ['P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k', ' '];
        println!("   {}", String::from_utf8(vec![b'_'; 16]).unwrap());

        for y in (0..8).rev() {
            print!("{} |", y + 1);
            for x in 0..8 {
                let p = type_field[x + y * 8];
                
                print!("{} ", PIECE_CHAR[p as usize]);
                
            }
            println!("|");
        }

        println!("   {}", String::from_utf8(vec![b'-'; 16]).unwrap());
        println!("   a b c d e f g h");
    }
    pub fn print(&self) {
        //"https://lichess.org/editor/r6r/2pk1pp1/4P3/p6p/P1bp4/2q2NQP/2P3P1/2BK3R_b_-_-_0_1
        println!("https://lichess.org/editor/{}", self.get_fen().replace(" ", "_"));

        Self::print_type_field(&self.type_field);

        if self.white_king_castle || self.white_queen_castle || self.black_king_castle || self.black_queen_castle {
            //KQkq

            if self.white_king_castle {
                print!("K");
            }
            if self.white_queen_castle {
                print!("Q");
            }
            if self.black_king_castle {
                print!("k");
            }
            if self.black_queen_castle {
                print!("q");
            }

            println!();
        }
        else{
            println!("No castle rights");
        }
        println!("{}", if self.whites_turn { "White to move" } else { "Black to move" });
        if self.en_passant_square != Square::None {
            print!("Ep: ", );
            self.en_passant_square.print();
            println!();
        }

        println!("Hash: {}", self.get_zoberist_hash());

        println!();
    }

    pub fn print_bitboards(&self) {
        println!("White pieces: ");
        bitboard_helper::print_bitboard(self.white_pieces);

        println!("Black pieces: ");
        bitboard_helper::print_bitboard(self.black_pieces);

        println!("Pawns: ");
        bitboard_helper::print_bitboard(self.pawns);

        println!("Knights: ");
        bitboard_helper::print_bitboard(self.knights);

        println!("Diagonal sliders: ");
        bitboard_helper::print_bitboard(self.diagonal_sliders);

        println!("Orthogonal sliders: ");
        bitboard_helper::print_bitboard(self.orthogonal_sliders);

        println!("Kings: ");
        bitboard_helper::print_bitboard(self.kings);
    }

    pub fn print_moves(list: &ArrayVec<ChessMove, 200>){
        print!("Moves {}[", list.len());
    
        for m in list {
            m.print();  
            print!(" ");      
        }
    
        println!("]");
    }

    pub fn print_local_moves(&self, list: &ArrayVec<ChessMove, 200>) {
        print!("Moves {}[", list.len());
    
        for m in list {
            print!("{} ", m.get_board_name(self));  
        }
    
        println!("]");
    }
}