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

pub const NULL_PIECE: u8 = 12;

pub const PIECE_CHAR: [char; 21] = ['P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k', ' ', '1', '2', '3','4','5','6','7','8'];

pub const MAX_MOVE_COUNT: usize = 200;

pub const A1: u8 = 0;
pub const B1: u8 = 1;
pub const C1: u8 = 2;
pub const D1: u8 = 3;
pub const E1: u8 = 4;
pub const F1: u8 = 5;
pub const G1: u8 = 6;
pub const H1: u8 = 7;

pub const A2: u8 = 8;
pub const B2: u8 = 9;
pub const C2: u8 = 10;
pub const D2: u8 = 11;
pub const E2: u8 = 12;
pub const F2: u8 = 13;
pub const G2: u8 = 14;
pub const H2: u8 = 15;

pub const A3: u8 = 16;
pub const B3: u8 = 17;
pub const C3: u8 = 18;
pub const D3: u8 = 19;
pub const E3: u8 = 20;
pub const F3: u8 = 21;
pub const G3: u8 = 22;
pub const H3: u8 = 23;

pub const A4: u8 = 24;
pub const B4: u8 = 25;
pub const C4: u8 = 26;
pub const D4: u8 = 27;
pub const E4: u8 = 28;
pub const F4: u8 = 29;
pub const G4: u8 = 30;
pub const H4: u8 = 31;

pub const A5: u8 = 32;
pub const B5: u8 = 33;
pub const C5: u8 = 34;
pub const D5: u8 = 35;
pub const E5: u8 = 36;
pub const F5: u8 = 37;
pub const G5: u8 = 38;
pub const H5: u8 = 39;

pub const A6: u8 = 40;
pub const B6: u8 = 41;
pub const C6: u8 = 42;
pub const D6: u8 = 43;
pub const E6: u8 = 44;
pub const F6: u8 = 45;
pub const G6: u8 = 46;
pub const H6: u8 = 47;

pub const A7: u8 = 48;
pub const B7: u8 = 49;
pub const C7: u8 = 50;
pub const D7: u8 = 51;
pub const E7: u8 = 52;
pub const F7: u8 = 53;
pub const G7: u8 = 54;
pub const H7: u8 = 55;

pub const A8: u8 = 56;
pub const B8: u8 = 57;
pub const C8: u8 = 58;
pub const D8: u8 = 59;
pub const E8: u8 = 60;
pub const F8: u8 = 61;
pub const G8: u8 = 62;
pub const H8: u8 = 63;

pub const NO_SQUARE: u8 = 255;

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

