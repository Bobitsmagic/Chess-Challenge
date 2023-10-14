use crate::colored_piece_type::ColoredPieceType;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn, Knight, Bishop, Rook, Queen, King, None
}

impl PieceType {
    pub fn from_cpt(colored_piece_type: ColoredPieceType) -> PieceType {
        return match colored_piece_type {
            ColoredPieceType::WhitePawn   | ColoredPieceType::BlackPawn  => PieceType::Pawn,
            ColoredPieceType::WhiteKnight | ColoredPieceType::BlackKnight => PieceType::Knight,
            ColoredPieceType::WhiteBishop | ColoredPieceType::BlackBishop => PieceType::Bishop,
            ColoredPieceType::WhiteRook   | ColoredPieceType::BlackRook => PieceType::Rook,
            ColoredPieceType::WhiteQueen  | ColoredPieceType::BlackQueen => PieceType::Queen,
            ColoredPieceType::WhiteKing   | ColoredPieceType::BlackKing => PieceType::King,
            ColoredPieceType::None => PieceType::None
        };
    }

    pub fn is_slider(&self) -> bool {
        return match *self {
            PieceType::Bishop => true,
            PieceType::Rook => true,
            PieceType::Queen => true,
            _ => false
        }
    }
}
