use graphics::{types::Color, modular_index::next};
use piston::controller;

use crate::{colored_piece_type::{ColoredPieceType, self}, piece_type::PieceType, bit_board::BitBoard, square::{Square, self}, constants, zoberist_hash};

pub struct EndgameTable {
    sorted_positions: Vec<Vec<i8>>
}

impl EndgameTable {
    pub fn generate(max_piece_count: usize) {
        assert!(max_piece_count >= 2);

        let mut start_set = Vec::new();

        start_set.push(ColoredPieceType::WhiteKing);
        start_set.push(ColoredPieceType::BlackKing);

        let mut all_set = Vec::new();

        create_piece_types(&mut start_set, 0, max_piece_count, &mut all_set);

        println!("Created sets");

        let mut board = [ColoredPieceType::None; 64];
        //let mut board_set = Vec::new();
        let mut sum = 0 as usize;

        for list in &all_set {
            let mut set = Vec::new();
            create_boards(list, 0, 0, &mut board, &mut set);

            for cpt in list {
                print!("{} ", cpt.get_char());
            }
            println!("-> {}", set.len());

            sum += set.len();

            //board_set.push(set);
        }

        println!("PieceCount: {}, KombCount: {}, Board count: {}", max_piece_count, all_set.len(), sum);

        fn create_boards(list: &Vec<ColoredPieceType>, depth: usize, min_pos: usize, board: &mut [ColoredPieceType; 64], ret: &mut Vec<[ColoredPieceType; 64]>) {
            if depth == list.len() {
                ret.push(board.clone());
                return;
            }

            let pt = list[depth];
            for i in min_pos..64 {
                if board[i] != ColoredPieceType::None {
                    continue;
                }

                if PieceType::from_cpt(pt) == PieceType::Pawn {
                    if i < 8 || i >= 64 - 8 {
                        continue;
                    }
                }

                if pt == ColoredPieceType::BlackKing {
                    let mut collision = false;
                    for s in constants::KING_MOVES[i] {
                        if board[*s as usize] == ColoredPieceType::WhiteKing {
                            collision = true;
                            break;
                        }
                    }

                    if collision {
                        continue;
                    }
                }

                if pt == ColoredPieceType::WhiteKing {
                    let mut contains_pawn = false;
                    for other in list {
                        if PieceType::from_cpt(*other) == PieceType::Pawn {
                            contains_pawn = true;
                            break;
                        }
                    }

                    let square = Square::from_u8(i as u8);
                    if contains_pawn {
                        if square.file() >= 4 {
                            continue;
                        }
                    }
                    else {
                        if square.file() >= 4 || square.rank() >= 4 || square.rank() > square.file() {
                            continue;
                        }
                    }
                }

                board[i] = pt;

                let mut next_min_pos = 0;
                if depth < list.len() - 1 {
                    if list[depth + 1] == pt {
                        next_min_pos = i + 1;
                    }
                }
                
                create_boards(list,  depth + 1,  next_min_pos, board, ret);

                board[i] = ColoredPieceType::None;
            }
        }

        fn create_piece_types(list: &mut Vec<ColoredPieceType>, min_piece_type: u8, max_piece_count: usize, ret: &mut Vec<Vec<ColoredPieceType>>) {
            if list.len() == max_piece_count {

                let mut vec = Vec::with_capacity(list.len());


                let mut white_pieces = Vec::new();
                let mut black_pieces = Vec::new();

                for pt in list {
                    if *pt == ColoredPieceType::None {
                        break;
                    } 

                    vec.push(*pt);

                    if pt.is_white() {
                        white_pieces.push(*pt);
                    }
                    else {
                        black_pieces.push(*pt);
                    }
                }

                //white is always the one with equal more pieces
                if black_pieces.len() > white_pieces.len() {
                    return;
                }

                //white has the more valueable pieces
                if black_pieces.len() == white_pieces.len() {
                    for i in 0..white_pieces.len() {
                        if (PieceType::from_cpt(white_pieces[i]) as u8)
                            < (PieceType::from_cpt(black_pieces[i]) as u8) {
                            return;
                        }
                    }
                }

                ret.push(vec);

                return;
            }

            for i in min_piece_type..13 {    
                let cpt = ColoredPieceType::from_u8(i);

                if PieceType::from_cpt(cpt) == PieceType::King {
                    continue;
                }

                list.push(cpt);

                create_piece_types(list, i, max_piece_count, ret);

                list.pop();
            }
        }
    }
}