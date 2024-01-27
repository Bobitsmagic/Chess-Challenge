use crate::{constants, bit_board::{self, BitBoard}, square::Square, colored_piece_type::ColoredPieceType, piece_type::PieceType};
use std::num;

#[derive(Copy, Clone, PartialEq, Eq)]
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

    pub fn new_uci_move(uci: &str) -> ChessMove {
        return ChessMove { start_square: Square::from_str(&uci[0..2]), target_square: Square::from_str(&uci[2..4]), 
            move_piece_type: ColoredPieceType::None, capture_piece_type: ColoredPieceType::None,
            promotion_piece_type: if uci.len() > 4 { ColoredPieceType::from_char(uci.chars().skip(4).next().unwrap()) } else  { ColoredPieceType::None } }
    }

    pub fn is_castle(&self) -> bool {
        return PieceType::from_cpt(self.move_piece_type) == PieceType::King && (self.start_square as u8).abs_diff(self.target_square as u8) == 2 
    }

    pub fn is_capture(&self) -> bool {
        return self.is_direct_capture() || self.is_en_passant();
    }

    pub fn is_direct_capture(&self) -> bool {
        return self.capture_piece_type != ColoredPieceType::None;
    }

    pub fn is_null_move(&self) -> bool {
        return *self == NULL_MOVE;
    }

    pub fn is_attack(&self) -> bool {
        //Not (vertical pawn move || castle move)
        return !((PieceType::from_cpt(self.move_piece_type) == PieceType::Pawn && self.start_square.file() == self.target_square.file()) ||
        (PieceType::from_cpt(self.move_piece_type) == PieceType::King && (self.start_square as u8).abs_diff(self.target_square as u8) == 2));
    }

    pub fn is_defence(&self) -> bool {
        if self.is_direct_capture() {
            return self.move_piece_type.is_white() != self.capture_piece_type.is_white();
        }

        return false;
    }

    //EP are not valid rn !!!!!
    pub fn is_valid(&self) -> bool {
        return !self.is_defence() && !(self.is_attack() && self.move_piece_type.is_pawn() && self.capture_piece_type != ColoredPieceType::None);
    }

    pub fn is_promotion(&self) -> bool {
        return self.promotion_piece_type != ColoredPieceType::None;
    }

    pub fn is_white_move(&self) -> bool {
        return self.move_piece_type.is_white();
    }

    pub fn is_en_passant(&self) -> bool {
        return PieceType::from_cpt(self.move_piece_type) == PieceType::Pawn
            && (self.start_square as u8).abs_diff(self.target_square as u8) % 8 != 0 
            && !self.is_direct_capture();        
    }

    pub fn get_uci(&self) -> String {
        if self.is_null_move() {
            return "NULL_MOVE".to_owned();
        }

        let mut x = constants::SQUARE_NAME[self.start_square as usize].to_owned() + constants::SQUARE_NAME[self.target_square as usize];
        
        if self.is_promotion() {
            x += match (PieceType::from_cpt(self.promotion_piece_type)) {
                PieceType::Knight => "n",
                PieceType::Bishop => "b",
                PieceType::Rook => "r",
                _ => "q",
            };
        }

        return x;
    }

    pub fn get_board_name(&self, board: &BitBoard) -> String {
        if self.is_castle() {
            if (self.target_square as u8) < (self.start_square as u8) {
                return "O-O-O".to_owned();
            }
            else {
                return "O-O".to_owned();
            }
        }

        let mut s = "".to_owned();

        if PieceType::from_cpt(self.move_piece_type) == PieceType::Pawn {
            if self.is_direct_capture() || self.is_en_passant() {
                s += &self.start_square.file_char().to_string();
            }
        }
        else {
            s += &PieceType::from_cpt(self.move_piece_type).get_char().to_string();
        }
        

        if self.is_direct_capture() || self.is_en_passant() {
            s += "x";
        }

        s += &self.target_square.to_string();

        return s;
    }

    pub fn print(&self) {
        if self.is_castle() {
            if (self.target_square as u8) < (self.start_square as u8) {
                print!("O-O-O");
            }
            else {
                print!("O-O");
            }
            return;
        }

        print!("{}-", self.move_piece_type.get_char());
        self.start_square.print();
        if self.is_direct_capture() {
            print!("x{}-", self.capture_piece_type.get_char());
        }
        self.target_square.print();
        if self.is_en_passant() {
            print!("!");
        }

        if self.is_promotion() {
            print!(" > {}", self.promotion_piece_type.get_char());
        }
    }
}
