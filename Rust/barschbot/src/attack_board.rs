use std::cmp;

use crate::{constants, chess_move::ChessMove};

#[derive(Clone, Copy)]
struct PieceList {
    count: u8,
    is_free: u16,
    types: [u8; 16],
    map:   [u8; 16],
    move_count: [u8; 16],
    field: [u8; 64],
}

const SLIDING_DIRECTIONS: [&[(i8, i8)]; 3] = [
    &[(1,1), (-1, -1), (1, -1), (-1, 1)],
    &[(1,0), (-1, 0), (0, 1), (0, -1)],
    &[(1,0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, - 1), (-1, 1)],
];

impl PieceList {
    pub fn empty() -> Self {
        return PieceList { count: 0, is_free: u16::MAX, types: [constants::NULL_PIECE; 16], map: [255; 16], field:  [255; 64], move_count: [0; 16]  }
    }

    pub fn add_new_piece(&mut self, piece: u8, square: u8) -> u8 {
        //println!("{:b}", self.is_free);

        let index = self.is_free.trailing_zeros() as u8;
        //println!("Adding to piecelist {} at {} index: {}" , constants::PIECE_CHAR[piece as usize], constants::SQUARE_NAME[square as usize], index);
        self.is_free &= !(1_u16 << index);

        self.move_count[index as usize] = 0;
        self.types[index as usize] = piece;
        self.map[index as usize] = square;
        self.field[square as usize] = index;
        self.count += 1;

        return index;
    }

    pub fn remove_piece(&mut self, square: u8) {
        let index = self.field[square as usize];

        //println!("Removing from piecelist {} at {}" , index, constants::SQUARE_NAME[square as usize]);


        self.field[square as usize] = 255;
        self.types[index as usize] = constants::NULL_PIECE;
        self.map[index as usize] = 255;
        self.move_count[index as usize] = 0;
        self.count -= 1;

        self.is_free |= 1_u16 << index;
    }

    pub fn add_to_move_count(&mut self, index: u8, value: u8) {
        self.move_count[index as usize] += value;
    }

    pub fn remove_from_move_count(&mut self, index: u8, value: u8) {
        self.move_count[index as usize] -= value;
    }

    pub fn get_move_count(&self, square: u8) -> u8 {
        return self.move_count[self.field[square as usize] as usize];
    }
    pub fn get_piece_square(&self, index: u8) -> u8 {
        return self.map[index as usize];
    }

    pub fn get_piece_type(&self, index: u8) -> u8 {
        return self.types[index as usize];
    }

    pub fn get_piece_index(&self, square: u8) -> u8 {
        return self.field[square as usize];
    }
}

#[derive(Clone, Copy)]
pub struct AttackBoard {
    white_piece_list: PieceList,
    black_piece_list: PieceList,
    white_targets: [u16; 64],      //square -> white piece bool array
    black_targets: [u16; 64],      //square -> black piece bool array
}

impl AttackBoard {
    pub fn empty() -> Self {
        return AttackBoard { 
            white_piece_list: PieceList::empty(), 
            black_piece_list: PieceList::empty(), 
            white_targets: [0; 64], black_targets: [0; 64] }
    }

