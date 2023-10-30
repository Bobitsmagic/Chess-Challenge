use std::{collections::HashSet, fmt};

use arrayvec::ArrayVec;

use crate::{chess_move::ChessMove, constants, bitboard_helper, bit_board::BitBoard, piece_type::PieceType};

#[derive(PartialEq)]
pub enum GameState  {
    Undecided, WhiteCheckmate, BlackCheckmate, Stalemate, FiftyMove, Repetition, InsuffMaterial
}

impl GameState {
    pub fn is_draw(&self) -> bool {
        return *self != GameState::Undecided && !self.is_checkmate();
    }

    pub fn is_checkmate(&self) -> bool {
        return *self == GameState::WhiteCheckmate || 
            *self == GameState::BlackCheckmate;
    } 

    pub fn to_string(&self) -> &str {
        return match *self {
            GameState::Undecided => "Undecided",

            GameState::WhiteCheckmate => "White checkmate",
            GameState::BlackCheckmate => "Black checkmate",
            
            GameState::Stalemate => "Draw: Stalemate",
            GameState::FiftyMove => "Draw: Fifty move rule",
            GameState::Repetition => "Draw: Repetition",
            GameState::InsuffMaterial => "Draw: Insufficient material",

            _ => "Oh no",
        }
    }
}

pub struct Game {
    board_history: HashSet<u128>,
    board_stack: Vec<BitBoard>,
    move_stack: Vec<ChessMove>,
    dmc_stack: Vec<u32>,
    board: BitBoard,
    
    moves_generated: bool,
    cached_moves: ArrayVec<ChessMove, 200>,
}


impl  Game {
    pub fn from_fen(fen: &str) -> Self {
        let parts = fen.split(" ").collect::<Vec<_>>();

        let mut board = BitBoard::from_fen(fen);

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

        return Game { board_history: HashSet::new(), board_stack: Vec::new(), move_stack: Vec::new(), board, dmc_stack, 
            cached_moves: ArrayVec::new(), moves_generated: false }
    }

    pub fn from_board(board: BitBoard) -> Self {
        let mut dmc = 0;        
        let mut dmc_stack = Vec::new();
        dmc_stack.push(dmc);

        let mut white_pawns_bitboard = 0;
        let mut black_pawns_bitboard = 0;

        return Game { board_history: HashSet::new(), board_stack: Vec::new(), move_stack: Vec::new(), board, dmc_stack, 
            cached_moves: ArrayVec::new(), moves_generated: false }
    }

    pub fn is_whites_turn(&self) -> bool {
        return self.board.is_whites_turn();
    }

    pub fn get_start_position() -> Self {
        return Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    pub fn get_board(&self) -> BitBoard {
        return self.board;
    }

    pub fn make_move(&mut self, m: ChessMove) {

        assert!(self.get_game_state() == GameState::Undecided);
        
        let mut dmc = self.fifty_move_counter();
        dmc += 1;
        if m.is_direct_capture() ||  PieceType::from_cpt(m.move_piece_type) == PieceType::Pawn {
            dmc = 0;
        }

        //update stacks
        self.dmc_stack.push(dmc);
        self.board_stack.push(self.board);
        self.board_history.insert(self.board.get_hash_u128());
        self.move_stack.push(m);

        //make move
        self.board = self.board.clone();
        self.board.make_move(m);

        self.moves_generated = false;
    }

    pub fn undo_move(&mut self) {
        self.dmc_stack.pop();

        self.board = self.board_stack.pop().unwrap();

        self.board_history.remove(&self.board.get_hash_u128());
        self.move_stack.pop();

        self.moves_generated = false;
    }

    pub fn to_string(&self) -> String {
        let mut s = "".to_owned();

        for i in 0..self.board_stack.len() {
            s += &self.move_stack[i].get_board_name(&self.board_stack[i]);
            
            s += " ";
        }

        return s;
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

    //[TODO] insuff material
    pub fn get_game_state(&mut self) -> GameState {
        if self.get_legal_moves().len() == 0 {
            if self.board.in_check() {
                if self.is_whites_turn() {
                    return GameState::WhiteCheckmate;
                }
                else {
                    return GameState::BlackCheckmate;
                }
            } 
            else {
                //Stale mate
                return GameState::Stalemate;
            }
        }   

        if self.board_history.contains(&self.board.get_hash_u128()) {
            return GameState::Repetition;
        }

        if self.fifty_move_counter() >= 50 {
            return GameState::FiftyMove;
        }

        return GameState::Undecided;
    }    
}
