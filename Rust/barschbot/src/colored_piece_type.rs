use graphics::types::Color;

use crate::piece_type::PieceType;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    pub fn get_opposite_color(&self) -> ColoredPieceType {
        return match *self {
            ColoredPieceType::WhitePawn => ColoredPieceType::BlackPawn,
            ColoredPieceType::BlackPawn => ColoredPieceType::WhitePawn,

            ColoredPieceType::WhiteKnight => ColoredPieceType::BlackKnight,
            ColoredPieceType::BlackKnight => ColoredPieceType::WhiteKnight,
            
            ColoredPieceType::WhiteBishop => ColoredPieceType::BlackBishop,
            ColoredPieceType::BlackBishop => ColoredPieceType::WhiteBishop,

            ColoredPieceType::WhiteRook => ColoredPieceType::BlackRook,
            ColoredPieceType::BlackRook => ColoredPieceType::WhiteRook,

            ColoredPieceType::WhiteQueen => ColoredPieceType::BlackQueen,
            ColoredPieceType::BlackQueen => ColoredPieceType::WhiteQueen,

            ColoredPieceType::WhiteKing => ColoredPieceType::BlackKing,
            ColoredPieceType::BlackKing => ColoredPieceType::WhiteKing,

            ColoredPieceType::None => ColoredPieceType::None,
        }
    }

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

    pub fn is_slider(&self) -> bool {
        return PieceType::from_cpt(*self).is_slider();
    }

    pub fn is_orthogonal_slider(&self) -> bool {
        return PieceType::from_cpt(*self).is_orthogonal_slider();
    }

    pub fn is_diagonal_slider(&self) -> bool {
        return PieceType::from_cpt(*self).is_diagonal_slider();
    }


    pub fn is_pawn(&self) -> bool {
        return PieceType::from_cpt(*self) == PieceType::Pawn;
    }
    pub fn is_knight(&self) -> bool {
        return PieceType::from_cpt(*self) == PieceType::Knight;
    }
    pub fn is_bishop(&self) -> bool {
        return PieceType::from_cpt(*self) == PieceType::Bishop;
    }
    pub fn is_rook(&self) -> bool {
        return PieceType::from_cpt(*self) == PieceType::Rook;
    }
    pub fn is_queen(&self) -> bool {
        return PieceType::from_cpt(*self) == PieceType::Queen;
    }
    pub fn is_king(&self) -> bool {
        return PieceType::from_cpt(*self) == PieceType::King;
    }

    pub fn get_char(&self) -> &char {
        return &PIECE_CHAR[*self as usize];
    }
}