    pub fn add_at_square(&mut self, square: u8, piece_type: u8, piece_field: &[u8; 64]) {
        debug_assert!(square < 64);
        debug_assert!(piece_type != constants::NULL_PIECE);
        
        //add to field
        let is_white = (piece_type & 1) == 0;
        let piece_list = if is_white { &mut self.white_piece_list } else { &mut self.black_piece_list };
        let index = piece_list.add_new_piece(piece_type, square);
        
        let mut target_field = if is_white { &mut self.white_targets } else {&mut self.black_targets };
        let flag = 1_u16 << index;

        let x = square % 8;
        let y = square / 8;
        
        
        //println!("Adding to attack board {} at {}" , constants::PIECE_CHAR[piece_type as usize], constants::SQUARE_NAME[square as usize]);
        //AttackBoard::print_flags(&target_field, flag);
        
        //add piece attacks
        match piece_type >> 1 {
            constants::KING => {
                piece_list.add_to_move_count(index, constants::KING_MOVES[square as usize].len() as u8);
                for target_square in constants::KING_MOVES[square as usize] {                      
                    target_field[*target_square as usize] |= flag;
                }
            },
            constants::KNIGHT => {
                piece_list.add_to_move_count(index, constants::KNIGHT_MOVES[square as usize].len() as u8);
                for target_square in constants::KNIGHT_MOVES[square as usize] {                      
                    target_field[*target_square as usize] |= flag;
                }
            }
            constants::PAWN => {    
                debug_assert!(y > 0 && y < 7);

                let pawn_direction: i32 = if is_white { 1 } else { -1 };

                if y > 0 && y < 7 {
                    //capture left
                    if x > 0 {
                        let capture_square = (square as i32 + 8 * pawn_direction - 1) as u8;
                        target_field[capture_square as usize] |= flag;
                    }
                    //capture right
                    if x < 7 {
                        let capture_square = (square as i32 + 8 * pawn_direction + 1) as u8;
                        target_field[capture_square as usize] |= flag;
                    }
                }       
            }
            //slider (Bishop, Rook, Queen)
            _ => {
                
                //[TODO] faster with direct squares indices?
                for dir in SLIDING_DIRECTIONS[((piece_type >> 1) - constants::BISHOP) as usize] {
                    let mut dx = x as i8 + dir.0;
                    let mut dy = y as i8 + dir.1;

                    while dx >= 0 && dx < 8 && dy >= 0 && dy < 8 {
                        let target_square = dx + dy * 8;
                        
                        target_field[target_square as usize] |= flag;
                        //println!("Target field: {:?}", target_field);
                        piece_list.add_to_move_count(index, 1);

                        if piece_field[target_square as usize] != constants::NULL_PIECE{
                            break;
                        }

                        dx += dir.0;
                        dy += dir.1;
                    }
                }
            }
        }
        
        //println!("white");
        //updates all attackers
        block_targets(&mut self.white_piece_list, &mut self.white_targets, square, piece_field);

        //println!("Black");
        block_targets(&mut self.black_piece_list, &mut self.black_targets, square, piece_field);

        fn block_targets(piece_list: &mut PieceList, targets: &mut [u16; 64], block_square: u8, piece_field: &[u8; 64]) {
            let block_x = block_square % 8;
            let block_y = block_square / 8;
    
            let flag_list = targets[block_square as usize];
            if flag_list == 0 {
                return;
            }
    
            for piece_index in 0..16 {
                let flag = 1 << piece_index;
                if flag_list & flag != 0 {
                    let piece_type = piece_list.get_piece_type(piece_index);
                    let piece_square = piece_list.get_piece_square(piece_index);
                    let piece_x = piece_square % 8;
                    let piece_y = piece_square / 8;

                    //println!("Blocking: {} at {}", constants::PIECE_CHAR[piece_type as usize], constants::SQUARE_NAME[piece_square as usize]);
                    //AttackBoard::print_flags(&targets, flag);

                    match piece_type >> 1 {
                        constants::KING => (),
                        constants::KNIGHT => (),
                        constants::PAWN => (),
                        //slider (Bishop, Rook, Queen)
                        _ => {
                            let mut dx = block_x as i8 - piece_x as i8;
                            let mut dy = block_y as i8 - piece_y as i8;
                            let max = cmp::max(dx.abs(), dy.abs());
                            
                            //println!("block: {},{}, piece {}, {}", block_x, block_y, piece_x, piece_y);

                            dx /= max;
                            dy /= max;
    
                            let mut tx = block_x as i8 + dx;
                            let mut ty = block_y as i8 + dy;

                            //println!("Direction: {}, {}", dx, dy);
    
                            //block all moves after behind blocker
                            while tx >= 0 && tx < 8 && ty >= 0 && ty < 8 {
                                let target_square = tx + ty * 8;
                                
                                targets[target_square as usize] &= !flag;

                                piece_list.remove_from_move_count(piece_index, 1);

                                if piece_field[target_square as usize] != constants::NULL_PIECE{
                                    break;
                                }

                                //println!("{:?}", targets);
                                tx += dx;
                                ty += dy;
                            }
                        }
                    }
                }
            }
        }
    }
    
