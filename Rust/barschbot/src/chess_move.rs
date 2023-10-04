use crate::constants;
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
        return ChessMove { start_square, target_square, move_piece_type, target_piece_type, promotion_piece_type: constants::EMPTY, is_en_passant: false }
    }

    pub fn new_pawn_move(start_square: u8, target_square: u8, move_piece_type: u8, target_piece_type: u8, promotion_piece_type: u8, is_en_passant: bool) -> Self {
        return ChessMove { start_square, target_square, move_piece_type, target_piece_type, promotion_piece_type, is_en_passant }
    }

    pub fn is_castle(&self) -> bool {
        return self.move_piece_type >> 1 == constants::KING && self.start_square.abs_diff(self.target_square) == 2 
    }

    pub fn is_capture(&self) -> bool {
        return self.target_piece_type != constants::EMPTY;
    }

    pub fn is_promotion(&self) -> bool {
        return self.promotion_piece_type != constants::EMPTY;
    }

    pub fn is_white_move(&self) -> bool {
        return self.move_piece_type & 1 == 0;
    }

    pub fn print_uci(&self) {
        print!("{}{}", constants::SQUARE_NAME[self.start_square as usize], constants::SQUARE_NAME[self.target_square as usize]);
        
        if self.is_promotion() {
            print!("{}", match (self.promotion_piece_type >> 1) {
                constants::KNIGHT => "n",
                constants::BISHOP => "b",
                constants::ROOK => "r",
                _ => "q",
            });
        }
    }

    pub fn get_uci(&self) -> String {
        let mut x = constants::SQUARE_NAME[self.start_square as usize].to_owned() + constants::SQUARE_NAME[self.target_square as usize];
        
        if self.is_promotion() {
            x += match (self.promotion_piece_type >> 1) {
                constants::KNIGHT => "n",
                constants::BISHOP => "b",
                constants::ROOK => "r",
                _ => "q",
            };
        }

        return x;
    }

    pub fn print(&self) {
        if self.is_castle() {
            if (self.target_square < self.start_square) {
                print!("O-O-O");
            }
            else {
                print!("O-O");
            }
            return;
        }

        print!("{}-", constants::PIECE_CHAR[self.move_piece_type as usize]);
        Self::print_square(self.start_square);
        if self.target_piece_type != constants::EMPTY {
            print!("x{}-", constants::PIECE_CHAR[self.target_piece_type as usize]);
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
