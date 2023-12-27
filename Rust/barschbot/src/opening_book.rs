use std::{collections::HashMap, fs::{File, read_to_string}, io::BufReader};

use crate::{chess_move::{ChessMove, NULL_MOVE}, bit_board::BitBoard};

pub struct OpeningBook {
    moves: HashMap<u64, ChessMove>,
}

impl OpeningBook {
    pub fn new() -> OpeningBook {
        OpeningBook {
            moves: HashMap::new(),
        }
    }

    pub fn load_from_file(path: &str) -> OpeningBook {
        let mut moves: HashMap<u64, ChessMove> = HashMap::new();

        for line in read_to_string(path).unwrap().lines() {
            let parts = line.split(",").collect::<Vec<_>>();

            let board = BitBoard::from_fen(parts[0]);
            let hash = board.get_zoberist_hash();
            
            let list = board.get_legal_moves();

            let m = list.iter().filter(|m| m.get_board_name(&board) == parts[1]).collect::<Vec<_>>()[0];

            board.print();
            println!("Book move {}", m.get_board_name(&board));

            moves.insert(hash, *m);
        }

        return OpeningBook {
            moves
        }
    }

    pub fn get_move(&self, hash: u64) -> ChessMove {
        if self.moves.contains_key(&hash) {
            return self.moves.get(&hash).unwrap().clone();
        }

        return NULL_MOVE;
    }
}