    //[TODO] unblock
    pub fn remove_at_square(&mut self, square: u8, piece_field: &[u8; 64]) -> u8 {
        debug_assert!(square < 64);

        let is_white = piece_field[square as usize] & 1 == 0;

        let mut piece_list = if is_white { &mut self.white_piece_list } else { &mut self.black_piece_list };
        let mut target_field = if is_white { &mut self.white_targets } else { &mut self.black_targets };

        let index = piece_list.get_piece_index(square);
        let piece_type = piece_list.get_piece_type(index);
        let flag = 1_u16 << index;

        piece_list.remove_piece(square);

        //println!("Removing from attack board {} at {} index {}" , constants::PIECE_CHAR[piece_type as usize], constants::SQUARE_NAME[square as usize], index);
        //AttackBoard::print_flags(&target_field, flag);

        let x = square % 8;
        let y = square / 8;

        //remove piece attacks
        match piece_type >> 1 {
            constants::KING => {
                for target_square in constants::KING_MOVES[square as usize] {                      
                    target_field[*target_square as usize] &= !flag;
                }
            },
            constants::KNIGHT => {
                for target_square in constants::KNIGHT_MOVES[square as usize] {                      
                    target_field[*target_square as usize] &= !flag;
                }
            }
            constants::PAWN => {    
                let pawn_direction: i32 = if is_white { 1 } else { -1 };
                
                if y > 0 && y < 7 {
                    //capture left
                    if x > 0 {
                        let capture_square = (square as i32 + 8 * pawn_direction - 1) as u8;
                        target_field[capture_square as usize] &= !flag;
                    }
                    //capture right
                    if x < 7 {
                        let capture_square = (square as i32 + 8 * pawn_direction + 1) as u8;
                        target_field[capture_square as usize] &= !flag;
                    }
                }
            }
            //slider (Bishop, Rook, Queen)
            _ => {
                //[TODO] faster with direct squares indices?
                for dir in SLIDING_DIRECTIONS[((piece_type >> 1) - constants::BISHOP) as usize] {
                    let mut dx = x as i8 + dir.0;
                    let mut dy = y as i8 + dir.1;
                    
                    //println!("Old");
                    //AttackBoard::print_flags(&target_field, flag);

                    while dx >= 0 && dx < 8 && dy >= 0 && dy < 8 {
                        let target_square = dx + dy * 8;
                        
                        target_field[target_square as usize] &= !flag;
                            
                        if piece_field[target_square as usize] != constants::NULL_PIECE {
                            break;
                        }

                        dx += dir.0;
                        dy += dir.1;
                    }

                    //println!("new");
                    //AttackBoard::print_flags(&target_field, flag);
                }
            }
        }
        
        //println!("After removal");
        //AttackBoard::print_flags(&target_field, flag);

        //updates all attackers
        unblock_targets(&mut self.white_piece_list, &mut self.white_targets, square, piece_field);
        unblock_targets(&mut self.black_piece_list, &mut self.black_targets, square, piece_field);

        return piece_type;

        fn unblock_targets(piece_list: &mut PieceList, targets: &mut [u16; 64], block_square: u8, piece_field: &[u8; 64]) {
            let block_x = block_square % 8;
            let block_y = block_square / 8;
    
            let flag_list = targets[block_square as usize];
            if flag_list == 0 {
                return;
            }
    
            for piece_index in 0..16 {
                let flag = 1 << piece_index;
                if flag_list & flag != 0 {
                    let piece_type = piece_list.get_piece_type(piece_index);
                    let piece_square = piece_list.get_piece_square(piece_index);
                    let piece_x = piece_square % 8;
                    let piece_y = piece_square / 8;
                    //println!("Unblocking: {} at {}", constants::PIECE_CHAR[piece_type as usize], constants::SQUARE_NAME[piece_square as usize]);
                    
                    match piece_type >> 1 {
                        constants::KING => (),
                        constants::KNIGHT => (),
                        constants::PAWN => (),
                        //slider (Bishop, Rook, Queen)
                        _ => {
                            let mut dx = block_x as i8 - piece_x as i8;
                            let mut dy = block_y as i8 - piece_y as i8;
                            let max = cmp::max(dx.abs(), dy.abs());
                            
                            dx /= max;
                            dy /= max;
                            
                            let mut tx = block_x as i8 + dx;
                            let mut ty = block_y as i8 + dy;
                            
                            //block all moves after behind blocker
                            while tx >= 0 && tx < 8 && ty >= 0 && ty < 8 {
                                let target_square = tx + ty * 8;
                                
                                targets[target_square as usize] |= flag;
                                
                                piece_list.add_to_move_count(piece_index, 1);

                                if piece_field[target_square as usize] != constants::NULL_PIECE {
                                    break;
                                }

                                tx += dx;
                                ty += dy;
                            }

                            //AttackBoard::print_flags(&targets, flag);
                        }
                    }
                }
            }
        }
    }

