use crate::{bitboard_helper, constants, uci_move::{UCIMove, self}, piece_list::{PieceList, self}, chess_move::{ChessMove, self}, zoberist_hash::{self, ZoberistHash}, board, attack_board::{AttackBoard, self}};
use std::{cmp, string, ops::Index };
use arrayvec::ArrayVec;

#[derive(Clone, Copy)]
pub struct Board {
    //Flags
    whites_turn: bool,
    en_passant_square: u8,
    castle_move_square: u8,
    castle_start_square: u8,
    white_queen_castle: bool,
    white_king_castle: bool,
    black_queen_castle: bool,
    black_king_castle: bool,

    //Pieces
    pub type_field: [u8; 64],
    piece_map: [u8; 64],
    pub piece_lists: [PieceList; 10],
    white_king_pos: u8,
    black_king_pos: u8,

    //Extra data 
    attack_board: AttackBoard,
    zoberist_hash: ZoberistHash,
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
    pub fn get_hash(&self) -> u64 {
        return self.zoberist_hash.get_hash();
    }

    pub fn is_whites_turn(&self) -> bool {
        return self.whites_turn;
    }

    fn empty_board() -> Self {
        let piece_field: [u8; 64] = [constants::NULL_PIECE; 64];
        let piece_lists: [PieceList; 10] = [PieceList::new(); 10];
        let piece_map: [u8; 64] = [constants::NO_SQUARE; 64];

        return Board { whites_turn: true, 
            en_passant_square: constants::NO_SQUARE, castle_move_square: constants::NO_SQUARE, castle_start_square: constants::NO_SQUARE, 
            type_field: piece_field, piece_lists, piece_map, 
            white_queen_castle: false, white_king_castle: false, black_queen_castle: false, black_king_castle: false, 
            white_king_pos: constants::E1, black_king_pos: constants::E8,
            attack_board: AttackBoard::empty(), zoberist_hash: ZoberistHash::new() };
    }
    pub fn start_position() -> Self {
        return Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    //half move clock and full move number missing
    pub fn from_fen(fen: &str) -> Self {
        let parts = fen.split(" ").collect::<Vec<_>>();
        let mut board = Board::empty_board();
        
        let mut square = 64 - 8;
        for c in parts[0].chars() {
            if c == '/' {
                square -= 16;
                continue;
            }
            
            let index = constants::PIECE_CHAR.iter().position(|&r| r == c).unwrap();
            
            if index <= constants::BLACK_KING as usize {
                //println!("Adding {} at {}" , constants::PIECE_CHAR[index], constants::SQUARE_NAME[square as usize]);
                board.add_piece(square, index as u8);
                
                square += 1;
            }
            else {
                square += (index - 12) as u8;
            }
        }
        
        board.whites_turn = parts[1] == "w";
        
        for c in parts[2].chars() {
            match c {
                'K' => board.white_king_castle = true,
                'Q' => board.white_queen_castle = true,
                'k' => board.black_king_castle = true,
                'q' => board.black_queen_castle = true,
                _ => ()
            }
        }
        
        if parts[3] != "-" {
            board.en_passant_square = constants::SQUARE_NAME.iter().position(|&r| r == parts[3]).unwrap() as u8;
        }

        println!("Loaded FEN {}", fen);
        
        board.zoberist_hash.recalculate_hash(&board.type_field, board.whites_turn,
            board.en_passant_square, board.white_queen_castle, board.white_king_castle, board.black_queen_castle, board.black_king_castle);
        
        board.print();

        return board;
    }
    
    pub fn get_fen(&self) -> String {
        let mut s = "".to_owned();
        for y in (0..8).rev() {
            let mut empty_count = 0;
            for x in 0..8 {
                let square = x + y * 8;

                if self.type_field[square as usize] != constants::NULL_PIECE {
                    if empty_count > 0 {
                        s += &empty_count.to_string();
                        empty_count = 0;
                    }

                    s += &constants::PIECE_CHAR[self.type_field[square as usize] as usize].to_string();
                }
                else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                s += &empty_count.to_string();
            }

            if y != 0 {
                s += "/";
            }
        }

        return s;
    }

    pub fn add_piece(&mut self, square: u8, piece_type: u8) {
        debug_assert!(square < 64);
        debug_assert!(piece_type < constants::NULL_PIECE);

        self.type_field[square as usize] = piece_type;

        self.attack_board.add_at_square(square, piece_type, &self.type_field);

        if piece_type == constants::WHITE_KING {
            self.white_king_pos = square;
            return;
        }
        if piece_type == constants::BLACK_KING {
            self.black_king_pos = square;
            return;
        }

        self.piece_lists[piece_type as usize].add_at_square(square, &mut self.piece_map);


    }
    pub fn remove_piece(&mut self, square: u8) {
        debug_assert!(square < 64);
        let piece = self.type_field[square as usize];
        debug_assert!(piece < constants::NULL_PIECE);

        self.attack_board.remove_at_square(square, &self.type_field);

        self.type_field[square as usize] = constants::NULL_PIECE;

        if piece == constants::WHITE_KING ||  piece == constants::BLACK_KING {   
            return;
        }

        self.piece_lists[piece as usize].remove_at_square(square, &mut self.piece_map);
    }

    pub fn move_piece(&mut self, start_square: u8, target_square: u8) {
        debug_assert!(target_square != start_square);
        let piece_type = self.type_field[start_square as usize];

        self.remove_piece(start_square);
        self.add_piece(target_square, piece_type);
    }
    
    pub fn capture_piece(&mut self, start_square: u8, target_square: u8) {
        //println!("Capture at {}", constants::SQUARE_NAME[target_square as usize]);
        self.remove_piece(target_square);
        
        self.move_piece(start_square, target_square);
    }
    
    pub fn make_move(&mut self, m: &ChessMove) {

        self.zoberist_hash.update_hash(*m, self.en_passant_square, self.white_queen_castle, self.white_king_castle, self.black_queen_castle, self.black_king_castle);

        if m.move_piece_type == constants::WHITE_KING {
            self.white_queen_castle = false;
            self.white_king_castle = false;
        }
        
        if m.move_piece_type == constants::BLACK_KING {
            self.black_queen_castle = false;
            self.black_king_castle = false;
        }
        
        if m.start_square == constants::A1 || m.target_square == constants::A1 {
            self.white_queen_castle = false;
        }
        if m.start_square == constants::H1 || m.target_square == constants::H1 {
            self.white_king_castle = false;
        }
        if m.start_square == constants::A8 || m.target_square == constants::A8 {
            self.black_queen_castle = false;
        }
        if m.start_square == constants::H8 || m.target_square == constants::H8 {
            self.black_king_castle = false;
        }
        
        let pawn_direction: i32 = if self.whites_turn { 1 } else { -1 };
        //double pawn move
        if m.move_piece_type >> 1 == constants::PAWN && m.start_square.abs_diff(m.target_square) == 16 {
            self.en_passant_square = (m.target_square as i32 - pawn_direction * 8) as u8;
            //println!("Found ep square {}", self.en_passant_square);
        }
        else {
            self.en_passant_square = constants::NO_SQUARE;
        }
        
        //Moves the rooks
        if m.is_castle() {
            let king_height = m.start_square / 8;
            
            self.castle_move_square = (m.start_square + m.target_square) / 2;
            self.castle_start_square = m.start_square;
            //left castle
            if m.start_square > m.target_square {
                self.move_piece(0 + king_height * 8, m.target_square + 1);
            }
            //right castle
            else {
                self.move_piece(7 + king_height * 8, m.target_square - 1);      
            }
        }
        else {
            self.castle_move_square = constants::NO_SQUARE;
            self.castle_start_square = constants::NO_SQUARE;
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
    
    pub fn in_check(&self) -> bool {
        let king_square = if self.whites_turn { self.white_king_pos } else { self.black_king_pos };

        return self.attack_board.square_is_attacked(!self.whites_turn, king_square);
    }

    pub fn in_double_check(&self) -> bool {
        let king_square = if self.whites_turn { self.white_king_pos } else { self.black_king_pos };

        let count = self.attack_board.square_attack_count(!self.whites_turn, king_square);
        if count > 2 {
            println!("This should never happen {}", count);
        }
        return count == 2;
    }

    pub fn generate_pseudo_legal_moves(&self) -> ArrayVec<ChessMove, 200> {      
        let mut list: ArrayVec<ChessMove, 200> = ArrayVec::new();
        
        let moving_color: u8 = if self.whites_turn { 0 } else { 1 };

        //Pawns
        fn add_pawn_move(start_square: u8, target_square: u8, move_piece_type: u8, target_piece_type: u8, promotion_rank: u8, list: &mut ArrayVec<ChessMove, 200>) {
            if(target_square / 8 == promotion_rank) {
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, constants::WHITE_KNIGHT | (move_piece_type & 1), false));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, constants::WHITE_BISHOP | (move_piece_type & 1), false));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, constants::WHITE_ROOK   | (move_piece_type & 1), false));
                list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, target_piece_type, constants::WHITE_QUEEN  | (move_piece_type & 1), false));
            }
            else {
                list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
            }
        }

        let pawn_direction: i32 = if self.whites_turn { 1 } else { -1 };
        let start_rank: u8 = if self.whites_turn { 1 } else { 6 };
        let promotion_rank: u8 = if self.whites_turn { 7 } else { 0 };      
        let mut move_piece_type = (constants::WHITE_PAWN | moving_color);
        let mut piece_list = self.piece_lists[move_piece_type as usize];
        for i in 0..piece_list.count() {
            let start_square = piece_list.get_occupied_square(i);
            let x = start_square % 8;
            let y = start_square / 8;
            
            let mut target_square = (start_square as i32 + 8 * pawn_direction) as u8;

            //forward move 
            if  self.type_field[target_square as usize] == constants::NULL_PIECE {
                
                add_pawn_move(start_square, target_square, move_piece_type, constants::NULL_PIECE, promotion_rank, &mut list);

                target_square = (start_square as i32 + 2 * 8 * pawn_direction) as u8;

                if start_square / 8 == start_rank {
                    if self.type_field[target_square as usize] == constants::NULL_PIECE {
                        list.push(ChessMove::new_move(start_square, target_square, move_piece_type, constants::NULL_PIECE));
                    }
                }
            }

            //capture left
            if x > 0 {
                target_square = (start_square as i32 + 8 * pawn_direction - 1) as u8;
                let target_piece_type = self.type_field[target_square as usize];
                if  target_piece_type != constants::NULL_PIECE && target_piece_type & 1 != moving_color || target_square == self.castle_move_square || target_square == self.castle_start_square {
                    add_pawn_move(start_square, target_square, move_piece_type, target_piece_type, promotion_rank, &mut list);
                }

                if target_square == self.en_passant_square {
                    list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, constants::NULL_PIECE, constants::NULL_PIECE, true));
                }
            }

            //capture right
            if x < 7 {
                target_square = (start_square as i32 + 8 * pawn_direction + 1) as u8;
                let target_piece_type = self.type_field[target_square as usize];
                if  target_piece_type != constants::NULL_PIECE && target_piece_type & 1 != moving_color || target_square == self.castle_move_square || target_square == self.castle_start_square {
                    add_pawn_move(start_square, target_square, move_piece_type, target_piece_type, promotion_rank, &mut list);
                }

                if target_square == self.en_passant_square {
                    list.push(ChessMove::new_pawn_move(start_square, target_square, move_piece_type, constants::NULL_PIECE, constants::NULL_PIECE, true));
                }
            }
        }


        //Knight moves
        move_piece_type = (constants::WHITE_KNIGHT | moving_color);
        piece_list = self.piece_lists[move_piece_type as usize];

        for i in 0..piece_list.count() {
            let start_square = piece_list.get_occupied_square(i);
            
            for target_square in KNIGHT_MOVES[start_square as usize] {              
                let target_piece_type = self.type_field[*target_square as usize];

                if target_piece_type == constants::NULL_PIECE || target_piece_type & 1 != moving_color {
                    list.push(ChessMove::new_move(start_square, *target_square, move_piece_type, target_piece_type))
                }
            }
        }

        fn add_slide_move(start_square: u8, target_square: u8, move_piece_type: u8, target_piece_type: u8, moving_color: u8, list: &mut ArrayVec<ChessMove, 200>) -> bool{
            if target_piece_type == constants::NULL_PIECE || target_piece_type & 1 != moving_color {
                list.push(ChessMove::new_move(start_square, target_square, move_piece_type, target_piece_type));
            }

            return target_piece_type != constants::NULL_PIECE
        }

        //Bishop
        move_piece_type = (constants::WHITE_BISHOP | moving_color);
        piece_list = self.piece_lists[move_piece_type as usize];
        for i in 0..piece_list.count() {
            let start_square = piece_list.get_occupied_square(i);
            let x = start_square % 8;
            let y = start_square / 8;

            //println!("{}", start_square);    
        
            //up right 
            for delta in 1..(cmp::min(7 - x, 7 - y) + 1) {
                let target_square = start_square + delta * 9;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }

            //up left
            for delta in 1..(cmp::min(x, 7 - y) + 1) {
                let target_square = start_square + delta * 7;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
            
            //down right
            for delta in 1..(cmp::min(7 - x, y) + 1) {
                let target_square = start_square - delta * 7;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }

            //down left
            for delta in 1..(cmp::min(x, y) + 1) {
                let target_square = start_square - delta * 9;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
        }

        //Rook
        move_piece_type = (constants::WHITE_ROOK | moving_color);
        piece_list = self.piece_lists[move_piece_type as usize];
        for i in 0..piece_list.count() {
            let start_square = piece_list.get_occupied_square(i);
            let x = start_square % 8;
            let y = start_square / 8;

            for ty in (y + 1)..8 {
                let target_square = x + ty * 8;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
            
            for ty in (0..y).rev() {
                let target_square = x + ty * 8;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }

            for tx in (x + 1)..8 {
                let target_square = tx + y * 8;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
            
            for tx in (0..x).rev() {
                let target_square = tx + y * 8;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
        }

        //Queen
        move_piece_type = (constants::WHITE_QUEEN | moving_color);
        piece_list = self.piece_lists[move_piece_type as usize];
        for i in 0..piece_list.count() {
            let start_square = piece_list.get_occupied_square(i);
            let x = start_square % 8;
            let y = start_square / 8;

            for ty in (y + 1)..8 {
                let target_square = x + ty * 8;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
            
            for ty in (0..y).rev() {
                let target_square = x + ty * 8;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }

            for tx in (x + 1)..8 {
                let target_square = tx + y * 8;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
            
            for tx in (0..x).rev() {
                let target_square = tx + y * 8;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
        
            
            //up right 
            for delta in 1..(cmp::min(7 - x, 7 - y)  + 1) {
                let target_square = start_square + delta * 9;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }

            //up left
            for delta in 1..(cmp::min(x, 7 - y) + 1) {
                let target_square = start_square + delta * 7;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
            
            //down right
            for delta in 1..(cmp::min(7 - x, y) + 1) {
                let target_square = start_square - delta * 7;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }

            //down left
            for delta in 1..(cmp::min(x, y) + 1) {
                let target_square = start_square - delta * 9;
                let target_piece_type = self.type_field[target_square as usize];

                if add_slide_move(start_square, target_square, move_piece_type, target_piece_type, moving_color, &mut list) {
                    break;
                }
            }
        }

        //King
        let king_pos = if self.whites_turn { self.white_king_pos } else { self.black_king_pos };
        let moving_king = (constants::WHITE_KING | moving_color);
        for target_square in KING_MOVES[king_pos as usize] {              
            let target_piece_type = self.type_field[*target_square as usize];

            if target_piece_type == constants::NULL_PIECE || target_piece_type & 1 != moving_color {
                list.push(ChessMove::new_move(king_pos, *target_square, moving_king, target_piece_type))
            }
        }
        
        if !self.in_check() {
            if self.whites_turn {
                if self.white_queen_castle {
                    if self.type_field[constants::B1 as usize] == constants::NULL_PIECE && 
                        self.type_field[constants::C1 as usize] == constants::NULL_PIECE && 
                        self.type_field[constants::D1 as usize] == constants::NULL_PIECE && 
                        !self.attack_board.square_is_attacked(!self.whites_turn, constants::D1) {
                        list.push(ChessMove::new_move(king_pos, constants::C1, moving_king, constants::NULL_PIECE));
                    }
                }
    
                if self.white_king_castle {
                    if self.type_field[constants::F1 as usize] == constants::NULL_PIECE && 
                        self.type_field[constants::G1 as usize] == constants::NULL_PIECE && 
                        !self.attack_board.square_is_attacked(!self.whites_turn, constants::F1) {
                        list.push(ChessMove::new_move(king_pos, constants::G1, moving_king, constants::NULL_PIECE));
                    }
                }
            }
            else {
                if self.black_queen_castle {
                    if self.type_field[constants::B8 as usize] == constants::NULL_PIECE && 
                        self.type_field[constants::C8 as usize] == constants::NULL_PIECE && 
                        self.type_field[constants::D8 as usize] == constants::NULL_PIECE && 
                        !self.attack_board.square_is_attacked(!self.whites_turn, constants::D8) {
                        list.push(ChessMove::new_move(king_pos, constants::C8, moving_king, constants::NULL_PIECE));
                    }
                }
    
                if self.black_king_castle {
                    if self.type_field[constants::F8 as usize] == constants::NULL_PIECE && 
                        self.type_field[constants::G8 as usize] == constants::NULL_PIECE && 
                        !self.attack_board.square_is_attacked(!self.whites_turn, constants::F8) {
                        list.push(ChessMove::new_move(king_pos, constants::G8, moving_king, constants::NULL_PIECE));
                    }
                }
            }
        }

        return list;
    }

    pub fn get_piece_attack_count(&self, square: u8) -> u8 {
        debug_assert!(self.type_field[square as usize] != constants::NULL_PIECE);

        return self.attack_board.piece_attack_move_count((self.type_field[square as usize] & 1) == 0, square);
    }

    pub fn get_square_attack_count(&self, whites_turn: bool,  square: u8) -> u8 {
        return self.attack_board.square_attack_count(whites_turn, square);
    }

    pub fn get_king_square(&self, whites_turn: bool) -> u8 {
        return if whites_turn {self.white_king_pos } else { self.black_king_pos };
    }

    //does not check castle move square and start square
    pub fn check_move_legality(&self, m: ChessMove) -> bool {
        let mut res = true;
        let mut attack_board = self.attack_board.clone();
        attack_board.make_move_for_legallity_check(m, &self.type_field);

        let mut king_square = if self.whites_turn { self.white_king_pos } else { self.black_king_pos };

        if m.move_piece_type >> 1 == constants::KING {
            king_square = m.target_square;
            //println!("Turn: {}", self.whites_turn) ;
        }

        return !attack_board.square_is_attacked(!self.whites_turn, king_square);   
    }
    
    pub fn filter_legal_moves(&self, list: &mut ArrayVec<ChessMove, 200>) {
       let mut remove: Vec<usize> = Vec::new();

       for i in 0..list.len() {
           let m = list[i];
           
           if !self.check_move_legality(m) {
               remove.push(i);
           }
       }

       for i in (0..remove.len()).rev() {
           let index = remove[i];

           list.remove(index);
       }
   }

    pub fn get_legal_moves(&self) -> ArrayVec<ChessMove, 200> {
        let mut list = self.generate_pseudo_legal_moves();

        //println!("Pseudo legal moves: ");
        //Self::print_moves(&list);
        
        self.filter_legal_moves(&mut list);

        return list;
    }

    pub fn print_moves(list: &ArrayVec<ChessMove, 200>){
        print!("Moves {}[", list.len());
    
        for m in list {
            m.print();  
            print!(" ");      
        }
    
        println!("]");
    }
 
    pub fn get_piece_color(&self, index: u8) -> u8 {
        debug_assert!(index < 64);

        let piece = self.type_field[index as usize];
        if  piece == constants::NULL_PIECE {
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
                let p = self.type_field[x + y * 8];
                
                print!("{} ", PIECE_CHAR[p as usize]);
                
            }
            println!("|");
        }

        println!("   {}", String::from_utf8(vec![b'-'; 16]).unwrap());
        println!("   a b c d e f g h");

        if self.white_king_castle || self.white_queen_castle || self.black_king_castle || self.black_queen_castle {
            //KQkq

            if self.white_king_castle {
                print!("K");
            }
            if self.white_queen_castle {
                print!("Q");
            }
            if self.black_king_castle {
                print!("k");
            }
            if self.black_queen_castle {
                print!("q");
            }

            println!();
        }
        println!("{}", if self.whites_turn { "White to move" } else { "Black to move" });
        if self.en_passant_square != constants::NO_SQUARE {
            println!("Ep: {}", constants::SQUARE_NAME[self.en_passant_square as usize]);
        }
        if self.castle_move_square != constants::NO_SQUARE {
            println!("Castle start square: {}", constants::SQUARE_NAME[self.castle_start_square as usize]);
            println!("Castle move square: {}", constants::SQUARE_NAME[self.castle_move_square as usize]);
        }
    }

    pub fn print_attackers(&self) {
        for s in 0..64 {
            let pt = self.type_field[s as usize];
            if pt != constants::NULL_PIECE {
                self.attack_board.print_square_attacker(s, pt);
            }
        }
    }
}