use crate::{bitboard_helper, piece_type, uci_move::{UCIMove, self}, piece_list::{PieceList, self}, chess_move::ChessMove, zoberist_hash};
use std::cmp;

#[derive(Clone, Copy)]
pub struct Board {
    //Flags
    whites_turn: bool,
    en_passant_square: u8,
    white_left_castle: bool,
    white_right_castle: bool,
    black_left_castle: bool,
    black_right_castle: bool,

    //Pieces
    piece_field: [u8; 64],
    piece_lists: [PieceList; 10],
    white_king_pos: u8,
    black_king_pos: u8,

    //cache
    //cached_pseudo_legal_moves: Vec<ChessMove>
}

const KING_MOVES: [&[u8]; 64] = [
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

const KNIGHT_MOVES: [&[u8]; 64] = [
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

impl Board {
    pub fn gen_zoberist_hash(&self) -> u64 {
        let mut hash = 0;
        for i in 0..64 {
            if self.piece_field[i] != piece_type::EMPTY {
                hash ^= zoberist_hash::VALUES[i][self.piece_field[i] as usize];
            }
        }

        return  hash;
    }

    pub fn start_position() -> Board {
        let piece_field: [u8; 64] = [
            piece_type::WHITE_ROOK, piece_type::WHITE_KNIGHT, piece_type::WHITE_BISHOP, piece_type::WHITE_QUEEN,
            piece_type::WHITE_KING, piece_type::WHITE_BISHOP, piece_type::WHITE_KNIGHT, piece_type::WHITE_ROOK,
            piece_type::WHITE_PAWN, piece_type::WHITE_PAWN, piece_type::WHITE_PAWN, piece_type::WHITE_PAWN, 
            piece_type::WHITE_PAWN, piece_type::WHITE_PAWN, piece_type::WHITE_PAWN, piece_type::WHITE_PAWN,
            
            piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY, 
            piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY,
            piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY,
            piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY,

            piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY,
            piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY,
            piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY,
            piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY, piece_type::EMPTY,

            piece_type::BLACK_PAWN, piece_type::BLACK_PAWN, piece_type::BLACK_PAWN, piece_type::BLACK_PAWN, 
            piece_type::BLACK_PAWN, piece_type::BLACK_PAWN, piece_type::BLACK_PAWN, piece_type::BLACK_PAWN,
            piece_type::BLACK_ROOK, piece_type::BLACK_KNIGHT, piece_type::BLACK_BISHOP, piece_type::BLACK_QUEEN,
            piece_type::BLACK_KING, piece_type::BLACK_BISHOP, piece_type::BLACK_KNIGHT, piece_type::BLACK_ROOK,
        ];

        let mut piece_lists: [PieceList; 10] = [PieceList::new(); 10];

        for i in 0..piece_field.len() {
            let p = piece_field[i];
            if p < piece_type::WHITE_KING {
                piece_lists[p as usize].add_at_square(i as u8);
            }
        }

        return Board { whites_turn: true, en_passant_square: 255, piece_field, piece_lists, white_left_castle: true, white_right_castle: true, black_left_castle: true, black_right_castle: true, white_king_pos: 4, black_king_pos: 60 };
    }

    pub fn add_piece(&mut self, square: u8, piece: u8) {
        debug_assert!(square < 64);
        debug_assert!(piece < piece_type::EMPTY);
        self.piece_field[square as usize] = piece;


        if piece == piece_type::WHITE_KING {
            self.white_king_pos = square;
            return;
        }
        if piece == piece_type::BLACK_KING {
            self.black_king_pos = square;
            return;
        }

        self.piece_lists[piece as usize].add_at_square(square);
    }
    pub fn remove_piece(&mut self, square: u8) {
        debug_assert!(square < 64);
        let piece = self.piece_field[square as usize];
        debug_assert!(piece < piece_type::EMPTY);


        self.piece_field[square as usize] = piece_type::EMPTY;

        //HACK?
        if piece == piece_type::WHITE_KING {
            //self.white_king_pos = 255;
            return;
        }
        if piece == piece_type::BLACK_KING {
            //self.black_king_pos = 255;
            return;
        }

        self.piece_lists[piece as usize].remove_at_square(square);
    }

    pub fn move_piece(&mut self, start_square: u8, target_square: u8) {
        debug_assert!(target_square != start_square);

        self.add_piece(target_square, self.piece_field[start_square as usize]);
        self.remove_piece(start_square);
    }

    pub fn capture_piece(&mut self, start_square: u8, target_square: u8) {
        self.remove_piece(target_square);

        self.move_piece(start_square, target_square);
    }


    pub fn make_move(&mut self, m: &ChessMove) {
        
        if m.move_piece_type == piece_type::WHITE_KING {
            self.white_left_castle = false;
            self.white_right_castle = false;
        }
        
        if m.move_piece_type == piece_type::BLACK_KING {
            self.black_left_castle = false;
            self.black_right_castle = false;
        }
        
        if m.start_square == 0 || m.target_square == 0 {
            self.white_left_castle = false;
        }
        if m.start_square == 7 || m.target_square == 7 {
            self.white_right_castle = false;
        }
        if m.start_square == 56 || m.target_square == 56 {
            self.black_left_castle = false;
        }
        if m.start_square == 63 || m.target_square == 63 {
            self.black_right_castle = false;
        }
        
        let pawn_direction: i32 = if self.whites_turn { 1 } else { -1 };
        //double pawn move
        if m.move_piece_type >> 1 == piece_type::PAWN && m.start_square.abs_diff(m.target_square) == 16 {
            self.en_passant_square = (m.target_square as i32 - pawn_direction * 8) as u8;
            //println!("Found ep square {}", self.en_passant_square);
        }
        else {
            self.en_passant_square = 255
        }
        
        
        //Moves the rooks
        if m.is_castle() {
            self.white_king_pos = m.target_square;
            let king_height = m.start_square / 8;
            
            //left castle
            if m.start_square < m.target_square {
                self.move_piece(0 + king_height * 8, m.target_square + 1);  
            }
            else {
                self.move_piece(7 + king_height * 8, m.target_square - 1);      
            }
        }
        
        if m.is_capture() && !m.is_en_passant {
            self.capture_piece(m.start_square, m.target_square);
        }
        else {
            self.move_piece(m.start_square, m.target_square);
        }
        
        if m.is_en_passant {
            
            self.remove_piece((m.target_square as i8 - pawn_direction as i8 * 8) as u8);
        }
        
        if m.is_promotion() {
            self.remove_piece(m.target_square);
            self.add_piece(m.target_square, m.promotion_piece_type);
        }

        self.whites_turn = !self.whites_turn;
    }
    
    pub fn generate_pseudo_legal_moves(&self) -> Vec<ChessMove> {
        let mut list: Vec<ChessMove> = Vec::new();
        let moving_color: u8 = if self.whites_turn { 0 } else { 1 };

        //Knight moves
        let moving_knight = (piece_type::WHITE_KNIGHT | moving_color);
        let mut piece_list = self.piece_lists[moving_knight as usize];

        for i in 0..piece_list.count() {
            let start_square = piece_list.get_occupied_square(i);
            
            for target_square in KNIGHT_MOVES[start_square as usize] {              
                let target_piece_type = self.piece_field[*target_square as usize];

                if target_piece_type == piece_type::EMPTY || target_piece_type & 1 != moving_color {
                    list.push(ChessMove::new_move(start_square, *target_square, moving_knight, target_piece_type))
                }
            }
        }

        //Pawns
        let pawn_direction: i32 = if self.whites_turn { 1 } else { -1 };
        let start_rank: u8 = if self.whites_turn { 1 } else { 6 };
        let promotion_rank: u8 = if self.whites_turn { 7 } else { 0 };      
        let moving_pawn = (piece_type::WHITE_PAWN | moving_color);
        piece_list = self.piece_lists[moving_pawn as usize];
        for i in 0..piece_list.count() {
            let start_square = piece_list.get_occupied_square(i);
            let x = start_square % 8;
            let y = start_square / 8;
            
            let mut target_square = (start_square as i32 + 8 * pawn_direction) as u8;

            //forward move 
            if  self.piece_field[target_square as usize] == piece_type::EMPTY {
                
                self.add_pawn_move(start_square, target_square, moving_pawn, piece_type::EMPTY, promotion_rank, &mut list);

                target_square = (start_square as i32 + 2 * 8 * pawn_direction) as u8;

                if start_square / 8 == start_rank {
                    if self.piece_field[target_square as usize] == piece_type::EMPTY {
                        list.push(ChessMove::new_move(start_square, target_square, moving_pawn, piece_type::EMPTY));
                    }
                }
            }

            //capture left
            if x > 0 {
                target_square = (start_square as i32 + 8 * pawn_direction - 1) as u8;
                let target_piece_type = self.piece_field[target_square as usize];
                if  target_piece_type != piece_type::EMPTY && target_piece_type & 1 != moving_color{
                    self.add_pawn_move(start_square, target_square, moving_pawn, target_piece_type, promotion_rank, &mut list);
                }

                if target_square == self.en_passant_square {
                    list.push(ChessMove::new_pawn_move(start_square, target_square, moving_pawn, piece_type::EMPTY, piece_type::EMPTY, true));
                }
            }

            //capture right
            if x < 7 {
                target_square = (start_square as i32 + 8 * pawn_direction + 1) as u8;
                let target_piece_type = self.piece_field[target_square as usize];
                if  target_piece_type != piece_type::EMPTY && target_piece_type & 1 != moving_color{
                    self.add_pawn_move(start_square, target_square, moving_pawn, target_piece_type, promotion_rank, &mut list);
                }

                if target_square == self.en_passant_square {
                    list.push(ChessMove::new_pawn_move(start_square, target_square, moving_pawn, piece_type::EMPTY, piece_type::EMPTY, true));
                }
            }
        }


        fn add_slide_move(start_square: u8, target_square: u8, move_piece_type: u8, target_piece_type: u8, moving_color: u8, list: &mut Vec<ChessMove>) -> bool{
            if target_piece_type == piece_type::EMPTY || target_piece_type & 1 != moving_color {
                list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
            }

            return target_piece_type != piece_type::EMPTY
        }

        //Rook
        let mut move_piece_type = (piece_type::WHITE_ROOK | moving_color);
        piece_list = self.piece_lists[move_piece_type as usize];
        for i in 0..piece_list.count() {
            let start_square = piece_list.get_occupied_square(i);
            let x = start_square % 8;
            let y = start_square / 8;

            for ty in (y + 1)..8 {
                let target_square = x + ty * 8;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
            
            for ty in (0..y).rev() {
                let target_square = x + ty * 8;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }

            for tx in (x + 1)..8 {
                let target_square = tx + y * 8;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
            
            for tx in (0..x).rev() {
                let target_square = tx + y * 8;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
        }
        
        //Bishop
        move_piece_type = (piece_type::WHITE_BISHOP | moving_color);
        piece_list = self.piece_lists[move_piece_type as usize];
        for i in 0..piece_list.count() {
            let start_square = piece_list.get_occupied_square(i);
            let x = start_square % 8;
            let y = start_square / 8;

            //println!("{}", start_square);    
        
            //up right 
            for delta in 1..(cmp::min(7 - x, 7 - y) + 1) {
                let target_square = start_square + delta * 9;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }

            //up left
            for delta in 1..(cmp::min(x, 7 - y) + 1) {
                let target_square = start_square + delta * 7;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
            
            //down right
            for delta in 1..(cmp::min(7 - x, y) + 1) {
                let target_square = start_square - delta * 7;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }

            //down left
            for delta in 1..(cmp::min(x, y) + 1) {
                let target_square = start_square - delta * 9;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
        }

        //Queen
        move_piece_type = (piece_type::WHITE_QUEEN | moving_color);
        piece_list = self.piece_lists[move_piece_type as usize];
        for i in 0..piece_list.count() {
            let start_square = piece_list.get_occupied_square(i);
            let x = start_square % 8;
            let y = start_square / 8;

            for ty in (y + 1)..8 {
                let target_square = x + ty * 8;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
            
            for ty in (0..y).rev() {
                let target_square = x + ty * 8;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }

            for tx in (x + 1)..8 {
                let target_square = tx + y * 8;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
            
            for tx in (0..x).rev() {
                let target_square = tx + y * 8;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
        
            
            //up right 
            for delta in 1..(cmp::min(7 - x, 7 - y)  + 1) {
                let target_square = start_square + delta * 9;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }

            //up left
            for delta in 1..(cmp::min(x, 7 - y) + 1) {
                let target_square = start_square + delta * 7;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
            
            //down right
            for delta in 1..(cmp::min(7 - x, y) + 1) {
                let target_square = start_square - delta * 7;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }

            //down left
            for delta in 1..(cmp::min(x, y) + 1) {
                let target_square = start_square - delta * 9;
                let target_piece_type = self.piece_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
        }

        //King
        let king_pos = if self.whites_turn { self.white_king_pos } else { self.black_king_pos };
        let moving_king = (piece_type::WHITE_KING | moving_color);
        for target_square in KING_MOVES[king_pos as usize] {              
            let target_piece_type = self.piece_field[*target_square as usize];

            if target_piece_type == piece_type::EMPTY || target_piece_type & 1 != moving_color {
                list.push(ChessMove::new_move(king_pos, *target_square, moving_king, target_piece_type))
            }
        }
        
        if self.whites_turn {
            if self.white_left_castle {
                if self.piece_field[1] == piece_type::EMPTY && self.piece_field[2] == piece_type::EMPTY && self.piece_field[1] == piece_type::EMPTY {
                    list.push(ChessMove::new_move(king_pos, 2, moving_king, piece_type::EMPTY))
                }
            }

            if self.white_right_castle {
                if self.piece_field[5] == piece_type::EMPTY && self.piece_field[6] == piece_type::EMPTY {
                    list.push(ChessMove::new_move(king_pos, 6, moving_king, piece_type::EMPTY))
                }
            }
        }
        else {
            if self.black_left_castle {
                if self.piece_field[57] == piece_type::EMPTY && self.piece_field[58] == piece_type::EMPTY && self.piece_field[59] == piece_type::EMPTY {
                    list.push(ChessMove::new_move(king_pos, 58, moving_king, piece_type::EMPTY))
                }
            }

            if self.white_right_castle {
                if self.piece_field[61] == piece_type::EMPTY && self.piece_field[62] == piece_type::EMPTY {
                    list.push(ChessMove::new_move(king_pos, 62, moving_king, piece_type::EMPTY))
                }
            }
        }

        return list;
    }

    //[TODO] castle stuff
    pub fn has_king_capture(&self) -> bool {
        let moves = self.generate_pseudo_legal_moves();
        
        for m in moves {
            if m.target_piece_type >> 1 == piece_type::KING {
                return true;
            }
        }

        return false;
    }

    pub fn filter_legal_moves(&self, list: &mut Vec<ChessMove>) {
        let mut remove: Vec<usize> = Vec::new();

        for i in 0..list.len() {
            let m = list[i];

            let mut buffer = (*self).clone();
            buffer.make_move(&m);
            
            if buffer.has_king_capture() {
                remove.push(i);
            }    
        }

        for i in (0..remove.len()).rev() {
            let index = remove[i];

            list.remove(index);
        }
    }
    
    pub fn get_legal_moves(&self) -> Vec<ChessMove> {
        let mut list = self.generate_pseudo_legal_moves();

        self.filter_legal_moves(&mut list);

        return list;
    }   

    pub fn print_moves(list: Vec<ChessMove>){
        print!("Moves {}[", list.len());
    
        for m in &list {
            m.print();  
            print!(" ");      
        }
    
        println!("]");
    }

    fn add_pawn_move(&self, start_square: u8, target_square: u8, move_piece_type: u8, target_piece_type: u8, promotion_rank: u8, list: &mut Vec<ChessMove>) {
        if(target_square / 8 == promotion_rank) {
            list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, piece_type::WHITE_KNIGHT | (move_piece_type & 1), false));
            list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, piece_type::WHITE_BISHOP | (move_piece_type & 1), false));
            list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, piece_type::WHITE_ROOK   | (move_piece_type & 1), false));
            list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, piece_type::WHITE_QUEEN  | (move_piece_type & 1), false));
        }
        else {
            list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
        }
    }

    pub fn get_piece_color(&self, index: u8) -> u8 {
        debug_assert!(index < 64);

        let piece = self.piece_field[index as usize];
        if  piece == piece_type::EMPTY {
            return 2;
        }

        return piece & 1;
    }


    pub fn print(&self) {
        const PIECE_CHAR: [char; 13] = ['P', 'p', 'N', 'n', 'B', 'b', 'R', 'r', 'Q', 'q', 'K', 'k', ' '];

        println!("   {}", String::from_utf8(vec![b'_'; 16]).unwrap());

        for y in (0..8).rev() {
            print!("{} |", y + 1);
            for x in 0..8 {
                let p = self.piece_field[x + y * 8];
                
                print!("{} ", PIECE_CHAR[p as usize]);
                
            }
            println!("|");
        }

        println!("   {}", String::from_utf8(vec![b'-'; 16]).unwrap());
        println!("   a b c d e f g h");

        println!("Ep: {}", self.en_passant_square)
    }
}