use std::char;
use arrayvec::ArrayVec;

use crate::{bitboard_helper::{self, toggle_bit}, chess_move::ChessMove};

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn, Knight, Bishop, Rook, Queen, King, None
}

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

#[derive(Clone, Copy, PartialEq)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1, 
    A2, B2, C2, D2, E2, F2, G2, H2, 
    A3, B3, C3, D3, E3, F3, G3, H3, 
    A4, B4, C4, D4, E4, F4, G4, H4, 
    A5, B5, C5, D5, E5, F5, G5, H5, 
    A6, B6, C6, D6, E6, F6, G6, H6, 
    A7, B7, C7, D7, E7, F7, G7, H7, 
    A8, B8, C8, D8, E8, F8, G8, H8, 
    None,
}

pub fn piece_type_from_cpt(colored_piece_type: ColoredPieceType) -> PieceType {
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

pub fn piece_type_from_char(char: char) -> ColoredPieceType {
    return match char {
        'P' => ColoredPieceType::WhitePawn,
        'p' => ColoredPieceType::BlackPawn,

        'K' => ColoredPieceType::WhitePawn,
        'k' => ColoredPieceType::BlackPawn,

        'B' => ColoredPieceType::WhitePawn,
        'b' => ColoredPieceType::BlackPawn,

        'R' => ColoredPieceType::WhitePawn,
        'r' => ColoredPieceType::BlackPawn,

        'Q' => ColoredPieceType::WhitePawn,
        'q' => ColoredPieceType::BlackPawn,

        'K' => ColoredPieceType::WhitePawn,
        'k' => ColoredPieceType::BlackPawn,

        _ => ColoredPieceType::None
    };
}

pub fn is_white_piece(colored_piece_type: ColoredPieceType) -> bool {
    return (colored_piece_type as u8) & 1 == 0;
}

pub fn square_from_str(str: &str) -> Square {
    return match str {
        "a1" => Square::A1,
        "b1" => Square::B1,
        "c1" => Square::C1,
        "d1" => Square::D1,
        "e1" => Square::E1,
        "f1" => Square::F1,
        "g1" => Square::G1,
        "h1" => Square::H1,

        "a2" => Square::A2,
        "b2" => Square::B2,
        "c2" => Square::C2,
        "d2" => Square::D2,
        "e2" => Square::E2,
        "f2" => Square::F2,
        "g2" => Square::G2,
        "h2" => Square::H2,

        "a3" => Square::A3,
        "b3" => Square::B3,
        "c3" => Square::C3,
        "d3" => Square::D3,
        "e3" => Square::E3,
        "f3" => Square::F3,
        "g3" => Square::G3,
        "h3" => Square::H3,

        "a4" => Square::A4,
        "b4" => Square::B4,
        "c4" => Square::C4,
        "d4" => Square::D4,
        "e4" => Square::E4,
        "f4" => Square::F4,
        "g4" => Square::G4,
        "h4" => Square::H4,

        "a5" => Square::A5,
        "b5" => Square::B5,
        "c5" => Square::C5,
        "d5" => Square::D5,
        "e5" => Square::E5,
        "f5" => Square::F5,
        "g5" => Square::G5,
        "h5" => Square::H5,

        "a6" => Square::A6,
        "b6" => Square::B6,
        "c6" => Square::C6,
        "d6" => Square::D6,
        "e6" => Square::E6,
        "f6" => Square::F6,
        "g6" => Square::G6,
        "h6" => Square::H6,
        
        "a7" => Square::A7,
        "b7" => Square::B7,
        "c7" => Square::C7,
        "d7" => Square::D7,
        "e7" => Square::E7,
        "f7" => Square::F7,
        "g7" => Square::G7,
        "h7" => Square::H7,

        "a8" => Square::A8,
        "b8" => Square::B8,
        "c8" => Square::C8,
        "d8" => Square::D8,
        "e8" => Square::E8,
        "f8" => Square::F8,
        "g8" => Square::G8,
        "h8" => Square::H8,

        _ => panic!("Tried to parse weird square")
    }
}

pub fn square_from_u8(index: u8) -> Square {
    return match index {
        0  => Square::A1,
        1  => Square::B1,
        2  => Square::C1,
        3  => Square::D1,
        4  => Square::E1,
        5  => Square::F1,
        6  => Square::G1,
        7  => Square::H1,

        8  => Square::A2,
        9  => Square::B2,
        10 => Square::C2,
        11 => Square::D2,
        12 => Square::E2,
        13 => Square::F2,
        14 => Square::G2,
        15 => Square::H2,

        16 => Square::A3,
        17 => Square::B3,
        18 => Square::C3,
        19 => Square::D3,
        20 => Square::E3,
        21 => Square::F3,
        22 => Square::G3,
        23 => Square::H3,

        24 => Square::A4,
        25 => Square::B4,
        26 => Square::C4,
        27 => Square::D4,
        28 => Square::E4,
        29 => Square::F4,
        30 => Square::G4,
        31 => Square::H4,

        32 => Square::A5,
        33 => Square::B5,
        34 => Square::C5,
        35 => Square::D5,
        36 => Square::E5,
        37 => Square::F5,
        38 => Square::G5,
        39 => Square::H5,

        40 => Square::A6,
        41 => Square::B6,
        42 => Square::C6,
        43 => Square::D6,
        44 => Square::E6,
        45 => Square::F6,
        46 => Square::G6,
        47 => Square::H6,
        
        48 => Square::A7,
        49 => Square::B7,
        50 => Square::C7,
        51 => Square::D7,
        52 => Square::E7,
        53 => Square::F7,
        54 => Square::G7,
        55 => Square::H7,

        56 => Square::A8,
        57 => Square::B8,
        58 => Square::C8,
        59 => Square::D8,
        60 => Square::E8,
        61 => Square::F8,
        62 => Square::G8,
        63 => Square::H8,

        _ => panic!("Tried to parse weird square")
    }
}

struct BitBoard {
    whites_turn: bool,

    white_king_castle: bool,
    black_king_castle: bool,
    white_queen_castle: bool,
    black_queen_castle: bool,

    en_passant_square: Square,

    white_pieces: u64,
    black_pieces: u64,
    pawns: u64,
    knights: u64,
    orthogonal_sliders: u64,
    diagonal_sliders: u64,
    kings: u64,

    type_field: [ColoredPieceType; 64]
}

impl BitBoard {
    pub fn empty() -> Self {
        return BitBoard { whites_turn: true, white_queen_castle: false, white_king_castle: false, black_queen_castle: false, black_king_castle: false,
            en_passant_square: Square::None, 
            white_pieces: 0, black_pieces: 0, pawns: 0, knights: 0, orthogonal_sliders: 0, diagonal_sliders: 0, kings: 0, type_field: [ColoredPieceType::None; 64] };
    }

    pub fn start_position() -> Self {
        return Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq");
    }

    pub fn from_fen(fen: &str) -> Self {
        let parts = fen.split(" ").collect::<Vec<_>>();
        let mut board = BitBoard::empty();

        let mut square = 64 - 8;
        for c in parts[0].chars() {
            if c == '/' {
                square -= 16;
                continue;
            }
            
            let piece = piece_type_from_char(c);
            
            if piece != ColoredPieceType::None {
                board.place_piece(piece, square_from_u8(square));
                
                square += 1;
            }
            else {
                square += c.to_string().parse::<u8>().unwrap();
            }
        }

        board.whites_turn = parts[1] == "w";

        if parts[3] != "-" {
            board.en_passant_square = square_from_str(parts[3]);
        }

        println!("Loaded FEN {}", fen);

        return board;
    }

    fn toggle_piece_bitboards(&mut self, colored_piece_type: ColoredPieceType, square: Square) {
        match piece_type_from_cpt(colored_piece_type) {
            PieceType::Pawn                       => toggle_bit(&mut self.pawns, square),
            PieceType::Knight                     => toggle_bit(&mut self.knights, square),
            PieceType::Bishop | PieceType::Queen  => toggle_bit(&mut self.diagonal_sliders, square),
            PieceType::Rook   | PieceType::Queen  => toggle_bit(&mut self.orthogonal_sliders, square),
            PieceType::King                       => toggle_bit(&mut self.kings, square),
            PieceType::None => panic!("Toggling \"None\" piece")
        }

        if is_white_piece(colored_piece_type) {
            toggle_bit(&mut self.white_pieces, square);
        }
        else {
            toggle_bit(&mut self.black_pieces, square);
        }
    }

    fn place_piece(&mut self, colored_piece_type: ColoredPieceType, square: Square) {
        self.toggle_piece_bitboards(colored_piece_type, square);
        self.type_field[square as usize] = colored_piece_type;
    }

    fn remove_piece(&mut self, square: Square) {
        let cpt = self.type_field[square as usize];
        self.toggle_piece_bitboards(cpt, square);
        self.type_field[square as usize] = ColoredPieceType::None;
    }
    pub fn get_king_square(&self, white: bool) -> Square {
        let bitboard = self.kings & if white { self.white_pieces } else { self.black_pieces };
        return square_from_u8(bitboard.trailing_zeros() as u8);
    }

    fn square_is_attacked_by(&self, white: bool, target_square: Square) -> bool {
        let color_mask = if white { self.white_pieces } else { self.black_pieces };
        
        if self.kings & color_mask & bitboard_helper::KING_ATTACKS[target_square as usize] != 0 {
            return true;
        }

        if self.knights & color_mask & bitboard_helper::KNIGHT_ATTACKS[target_square as usize] != 0 {
            return true;
        }
   
        let pawn_mask = if white { bitboard_helper::WHITE_PAWN_ATTACKS[target_square as usize] } else {bitboard_helper::BLACK_PAWN_ATTACKS[target_square as usize] };
        if self.pawns & color_mask & pawn_mask != 0 {
            return true;
        }

        let all_pieces = self.white_pieces | self.black_pieces;
        
        let diagonal_attackers = self.diagonal_sliders & color_mask & bitboard_helper::DIAGONALE_ATTACKS[target_square as usize];
        if sliders_have_attack(target_square, diagonal_attackers, all_pieces) {
            return true;
        }

        let orthogonal_attackers = self.orthogonal_sliders & color_mask & bitboard_helper::ORTHOGONAL_ATTACKS[target_square as usize];
        if sliders_have_attack(target_square, orthogonal_attackers, all_pieces) {
            return true;
        }
        
        fn sliders_have_attack(target_square: Square, mut sliders: u64, blockers: u64) -> bool {
            for _ in 0..(sliders.count_ones()) {
                let index = sliders.trailing_zeros();
                let flag = 1_u64 << index;
                sliders &= !flag;

                let in_between = bitboard_helper::IN_BETWEEN_SQUARES[(target_square as u32 + index * 64) as usize];

                if blockers & in_between == 0 {
                    return true;
                }
            }

            return false;
        }

        return false;
    }

    fn in_check(&self) -> bool {
        return self.square_is_attacked_by(!self.whites_turn, self.get_king_square(self.whites_turn));
    }

    fn get_legal_moves(&self) -> ArrayVec<ChessMove, 200> {
        let mut list = ArrayVec::new();

        
    }
}