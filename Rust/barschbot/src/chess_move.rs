use crate::piece_type;
use std::num;

#[derive(Copy, Clone)]
pub struct ChessMove {
    pub start_square: u8,
    pub target_square: u8,
    pub move_piece_type: u8,
    pub target_piece_type: u8,
    
    pub promotion_piece_type: u8,
    pub is_en_passant: bool,
}

impl ChessMove {
    pub fn new_move(start_square: u8, target_square: u8, move_piece_type: u8, target_piece_type: u8) -> Self {
        return ChessMove { start_square, target_square, move_piece_type, target_piece_type, promotion_piece_type: piece_type::EMPTY, is_en_passant: false }
    }

    pub fn new_pawn_move(start_square: u8, target_square: u8, move_piece_type: u8, target_piece_type: u8, promotion_piece_type: u8, is_en_passant: bool) -> Self {
        return ChessMove { start_square, target_square, move_piece_type, target_piece_type, promotion_piece_type, is_en_passant }
    }

    pub fn is_castle(&self) -> bool {
        return self.move_piece_type == piece_type::KING && self.start_square.abs_diff(self.target_square) == 2 
    }

    pub fn is_capture(&self) -> bool {
        return self.target_piece_type != piece_type::EMPTY;
    }

    pub fn is_promotion(&self) -> bool {
        return self.promotion_piece_type != piece_type::EMPTY;
    }

    pub fn print(&self) {
        const PIECE_CHAR: [char; 13] = ['P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k', ' '];
        print!("{}-", PIECE_CHAR[self.move_piece_type as usize]);
        Self::print_square(self.start_square);
        if self.target_piece_type != piece_type::EMPTY {
            print!("x{}-", PIECE_CHAR[self.target_piece_type as usize]);
        }
        Self::print_square(self.target_square);
        if self.is_en_passant {
            print!("!!");
        }
    }

    fn print_square(index: u8) {
        const COLUMN_CHAR: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        let x = index % 8;
        let y = index / 8;

        print!("{}{}", COLUMN_CHAR[x as usize], y + 1);
    }
}
