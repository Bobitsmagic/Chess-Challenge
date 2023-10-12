use std::collections::HashSet;

use arrayvec::ArrayVec;

use crate::{board::{Board, self}, chess_move::ChessMove, constants, bitboard_helper};

#[derive(PartialEq)]
pub enum GameState  {
    Undecided, Draw, Checkmate
}

pub struct Game {
    board_history: HashSet<u64>,
    board_stack: Vec<Board>,
    dmc_stack: Vec<u32>,
    board: Board,
    
    moves_generated: bool,
    cached_moves: ArrayVec<ChessMove, 200>,

    pub white_pawns_bitboard_stack: Vec<u64>,
    pub black_pawns_bitboard_stack: Vec<u64>,
}


impl  Game {
    pub fn from_fen(fen: &str) -> Self {
        let parts = fen.split(" ").collect::<Vec<_>>();

        let mut board = Board::from_fen(fen);


        let mut dmc = 0;
        
        if parts.len() >= 5 {
            if parts[4].len() > 0 {
                dmc = parts[4].parse::<u32>().unwrap();
            }
        }
        
        let mut dmc_stack = Vec::new();
        dmc_stack.push(dmc);

        let mut white_pawns_bitboard = 0;
        let mut black_pawns_bitboard = 0;


        for i in 0..64  {
            if board.type_field[i] == constants::WHITE_PAWN {
                bitboard_helper::toggle_bit(&mut white_pawns_bitboard, i as u8);
            }
            if board.type_field[i] == constants::BLACK_PAWN {
                bitboard_helper::toggle_bit(&mut black_pawns_bitboard, i as u8);
            }
        }

        return Game { board_history: HashSet::new(), board_stack: Vec::new(), board, dmc_stack, 
            cached_moves: ArrayVec::new(), moves_generated: false, white_pawns_bitboard_stack: vec![white_pawns_bitboard], black_pawns_bitboard_stack: vec![black_pawns_bitboard]}
    }

    pub fn is_whites_turn(&self) -> bool {
        return self.board.is_whites_turn();
    }

    pub fn get_start_position() -> Self {
        return Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    pub fn get_board(&self) -> Board {
        return self.board;
    }

    pub fn white_pawns_bitboard(&self) -> u64 {
        return  *self.white_pawns_bitboard_stack.last().unwrap();
    }
    pub fn black_pawns_bitboard(&self) -> u64 {
        return  *self.black_pawns_bitboard_stack.last().unwrap();
    }

    pub fn make_move(&mut self, m: ChessMove) {
        debug_assert!(self.board.check_move_legality(m));
        debug_assert!(self.get_game_state() == GameState::Undecided);
        
        let mut dmc = self.fifty_move_counter();
        dmc += 1;
        if m.is_direct_capture() || m.move_piece_type >> 1 == constants::PAWN {
            dmc = 0;
        }

        //update stacks
        self.dmc_stack.push(dmc);
        self.board_stack.push(self.board);
        self.board_history.insert(self.board.get_hash());
        
        //make move
        self.board = self.board.clone();
        self.board.make_move(&m);

        self.moves_generated = false;

        let pawn_direction: i32 = if m.is_white_move() { 1 } else { -1 };
        let mut white_pawns_bitboard = self.white_pawns_bitboard();
        let mut black_pawns_bitboard = self.black_pawns_bitboard();
        match m.move_piece_type {
            constants::WHITE_PAWN => {
                bitboard_helper::toggle_bit(&mut white_pawns_bitboard, m.start_square);
                
                if !m.is_promotion() {
                    bitboard_helper::toggle_bit(&mut white_pawns_bitboard, m.target_square);
                }

                if m.is_en_passant {
                    bitboard_helper::toggle_bit(&mut black_pawns_bitboard, 
                        (m.target_square as i8 - pawn_direction as i8 * 8) as u8);
                }
            }
            constants::BLACK_PAWN => {
                bitboard_helper::toggle_bit(&mut black_pawns_bitboard, m.start_square);
                if !m.is_promotion() {
                    bitboard_helper::toggle_bit(&mut black_pawns_bitboard, m.target_square);
                }

                if m.is_en_passant {
                    bitboard_helper::toggle_bit(&mut white_pawns_bitboard, 
                        (m.target_square as i8 - pawn_direction as i8 * 8) as u8);
                }
            }
            _ => ()
        }

        match m.capture_piece_type {
            constants::WHITE_PAWN => bitboard_helper::toggle_bit(&mut white_pawns_bitboard, m.start_square),
            constants::BLACK_PAWN => bitboard_helper::toggle_bit(&mut black_pawns_bitboard, m.start_square),
            _ => ()
        }

        self.white_pawns_bitboard_stack.push(white_pawns_bitboard);
        self.black_pawns_bitboard_stack.push(black_pawns_bitboard);
    }

    pub fn undo_move(&mut self) {
        self.dmc_stack.pop();

        self.board = self.board_stack.pop().unwrap();

        self.board_history.remove(&self.board.get_hash());

        self.moves_generated = false;

        self.white_pawns_bitboard_stack.pop();
        self.black_pawns_bitboard_stack.pop();
    }

    pub fn get_legal_moves(&mut self) -> ArrayVec<ChessMove, 200> {
        if !self.moves_generated {
            self.cached_moves = self.board.get_legal_moves();
        }

        return self.cached_moves.clone();
    }

    pub fn fifty_move_counter(&self) -> u32 {
        return *self.dmc_stack.last().unwrap();
    }

    pub fn get_game_state(&mut self) -> GameState {
        if self.get_legal_moves().len() == 0 {
            if self.board.in_check() {
                return GameState::Checkmate;
            } 
            else {
                //Stale mate
                return GameState::Draw;
            }
        }   

        //Draw by repetition || draw by 50 move  rule
        if self.board_history.contains(&self.board.get_hash()) || self.fifty_move_counter() >= 50 {
            return GameState::Draw;
        }

        return GameState::Undecided;
    }    
}
