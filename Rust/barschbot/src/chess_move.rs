use crate::{constants, bit_board, square::Square, colored_piece_type::ColoredPieceType, piece_type::PieceType};
use std::num;

#[derive(Copy, Clone)]
pub struct ChessMove {
    pub start_square: Square,
    pub target_square: Square,
    pub move_piece_type: ColoredPieceType,
    pub capture_piece_type: ColoredPieceType,
    
    pub promotion_piece_type: ColoredPieceType,
}

pub const NULL_MOVE: ChessMove = ChessMove { start_square: Square::None, target_square: Square::None, move_piece_type: ColoredPieceType::None, capture_piece_type: ColoredPieceType::None, promotion_piece_type: ColoredPieceType::None };

impl ChessMove {
    pub fn new_move(start_square: Square, target_square: Square, move_piece_type: ColoredPieceType, target_piece_type: ColoredPieceType) -> Self {
        return ChessMove { start_square, target_square, move_piece_type, capture_piece_type: target_piece_type, promotion_piece_type: ColoredPieceType::None }
    }

    pub fn new_pawn_move(start_square: Square, target_square: Square, move_piece_type: ColoredPieceType, target_piece_type: ColoredPieceType, promotion_piece_type: ColoredPieceType) -> Self {
        return ChessMove { start_square, target_square, move_piece_type, capture_piece_type: target_piece_type, promotion_piece_type }
    }

    pub fn is_castle(&self) -> bool {
        return PieceType::from_cpt(self.move_piece_type) == PieceType::King && (self.start_square as u8).abs_diff(self.target_square as u8) == 2 
    }

    pub fn is_direct_capture(&self) -> bool {
        return self.capture_piece_type != ColoredPieceType::None;
    }

    pub fn is_promotion(&self) -> bool {
        return self.promotion_piece_type != ColoredPieceType::None;
    }

    pub fn is_white_move(&self) -> bool {
        return self.move_piece_type.is_white_piece();
    }

    pub fn is_en_passant(&self) -> bool {
        return PieceType::from_cpt(self.move_piece_type) == PieceType::Pawn
            && (self.start_square as u8).abs_diff(self.target_square as u8) % 8 != 0 
            && self.is_direct_capture();        
    }

    pub fn print_uci(&self) {
        print!("{}{}", constants::SQUARE_NAME[self.start_square as usize], constants::SQUARE_NAME[self.target_square as usize]);
        
        if self.is_promotion() {
            print!("{}", match (PieceType::from_cpt(self.promotion_piece_type)) {
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
        if self.capture_piece_type != constants::NULL_PIECE {
            print!("x{}-", constants::PIECE_CHAR[self.capture_piece_type as usize]);
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
