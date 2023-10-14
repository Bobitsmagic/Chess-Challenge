use std::char;
use arrayvec::ArrayVec;

use crate::{bitboard_helper::{self, toggle_bit}, chess_move::ChessMove, square::Square, colored_piece_type::{ColoredPieceType, self}, piece_type::PieceType};



#[derive(Clone, Copy)]
pub struct BitBoard {
    whites_turn: bool,

    white_king_castle: bool,
    black_king_castle: bool,
    white_queen_castle: bool,
    black_queen_castle: bool,

    en_passant_square: Square,

    white_pieces: u64,
    black_pieces: u64,
    pawns: u64,
    knights: u64,
    orthogonal_sliders: u64,
    diagonal_sliders: u64,
    kings: u64,

    type_field: [ColoredPieceType; 64]
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

    pub fn from_fen(fen: &str) -> Self {
        let parts = fen.split(" ").collect::<Vec<_>>();
        let mut board = BitBoard::empty();

        let mut square = 64 - 8;
        for c in parts[0].chars() {
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

        println!("Loaded FEN {}", fen);

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

        if colored_piece_type.is_white_piece() {
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
        
        if self.kings & color_mask & bitboard_helper::KING_ATTACKS[target_square as usize] != 0 {
            return true;
        }

        if self.knights & color_mask & bitboard_helper::KNIGHT_ATTACKS[target_square as usize] != 0 {
            return true;
        }
   
        let pawn_mask = if white { bitboard_helper::BLACK_PAWN_ATTACKS[target_square as usize] } else {  bitboard_helper::WHITE_PAWN_ATTACKS[target_square as usize] };
        if self.pawns & color_mask & pawn_mask != 0 {
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
                let in_between = bitboard_helper::IN_BETWEEN_SQUARES[(target_square as u32 + index * 64) as usize];

                if blockers & in_between == 0 {
                    return true;
                }
            }

            return false;
        }

        return false;
    }

    fn square_is_attacked_by_ignore_king(&self, white: bool, target_square: Square) -> bool {
        let color_mask = if white { self.white_pieces } else { self.black_pieces };
        
        if self.kings & color_mask & bitboard_helper::KING_ATTACKS[target_square as usize] != 0 {
            return true;
        }

        if self.knights & color_mask & bitboard_helper::KNIGHT_ATTACKS[target_square as usize] != 0 {
            return true;
        }
   
        let pawn_mask = if white { bitboard_helper::BLACK_PAWN_ATTACKS[target_square as usize] } else {  bitboard_helper::WHITE_PAWN_ATTACKS[target_square as usize] };
        if self.pawns & color_mask & pawn_mask != 0 {
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
                let in_between = bitboard_helper::IN_BETWEEN_SQUARES[(target_square as u32 + index * 64) as usize];

                if blockers & in_between == 0 {
                    return true;
                }
            }

            return false;
        }

        return false;
    }

