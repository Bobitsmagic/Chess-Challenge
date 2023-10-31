use graphics::types::Color;

use crate::piece_type::PieceType;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColoredPieceType {
    WhitePawn, BlackPawn, 
    WhiteKnight, BlackKnight,
    WhiteBishop, BlackBishop, 
    WhiteRook, BlackRook,
    WhiteQueen, BlackQueen,
    WhiteKing, BlackKing,
    None,
}

const PIECE_CHAR: [char; 21] = ['P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k', ' ', '1', '2', '3','4','5','6','7','8'];

impl ColoredPieceType {
    pub fn from_char(char: char) -> ColoredPieceType {
        return match char {
            'P' => ColoredPieceType::WhitePawn,
            'p' => ColoredPieceType::BlackPawn,
    
            'N' => ColoredPieceType::WhiteKnight,
            'n' => ColoredPieceType::BlackKnight,
    
            'B' => ColoredPieceType::WhiteBishop,
            'b' => ColoredPieceType::BlackBishop,
    
            'R' => ColoredPieceType::WhiteRook,
            'r' => ColoredPieceType::BlackRook,
    
            'Q' => ColoredPieceType::WhiteQueen,
            'q' => ColoredPieceType::BlackQueen,
    
            'K' => ColoredPieceType::WhiteKing,
            'k' => ColoredPieceType::BlackKing,
    
            _ => ColoredPieceType::None
        };
    }
    
    pub fn from_pt(pt: PieceType, white: bool) -> ColoredPieceType {
        return match (pt, white) {
            (PieceType::Pawn, true) => ColoredPieceType::WhitePawn,
            (PieceType::Pawn, false) => ColoredPieceType::BlackPawn,

            (PieceType::Knight, true) => ColoredPieceType::WhiteKnight,
            (PieceType::Knight, false) => ColoredPieceType::BlackKnight,

            (PieceType::Bishop, true) => ColoredPieceType::WhiteBishop,
            (PieceType::Bishop, false) => ColoredPieceType::BlackBishop,

            (PieceType::Rook, true) => ColoredPieceType::WhiteRook,
            (PieceType::Rook, false) => ColoredPieceType::BlackRook,

            (PieceType::Queen, true) => ColoredPieceType::WhiteQueen,
            (PieceType::Queen, false) => ColoredPieceType::BlackQueen,

            (PieceType::King, true) => ColoredPieceType::WhiteKing,
            (PieceType::King, false) => ColoredPieceType::BlackKing,

            _ => ColoredPieceType::None
        }
    }

    pub fn from_u8(value: u8) -> ColoredPieceType {
        const ARRAY: [ColoredPieceType; 13] = [
            ColoredPieceType::WhitePawn, 
            ColoredPieceType::BlackPawn, 

            ColoredPieceType::WhiteKnight, 
            ColoredPieceType::BlackKnight, 

            ColoredPieceType::WhiteBishop, 
            ColoredPieceType::BlackBishop, 

            ColoredPieceType::WhiteRook, 
            ColoredPieceType::BlackRook, 

            ColoredPieceType::WhiteQueen, 
            ColoredPieceType::BlackQueen, 

            ColoredPieceType::WhiteKing, 
            ColoredPieceType::BlackKing, 

            ColoredPieceType::None,
        ];

        return ARRAY[value as usize];
    }

    pub fn is_white(&self) -> bool {
        return (*self as u8) & 1 == 0;
    }

    pub fn get_char(&self) -> &char {
        return &PIECE_CHAR[*self as usize];
    }
}