pub const KNIGHT_MOVES: [&[u8]; 64] = [
    &[10, 17],
    &[11, 16, 18],
    &[8, 12, 17, 19],
    &[9, 13, 18, 20],
    &[10, 14, 19, 21],
    &[11, 15, 20, 22],
    &[12, 21, 23],
    &[13, 22],
    &[2, 18, 25],
    &[3, 19, 24, 26],
    &[0, 16, 4, 20, 25, 27],
    &[1, 17, 5, 21, 26, 28],
    &[2, 18, 6, 22, 27, 29],
    &[3, 19, 7, 23, 28, 30],
    &[4, 20, 29, 31],
    &[5, 21, 30],
    &[10, 26, 1, 33],
    &[11, 27, 0, 32, 2, 34],
    &[8, 24, 12, 28, 1, 33, 3, 35],
    &[9, 25, 13, 29, 2, 34, 4, 36],
    &[10, 26, 14, 30, 3, 35, 5, 37],
    &[11, 27, 15, 31, 4, 36, 6, 38],
    &[12, 28, 5, 37, 7, 39],
    &[13, 29, 6, 38],
    &[18, 34, 9, 41],
    &[19, 35, 8, 40, 10, 42],
    &[16, 32, 20, 36, 9, 41, 11, 43],
    &[17, 33, 21, 37, 10, 42, 12, 44],
    &[18, 34, 22, 38, 11, 43, 13, 45],
    &[19, 35, 23, 39, 12, 44, 14, 46],
    &[20, 36, 13, 45, 15, 47],
    &[21, 37, 14, 46],
    &[26, 42, 17, 49],
    &[27, 43, 16, 48, 18, 50],
    &[24, 40, 28, 44, 17, 49, 19, 51],
    &[25, 41, 29, 45, 18, 50, 20, 52],
    &[26, 42, 30, 46, 19, 51, 21, 53],
    &[27, 43, 31, 47, 20, 52, 22, 54],
    &[28, 44, 21, 53, 23, 55],
    &[29, 45, 22, 54],
    &[34, 50, 25, 57],
    &[35, 51, 24, 56, 26, 58],
    &[32, 48, 36, 52, 25, 57, 27, 59],
    &[33, 49, 37, 53, 26, 58, 28, 60],
    &[34, 50, 38, 54, 27, 59, 29, 61],
    &[35, 51, 39, 55, 28, 60, 30, 62],
    &[36, 52, 29, 61, 31, 63],
    &[37, 53, 30, 62],
    &[42, 58, 33],
    &[43, 59, 32, 34],
    &[40, 56, 44, 60, 33, 35],
    &[41, 57, 45, 61, 34, 36],
    &[42, 58, 46, 62, 35, 37],
    &[43, 59, 47, 63, 36, 38],
    &[44, 60, 37, 39],
    &[45, 61, 38],
    &[50, 41],
    &[51, 40, 42],
    &[48, 52, 41, 43],
    &[49, 53, 42, 44],
    &[50, 54, 43, 45],
    &[51, 55, 44, 46],
    &[52, 45, 47],
    &[53, 46]
];
pub const KING_MOVES: [&[u8]; 64] = [
    &[9, 8, 1],
    &[8, 10, 9, 2, 0],
    &[9, 11, 10, 3, 1],
    &[10, 12, 11, 4, 2],
    &[11, 13, 12, 5, 3],
    &[12, 14, 13, 6, 4],
    &[13, 15, 14, 7, 5],
    &[14, 15, 6],
    &[1, 17, 16, 0, 9],
    &[0, 16, 2, 18, 17, 1, 10, 8],
    &[1, 17, 3, 19, 18, 2, 11, 9],
    &[2, 18, 4, 20, 19, 3, 12, 10],
    &[3, 19, 5, 21, 20, 4, 13, 11],
    &[4, 20, 6, 22, 21, 5, 14, 12],
    &[5, 21, 7, 23, 22, 6, 15, 13],
    &[6, 22, 23, 7, 14],
    &[9, 25, 24, 8, 17],
    &[8, 24, 10, 26, 25, 9, 18, 16],
    &[9, 25, 11, 27, 26, 10, 19, 17],
    &[10, 26, 12, 28, 27, 11, 20, 18],
    &[11, 27, 13, 29, 28, 12, 21, 19],
    &[12, 28, 14, 30, 29, 13, 22, 20],
    &[13, 29, 15, 31, 30, 14, 23, 21],
    &[14, 30, 31, 15, 22],
    &[17, 33, 32, 16, 25],
    &[16, 32, 18, 34, 33, 17, 26, 24],
    &[17, 33, 19, 35, 34, 18, 27, 25],
    &[18, 34, 20, 36, 35, 19, 28, 26],
    &[19, 35, 21, 37, 36, 20, 29, 27],
    &[20, 36, 22, 38, 37, 21, 30, 28],
    &[21, 37, 23, 39, 38, 22, 31, 29],
    &[22, 38, 39, 23, 30],
    &[25, 41, 40, 24, 33],
    &[24, 40, 26, 42, 41, 25, 34, 32],
    &[25, 41, 27, 43, 42, 26, 35, 33],
    &[26, 42, 28, 44, 43, 27, 36, 34],
    &[27, 43, 29, 45, 44, 28, 37, 35],
    &[28, 44, 30, 46, 45, 29, 38, 36],
    &[29, 45, 31, 47, 46, 30, 39, 37],
    &[30, 46, 47, 31, 38],
    &[33, 49, 48, 32, 41],
    &[32, 48, 34, 50, 49, 33, 42, 40],
    &[33, 49, 35, 51, 50, 34, 43, 41],
    &[34, 50, 36, 52, 51, 35, 44, 42],
    &[35, 51, 37, 53, 52, 36, 45, 43],
    &[36, 52, 38, 54, 53, 37, 46, 44],
    &[37, 53, 39, 55, 54, 38, 47, 45],
    &[38, 54, 55, 39, 46],
    &[41, 57, 56, 40, 49],
    &[40, 56, 42, 58, 57, 41, 50, 48],
    &[41, 57, 43, 59, 58, 42, 51, 49],
    &[42, 58, 44, 60, 59, 43, 52, 50],
    &[43, 59, 45, 61, 60, 44, 53, 51],
    &[44, 60, 46, 62, 61, 45, 54, 52],
    &[45, 61, 47, 63, 62, 46, 55, 53],
    &[46, 62, 63, 47, 54],
    &[49, 48, 57],
    &[48, 50, 49, 58, 56],
    &[49, 51, 50, 59, 57],
    &[50, 52, 51, 60, 58],
    &[51, 53, 52, 61, 59],
    &[52, 54, 53, 62, 60],
    &[53, 55, 54, 63, 61],
    &[54, 55, 62]
];