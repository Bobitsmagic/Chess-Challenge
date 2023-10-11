use crate::piece_type::PieceType;

#[derive(Clone, Copy, PartialEq)]
pub enum ColoredPieceType {
    WhitePawn, BlackPawn, 
    WhiteKnight, BlackKnight,
    WhiteBishop, BlackBishop, 
    WhiteRook, BlackRook,
    WhiteQueen, BlackQueen,
    WhiteKing, BlackKing,
    None,
}

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
            (PieceType::Pawn, false) => ColoredPieceType::WhitePawn,

            (PieceType::Knight, true) => ColoredPieceType::WhiteKnight,
            (PieceType::Knight, false) => ColoredPieceType::WhiteKnight,

            (PieceType::Bishop, true) => ColoredPieceType::WhiteBishop,
            (PieceType::Bishop, false) => ColoredPieceType::WhiteBishop,

            (PieceType::Rook, true) => ColoredPieceType::WhiteRook,
            (PieceType::Rook, false) => ColoredPieceType::WhiteRook,

            (PieceType::Queen, true) => ColoredPieceType::WhiteQueen,
            (PieceType::Queen, false) => ColoredPieceType::WhiteQueen,

            (PieceType::King, true) => ColoredPieceType::WhiteKing,
            (PieceType::King, false) => ColoredPieceType::WhiteKing,

            _ => ColoredPieceType::None
        }
    }

    pub fn is_white_piece(&self) -> bool {
        return (*self as u8) & 1 == 0;
    }
}
