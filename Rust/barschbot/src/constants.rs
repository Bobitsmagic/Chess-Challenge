pub const PAWN:     u8 = 0;
pub const KNIGHT:   u8 = 1;
pub const BISHOP:   u8 = 2;
pub const ROOK:     u8 = 3;
pub const QUEEN:    u8 = 4;
pub const KING:     u8 = 5;

//pub const NONE:     u8 = 6;

pub const WHITE_PAWN:   u8 = 0;
pub const BLACK_PAWN:   u8 = 1;

pub const WHITE_KNIGHT: u8 = 2;
pub const BLACK_KNIGHT: u8 = 3;

pub const WHITE_BISHOP: u8 = 4;
pub const BLACK_BISHOP: u8 = 5;

pub const WHITE_ROOK:   u8 = 6;
pub const BLACK_ROOK:   u8 = 7;

pub const WHITE_QUEEN:  u8 = 8;
pub const BLACK_QUEEN:  u8 = 9;

pub const WHITE_KING:   u8 = 10;
pub const BLACK_KING:   u8 = 11;

pub const EMPTY: u8 = 12;

pub const PIECE_CHAR: [char; 21] = ['P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k', ' ', '1', '2', '3','4','5','6','7','8'];

pub const SQUARE_NAME: [&str; 64] = [
    "a1",  "b1", "c1", "d1", "e1", "f1", "g1", "h1", 
    "a2",  "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3",  "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4",  "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5",  "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6",  "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7",  "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8",  "b8", "c8", "d8", "e8", "f8", "g8", "h8"
    ];