    fn get_square_attacker(&self, white: bool, target_square: Square) -> ArrayVec<Square, 16> {
        let color_mask = if white { self.white_pieces } else { self.black_pieces };
        
        let mut list = ArrayVec::new();
        let mut res = self.kings & color_mask & bitboard_helper::KING_ATTACKS[target_square as usize];
        if res != 0 {
            list.push(Square::from_u8(res.trailing_zeros() as u8));
        }
        
        res = self.knights & color_mask & bitboard_helper::KNIGHT_ATTACKS[target_square as usize];
        for index in bitboard_helper::iterate_set_bits(res) {
            list.push(Square::from_u8(index as u8));
        }
        
        let pawn_mask = if white {bitboard_helper::BLACK_PAWN_ATTACKS[target_square as usize] } else { bitboard_helper::WHITE_PAWN_ATTACKS[target_square as usize] };
        res = self.pawns & color_mask & pawn_mask;
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
                let in_between = bitboard_helper::IN_BETWEEN_SQUARES[(target_square as u32 + index * 64) as usize];

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
                let in_between = bitboard_helper::IN_BETWEEN_SQUARES[(target_square as u32 + index * 64) as usize];

                if in_between != 0 {
                    list.push(Square::from_u8(index as u8));
                }
            }
        }

        return list;
    }
    
    fn in_check(&self) -> bool {
        return self.square_is_attacked_by(!self.whites_turn, self.get_king_square(self.whites_turn));
    }

    fn in_double_check(&self) -> bool {
        return self.get_square_attacker(!self.whites_turn, self.get_king_square(self.whites_turn)).len() == 2;
    }

    fn get_pseudo_legal_moves(&self) -> ArrayVec<ChessMove, 200> {
        let mut list = ArrayVec::new();
        let moving_color_mask = if self.whites_turn { self.white_pieces } else { self.black_pieces };
        let opponent_mask = if !self.whites_turn { self.white_pieces } else { self.black_pieces };
        let all_mask = self.white_pieces | self.black_pieces;
        
        //Pawns
        fn add_pawn_move(start_square: Square, target_square: Square, move_piece_type: ColoredPieceType, target_piece_type: ColoredPieceType, promotion_rank: u8, is_white: bool, list: &mut ArrayVec<ChessMove, 200>) {
            if(target_square.rank() == promotion_rank) {
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Knight, is_white)));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Bishop, is_white)));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Rook,   is_white)));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Queen,  is_white)));
            }
            else {
                list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
            }
        }

        let pawn_direction: i32 = if self.whites_turn { 1 } else { -1 };
        let start_rank: u8 = if self.whites_turn { 1 } else { 6 };
        let promotion_rank: u8 = if self.whites_turn { 7 } else { 0 };      
        let mut move_piece_type = ColoredPieceType::from_pt(PieceType::Pawn, self.whites_turn);

        //[TODO] Use bit boards for move checking/generating
        for index in bitboard_helper::iterate_set_bits(self.pawns & moving_color_mask) {
            let start_square = Square::from_u8(index as u8);
            let x = index % 8;
            let y = index / 8;

            let mut target_square = Square::from_u8((start_square as i32 + 8 * pawn_direction) as u8);

            //forward move 
            if  self.type_field[target_square as usize] == ColoredPieceType::None {
                
                add_pawn_move(start_square, target_square, move_piece_type, ColoredPieceType::None, promotion_rank, self.whites_turn, &mut list);
                
                if start_square.rank() == start_rank {
                    target_square = Square::from_u8((start_square as i32 + 2 * 8 * pawn_direction) as u8);
                    if self.type_field[target_square as usize] == ColoredPieceType::None {
                        list.push(ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None));
                    }
                }
            }

            //capture left
            if x > 0 {
                target_square = Square::from_u8((start_square as i32 + 8 * pawn_direction - 1) as u8);
                let target_piece_type = self.type_field[target_square as usize];
                if  target_piece_type != ColoredPieceType::None  &&  target_piece_type.is_white_piece() != self.whites_turn {
                    add_pawn_move(start_square, target_square, move_piece_type, target_piece_type, promotion_rank, self.whites_turn, &mut list);
                }

                if target_square == self.en_passant_square {
                    list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, ColoredPieceType::None, ColoredPieceType::None));
                }
            }

            //capture right
            if x < 7 {
                target_square = Square::from_u8((start_square as i32 + 8 * pawn_direction + 1) as u8);
                let target_piece_type = self.type_field[target_square as usize];
                if  target_piece_type != ColoredPieceType::None &&  target_piece_type.is_white_piece() != self.whites_turn {
                    add_pawn_move(start_square, target_square, move_piece_type, target_piece_type, promotion_rank, self.whites_turn, &mut list);
                }

                if target_square == self.en_passant_square {
                    list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, ColoredPieceType::None, ColoredPieceType::None));
                }
            }
        }

        //Knight moves
        move_piece_type = ColoredPieceType::from_pt(PieceType::Knight, self.whites_turn);;
        
        for start_index in bitboard_helper::iterate_set_bits(self.knights & moving_color_mask) {
            let start_square = Square::from_u8(start_index as u8);

            for target_index in bitboard_helper::iterate_set_bits(
                bitboard_helper::KNIGHT_ATTACKS[start_square as usize] & !moving_color_mask) {

                let target_square = Square::from_u8(target_index as u8);

                let target_piece_type = self.type_field[target_square as usize];

                list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type))
            }
        }

        fn add_slide_moves(start_square: Square, move_piece_type: ColoredPieceType, dx: i32, dy: i32, type_field: [ColoredPieceType; 64], white: bool, list: &mut ArrayVec<ChessMove, 200>) {
            let mut x = start_square.file() as i32 + dx; 
            let mut y = start_square.rank() as i32 + dy;

            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                let target_square = Square::from_u8((x + y * 8) as u8);
                let target_piece_type = type_field[target_square as usize];

                if target_piece_type == ColoredPieceType::None || target_piece_type.is_white_piece() != move_piece_type.is_white_piece() {
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

        for start_index in bitboard_helper::iterate_set_bits(self.diagonal_sliders & moving_color_mask) {
            let start_square = Square::from_u8(start_index as u8);
            move_piece_type = self.type_field[start_square as usize];

            for (dx, dy) in DIAGONAL_DIRECTIONS {
                add_slide_moves(start_square, move_piece_type, dx, dy, self.type_field, self.whites_turn, &mut list);
            }
        }

        const ORTHOGONAL_DIRECTIONS: [(i32, i32); 4] = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
        ];

        for start_index in bitboard_helper::iterate_set_bits(self.orthogonal_sliders & moving_color_mask) {
            let start_square = Square::from_u8(start_index as u8);
            move_piece_type = self.type_field[start_square as usize];

            for (dx, dy) in ORTHOGONAL_DIRECTIONS {
                add_slide_moves(start_square, move_piece_type, dx, dy, self.type_field, self.whites_turn, &mut list);
            }
        }

        //King moves
        move_piece_type = ColoredPieceType::from_pt(PieceType::King, self.whites_turn);
        let king_square = self.get_king_square(self.whites_turn);
               
        for target_index in bitboard_helper::iterate_set_bits(
            bitboard_helper::KING_ATTACKS[king_square as usize] & !moving_color_mask) {

            let target_square = Square::from_u8(target_index as u8);

            let target_piece_type = self.type_field[target_square as usize];

            list.push(ChessMove::new_move(king_square, target_square, move_piece_type, target_piece_type))
        }
    
        if !self.in_check() {
            if self.whites_turn {
                if self.white_queen_castle {
                    if  bitboard_helper::WHITE_QUEEN_CASTLE_MASK & all_mask == 0 && 
                        !self.square_is_attacked_by(!self.whites_turn, Square::D1) {
                        list.push(ChessMove::new_move(king_square, Square::C1, move_piece_type, ColoredPieceType::None));
                    }
                }
    
                if self.white_king_castle {
                    if  bitboard_helper::WHITE_KING_CASTLE_MASK & all_mask == 0 && 
                        !self.square_is_attacked_by(!self.whites_turn, Square::F1) {
                        list.push(ChessMove::new_move(king_square, Square::G1, move_piece_type, ColoredPieceType::None));
                    }
                }
            }
            else {
                if self.black_queen_castle {
                    if  bitboard_helper::BLACK_QUEEN_CASTLE_MASK & all_mask == 0 && 
                        !self.square_is_attacked_by(!self.whites_turn, Square::D8) {
                        list.push(ChessMove::new_move(king_square, Square::C8, move_piece_type, ColoredPieceType::None));
                    }
                }
    
                if self.black_king_castle {
                    if  bitboard_helper::BLACK_KING_CASTLE_MASK & all_mask == 0 && 
                        !self.square_is_attacked_by(!self.whites_turn, Square::F8) {
                        list.push(ChessMove::new_move(king_square, Square::G8, move_piece_type, ColoredPieceType::None));
                    }
                }
            }
        }
               
        return list;
    }

    fn generate_legal_moves(&self) -> ArrayVec<ChessMove, 200> {
        let attacker_list = self.get_square_attacker(!self.whites_turn, self.get_king_square(self.whites_turn));
        
        //Double check
        if attacker_list.len() == 2 {
            return self.generate_legal_moves_in_double_check();
        }
        
        let king_square = self.get_king_square(self.whites_turn);

        let mut  possible_move_mask = 0;
        if attacker_list.len() == 1 {
            let attacker_square = attacker_list[0];
            let attacker_type = PieceType::from_cpt(self.type_field[attacker_square as usize]);
            
            possible_move_mask = 1_u64 << attacker_square as u8;
            if attacker_type.is_slider() {
                possible_move_mask |= bitboard_helper::IN_BETWEEN_SQUARES[(king_square as u32 + attacker_square as u32 * 64) as usize];
            }
        }
        else {
            possible_move_mask = u64::MAX;
        }

        //println!("PMM");
        //bitboard_helper::print_bitboard(possible_move_mask);

        let mut list = ArrayVec::new();
        let moving_color_mask = if self.whites_turn { self.white_pieces } else { self.black_pieces };
        let opponent_mask = if !self.whites_turn { self.white_pieces } else { self.black_pieces };
        let all_mask = self.white_pieces | self.black_pieces;

        //For pins
        let ray_list = self.get_ray_attackers(!self.whites_turn, king_square);

        //println!("Ray count: {}", ray_list.len());

        //pieces that could be pinned
        let mut pin_mask = 0;
        for s in ray_list {
            pin_mask |= bitboard_helper::IN_BETWEEN_SQUARES[(king_square as u32 + s as u32 * 64) as usize];
        }

        //println!("Pin mask: ");
        //bitboard_helper::print_bitboard(pin_mask);

                
        pub fn check_pins(m: ChessMove, pin_mask: u64, board: &BitBoard) -> bool {
            if bitboard_helper::get_bit(pin_mask, m.start_square) {
                return board.move_is_legal(m);                
            }
            else {
                return true
            }
        }
        
        //Pawns
        fn add_pawn_move(start_square: Square, target_square: Square, move_piece_type: ColoredPieceType, target_piece_type: ColoredPieceType, promotion_rank: u8, is_white: bool, pin_mask: u64, board: &BitBoard, list: &mut ArrayVec<ChessMove, 200>) {

            if(target_square.rank() == promotion_rank) {
                let m = ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Queen,  is_white));
                if check_pins(m, pin_mask, board) {
                    list.push(m);
                    
                    list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Rook,   is_white)));
                    list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Bishop, is_white)));
                    list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, ColoredPieceType::from_pt(PieceType::Knight, is_white)));
                }
            }
            else {
                let m = ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type);
                if check_pins(m, pin_mask, board) {
                    list.push(m);
                }
            }
        }


        let pawn_direction: i32 = if self.whites_turn { 1 } else { -1 };
        let start_rank: u8 = if self.whites_turn { 1 } else { 6 };
        let promotion_rank: u8 = if self.whites_turn { 7 } else { 0 };      
        let mut move_piece_type = ColoredPieceType::from_pt(PieceType::Pawn, self.whites_turn);

        //[TODO] Use bit boards for move checking/generating
        for index in bitboard_helper::iterate_set_bits(self.pawns & moving_color_mask) {
            let start_square = Square::from_u8(index as u8);
            let x = index % 8;
            let y = index / 8;

            let mut target_square = Square::from_u8((start_square as i32 + 8 * pawn_direction) as u8);

            //forward move 
            if  self.type_field[target_square as usize] == ColoredPieceType::None {
                
                if bitboard_helper::get_bit(possible_move_mask, target_square) {
                    add_pawn_move(start_square, target_square, move_piece_type, ColoredPieceType::None, promotion_rank, self.whites_turn, pin_mask, self,&mut list);
                }
                
                if start_square.rank() == start_rank {
                    target_square = Square::from_u8((start_square as i32 + 2 * 8 * pawn_direction) as u8);
                    if self.type_field[target_square as usize] == ColoredPieceType::None 
                            && bitboard_helper::get_bit(possible_move_mask, target_square) {
                        let m = ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None);
                        if check_pins(m, pin_mask, self) {
                            list.push(m);
                        }
                    }
                }
            }

            //capture left
            if x > 0 {
                target_square = Square::from_u8((start_square as i32 + 8 * pawn_direction - 1) as u8);

                if  bitboard_helper::get_bit(possible_move_mask, target_square) {
                    let target_piece_type = self.type_field[target_square as usize];
                    if  (target_piece_type != ColoredPieceType::None  &&  target_piece_type.is_white_piece() != self.whites_turn) {
                        
                        add_pawn_move(start_square, target_square, move_piece_type, target_piece_type, promotion_rank, self.whites_turn, pin_mask, self,&mut list);
                    }
                }

                if target_square == self.en_passant_square && bitboard_helper::get_bit(possible_move_mask, Square::from_u8((start_square as i32 - 1) as u8)) {
                    let m = ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None);

                    if check_pins(m, pin_mask, self) {
                        list.push(m);
                    }
                }
            }

            //capture right
            if x < 7 {
                target_square = Square::from_u8((start_square as i32 + 8 * pawn_direction + 1) as u8);

                if  bitboard_helper::get_bit(possible_move_mask, target_square) {
                    let target_piece_type = self.type_field[target_square as usize];
                    if  (target_piece_type != ColoredPieceType::None  &&  target_piece_type.is_white_piece() != self.whites_turn) {
                        
                        add_pawn_move(start_square, target_square, move_piece_type, target_piece_type, promotion_rank, self.whites_turn, pin_mask, self,&mut list);
                    }
                }

                if target_square == self.en_passant_square && bitboard_helper::get_bit(possible_move_mask, Square::from_u8((start_square as i32 + 1) as u8)) {
                    let m = ChessMove::new_move(start_square, target_square, move_piece_type, ColoredPieceType::None);

                    if check_pins(m, pin_mask, self) {
                        list.push(m);
                    }
                }
            }
        }

        //Knight moves
        move_piece_type = ColoredPieceType::from_pt(PieceType::Knight, self.whites_turn);   
        for start_index in bitboard_helper::iterate_set_bits(self.knights & moving_color_mask) {
            let start_square = Square::from_u8(start_index as u8);

            for target_index in bitboard_helper::iterate_set_bits(
                bitboard_helper::KNIGHT_ATTACKS[start_square as usize] & !moving_color_mask & possible_move_mask) {

                let target_square = Square::from_u8(target_index as u8);

                let target_piece_type = self.type_field[target_square as usize];
                let m = ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type);
                if check_pins(m, pin_mask, self) {
                    list.push(m);
                }
            }
        }

        fn add_slide_moves(start_square: Square, move_piece_type: ColoredPieceType, dx: i32, dy: i32, type_field: [ColoredPieceType; 64], white: bool, pin_mask: u64, board: &BitBoard, possible_move_mask: u64, list: &mut ArrayVec<ChessMove, 200>) {
            let mut x = start_square.file() as i32 + dx; 
            let mut y = start_square.rank() as i32 + dy;

            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                let target_square = Square::from_u8((x + y * 8) as u8);
                let target_piece_type = type_field[target_square as usize];

                if (target_piece_type == ColoredPieceType::None || target_piece_type.is_white_piece() != move_piece_type.is_white_piece()) 
                    && bitboard_helper::get_bit(possible_move_mask, target_square) {
                    
                    let m = ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type);
                    if check_pins(m, pin_mask, board){
                        list.push(m);
                    }
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

        for start_index in bitboard_helper::iterate_set_bits(self.diagonal_sliders & moving_color_mask) {
            let start_square = Square::from_u8(start_index as u8);
            move_piece_type = self.type_field[start_square as usize];

            for (dx, dy) in DIAGONAL_DIRECTIONS {
                add_slide_moves(start_square, move_piece_type, dx, dy, self.type_field, self.whites_turn, pin_mask, self, possible_move_mask, &mut list);
            }
        }

        const ORTHOGONAL_DIRECTIONS: [(i32, i32); 4] = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
        ];

        for start_index in bitboard_helper::iterate_set_bits(self.orthogonal_sliders & moving_color_mask) {
            let start_square = Square::from_u8(start_index as u8);
            move_piece_type = self.type_field[start_square as usize];

            for (dx, dy) in ORTHOGONAL_DIRECTIONS {
                add_slide_moves(start_square, move_piece_type, dx, dy, self.type_field, self.whites_turn, pin_mask, self, possible_move_mask, &mut list);
            }
        }

        let move_piece_type = ColoredPieceType::from_pt(PieceType::King, self.whites_turn);
        for target_index in bitboard_helper::iterate_set_bits(
            bitboard_helper::KING_ATTACKS[king_square as usize] & !moving_color_mask) {

            let target_square = Square::from_u8(target_index as u8);

            let target_piece_type = self.type_field[target_square as usize];
            
            if !self.square_is_attacked_by_ignore_king(!self.whites_turn, target_square) {
                list.push(ChessMove::new_move(king_square, target_square, move_piece_type, target_piece_type))
            }
        }

        //not in check
        if attacker_list.len() == 0 {
            if self.whites_turn {
                if self.white_queen_castle {
                    if  bitboard_helper::WHITE_QUEEN_CASTLE_MASK & all_mask == 0 && 
                        !self.square_is_attacked_by(!self.whites_turn, Square::D1) && 
                        !self.square_is_attacked_by(!self.whites_turn, Square::C1) {
                        list.push(ChessMove::new_move(king_square, Square::C1, move_piece_type, ColoredPieceType::None));
                    }
                }
    
                if self.white_king_castle {
                    if  bitboard_helper::WHITE_KING_CASTLE_MASK & all_mask == 0 && 
                        !self.square_is_attacked_by(!self.whites_turn, Square::F1) && 
                        !self.square_is_attacked_by(!self.whites_turn, Square::G1) {
                        list.push(ChessMove::new_move(king_square, Square::G1, move_piece_type, ColoredPieceType::None));
                    }
                }
            }
            else {
                if self.black_queen_castle {
                    if  bitboard_helper::BLACK_QUEEN_CASTLE_MASK & all_mask == 0 && 
                        !self.square_is_attacked_by(!self.whites_turn, Square::D8) && 
                        !self.square_is_attacked_by(!self.whites_turn, Square::C8) {
                        list.push(ChessMove::new_move(king_square, Square::C8, move_piece_type, ColoredPieceType::None));
                    }
                }
    
                if self.black_king_castle {
                    if  bitboard_helper::BLACK_KING_CASTLE_MASK & all_mask == 0 && 
                        !self.square_is_attacked_by(!self.whites_turn, Square::F8) && 
                        !self.square_is_attacked_by(!self.whites_turn, Square::G8) {
                        list.push(ChessMove::new_move(king_square, Square::G8, move_piece_type, ColoredPieceType::None));
                    }
                }
            }
        }
               
        return list;
    }   

    //Only king moves
    fn generate_legal_moves_in_double_check(&self) -> ArrayVec<ChessMove, 200> {
        let mut list = ArrayVec::new();

        let moving_color_mask = if self.whites_turn { self.white_pieces } else { self.black_pieces };
        
        let king_queen_mask = bitboard_helper::QUEEN_ATTACKS[self.get_king_square(self.whites_turn) as usize];
        let in_check = self.in_check();

        //King moves
        let move_piece_type = ColoredPieceType::from_pt(PieceType::King, self.whites_turn);
        let king_square = self.get_king_square(self.whites_turn);
                
        for target_index in bitboard_helper::iterate_set_bits(
            bitboard_helper::KING_ATTACKS[king_square as usize] & !moving_color_mask) {

            let target_square = Square::from_u8(target_index as u8);

            let target_piece_type = self.type_field[target_square as usize];
            
            if !self.square_is_attacked_by_ignore_king(!self.whites_turn, target_square) {
                list.push(ChessMove::new_move(king_square, target_square, move_piece_type, target_piece_type))
            }
        }

        return list;
    } 

    pub fn make_move(&mut self, m: ChessMove) {
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
            self.en_passant_square = Square::from_u8((m.target_square as i32 - pawn_direction * 8) as u8);
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
        let mut gen_list = self.generate_legal_moves();
        return gen_list;

        let mut list = self.get_pseudo_legal_moves();

        //println!("Pseudo legal moves: ");
        //Self::print_moves(&list);

        
        
        let mut remove: Vec<usize> = Vec::new();
        
        for i in 0..list.len() {
            let m = list[i];
            
            if !self.move_is_legal(m) {
                remove.push(i);
            }
        }
        
        for i in (0..remove.len()).rev() {
            let index = remove[i];
            
            list.remove(index);
        }
        
        if list.len() != gen_list.len() {
            self.print();
            println!("Correct: ");
            Self::print_moves(&list);

            println!("Move gen: ");
            Self::print_moves(&gen_list);
        }

        return list;
    }
    
    pub fn print(&self) {
        println!("Fen: {}", self.get_fen());

        const PIECE_CHAR: [char; 13] = ['P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k', ' '];
        println!("   {}", String::from_utf8(vec![b'_'; 16]).unwrap());

        for y in (0..8).rev() {
            print!("{} |", y + 1);
            for x in 0..8 {
                let p = self.type_field[x + y * 8];
                
                print!("{} ", PIECE_CHAR[p as usize]);
                
            }
            println!("|");
        }

        println!("   {}", String::from_utf8(vec![b'-'; 16]).unwrap());
        println!("   a b c d e f g h");

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
}