    pub fn get_all_attacks(&self, whites_turn: bool) -> Vec<(u8, u8)> {
        let mut list:  Vec<(u8, u8)> =Vec::new();
        
        let mut piece_list = if whites_turn { self.white_piece_list } else { self.black_piece_list };
        let mut targets = if whites_turn { self.white_targets } else { self.black_targets };

        //println!("White: {:?}", self.white_targets);
        //println!("Black: {:?}", self.black_targets);        
        //Self::print_flags(targets);
        
        for s in 0..64 {
            let flag_list = targets[s as usize];
            if flag_list == 0 {
                continue
            }

            for piece_index in 0..16 {
                let flag = 1 << piece_index;
                if flag_list & flag != 0 { 
                    list.push((piece_list.get_piece_square(piece_index), s))
                }
            }
        }

        return list;
    }

    pub fn square_is_attacked(&self, whites_turn: bool, square: u8) -> bool {
        if whites_turn {
            return self.white_targets[square as usize] != 0;
        }
        else {
            return self.black_targets[square as usize] != 0; 
        }
    }

    pub fn square_attack_count(&self, whites_turn: bool, square: u8) -> u8 {
        if whites_turn {
            return self.white_targets[square as usize].count_ones() as u8;
        }
        else {
            return self.black_targets[square as usize].count_ones() as u8; 
        }
    }

    pub fn piece_attack_move_count(&self, is_whites_piece: bool, square: u8) -> u8 {
        if is_whites_piece {
            return self.white_piece_list.get_move_count(square);
        }
        else {
            return self.black_piece_list.get_move_count(square);
        }
    }

    //For legality check only !!!!!!!! does not work on castles
    pub fn make_move_for_legallity_check(&mut self, m: ChessMove, piece_field: &[u8; 64]) {
        if m.is_en_passant {
            let pawn_direction: i32 = if m.is_white_move() { 1 } else { -1 };
            self.remove_at_square((m.target_square as i8 - pawn_direction as i8 * 8) as u8, piece_field);
        }
        else { //Normal move
            if m.is_direct_capture() {
                self.remove_at_square(m.target_square, piece_field);
            }
        }

        let pt = self.remove_at_square(m.start_square, piece_field);
        self.add_at_square(m.target_square, pt, piece_field);
    }


    pub fn print_square_attacker(&self, square: u8, piece_type: u8) {
        let is_white = piece_type & 1 == 0;

        let mut piece_list = if is_white { self.white_piece_list } else { self.black_piece_list };
        let mut target_field = if is_white { self.white_targets } else { self.black_targets };

        println!("{} at {}", constants::PIECE_CHAR[piece_type as usize], constants::SQUARE_NAME[square as usize]);
        AttackBoard::print_flags(&target_field, 1u16 << piece_list.get_piece_index(square));
    }
    pub fn print_flags(field: &[u16; 64], flag: u16) {
        println!("Index: {}", flag.leading_zeros());
        println!("_____________");
        for y in (0..8).rev() {
            for x in 0..8 {
                let kek = (field[(x + y * 8) as usize] & flag).leading_zeros();
                print!("{} ", if kek == 16 {" "} else { "x" });
            }
            println!();
        }
        println!("_____________");
    }
}