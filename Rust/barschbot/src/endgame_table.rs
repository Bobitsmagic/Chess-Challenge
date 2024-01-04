use core::panic;
use std::{collections::HashMap, fs::File, io::{Write, Read}};

use std::{thread, time};

use crate::{colored_piece_type::ColoredPieceType, piece_type::PieceType, bit_board::BitBoard, square::Square, constants, chess_move };

const MAX_PIECE_COUNT: u8 = 4;

pub fn generate_type_fields(max_piece_count: usize) -> Vec<Vec<[ColoredPieceType; 64]>> {
    assert!(max_piece_count >= 2);

    let mut start_set = Vec::new();

    start_set.push(ColoredPieceType::WhiteKing);
    start_set.push(ColoredPieceType::BlackKing);

    let mut all_set = Vec::new();

    create_piece_types(&mut start_set, 0, max_piece_count, &mut all_set);

    println!("Created sets");

    let mut board = [ColoredPieceType::None; 64];
    let mut board_set = Vec::new();
    let mut sum = 0 as usize;

    for list in &all_set {
        let mut set = Vec::new();
        create_boards(list, 0, 0, &mut board, &mut set);

        for i in 2..list.len() {
            print!("{} ", list[i].get_char());
        }
        println!("-> {}", set.len());

        sum += set.len();

        board_set.push(set);
    }

    println!("PieceCount: {}, KombCount: {}, Board count: {}", max_piece_count, all_set.len(), sum);

    return board_set;

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


pub fn generate_lowest_symmetry(mut type_field: [ColoredPieceType; 64]) -> ([ColoredPieceType; 64], bool) {
    
    let mut pieces = Vec::new();

    for i in 0..64 {
        let pt = type_field[i];

        if pt == ColoredPieceType::None {
            continue;
        }

        pieces.push(pt);
    }

    pieces.sort_unstable_by(|x, y| (*x as u8).cmp(&(*y as u8)));
    let swap = color_swap(&mut type_field, &pieces);

    king_sym(&mut type_field, &pieces);

    return (type_field, swap);

    fn color_swap(type_field: &mut [ColoredPieceType; 64], list: &Vec<ColoredPieceType>) -> bool {
        let mut buffer = [ColoredPieceType::None; 64];

        let mut white_pieces = Vec::new();
        let mut black_pieces = Vec::new();

        for pt in list {
            if pt.is_white() {
                white_pieces.push(*pt);
            }
            else {
                black_pieces.push(*pt);
            }
        }
        
        let mut swap = false;      
        if black_pieces.len() > white_pieces.len() {
            swap = true;
        }
        if black_pieces.len() == white_pieces.len() {
            for i in 0..white_pieces.len() {
                if (PieceType::from_cpt(white_pieces[i]) as u8)
                    < (PieceType::from_cpt(black_pieces[i]) as u8) {
                    swap = true;
                    break;
                }
            }
        }

        if swap {
            for i in 0..64 {
                let pt = type_field[i];
                
                //mirror y axis
                let x = i % 8;
                let y = 7 - (i / 8);
                
                buffer[x + y * 8] = pt.get_opposite_color();
            }

            type_field.copy_from_slice(&buffer);
        }

        return swap;
    }
    
    fn king_sym(type_field: &mut [ColoredPieceType; 64], list: &Vec<ColoredPieceType>) {
        let mut white_king_square = Square::None;

        for i in 0..64 {
            if type_field[i] == ColoredPieceType::WhiteKing {
                white_king_square = Square::from_u8(i as u8);
                break;
            }
        }

        let mut contains_pawns = false;
        for pt in list {
            if PieceType::from_cpt(*pt) == PieceType::Pawn {
                contains_pawns = true;
                break;
            }
        }

        //vertical mirror is always possible
        if white_king_square.file() >= 4 {
            //white_king_square.print();

            let king_x = white_king_square.file();
            let king_y = white_king_square.rank();

            //println!("Flipping vert");
            //BitBoard::print_type_field(type_field);

            white_king_square = Square::from_u8(7 - king_x + king_y * 8);

            for x in 0..4 {
                for y in 0..8 {
                    let index1 = x + y * 8;
                    let index2 = 7 - x + y * 8;

                    let (a, b) = (type_field[index1], type_field[index2]);

                    type_field[index1] = b;
                    type_field[index2] = a;
                }
            }

            //BitBoard::print_type_field(type_field);
        }

        //update king square!!
        if !contains_pawns {
            if white_king_square.rank() >= 4 {
                let king_x = white_king_square.file();
                let king_y = white_king_square.rank();

                white_king_square = Square::from_u8(king_x + (7 - king_y) * 8);

                for x in 0..8 {
                    for y in 0..4 {
                        let (a, b) = (type_field[x + y * 8], type_field[flip_horz_axis(x, y)]);

                        type_field[x + y * 8] = b;
                        type_field[flip_horz_axis(x, y)] = a;
                    }
                }

                fn flip_horz_axis(x: usize, y: usize) -> usize {
                    return x + (7 - y) * 8;
                }
            }

            if white_king_square.rank() > white_king_square.file() {
                let king_x = white_king_square.file();
                let king_y = white_king_square.rank();

                white_king_square = Square::from_u8(king_y * king_x * 8);

                for x in 0..8 {
                    for y in 0..x {
                        let (a, b) = (type_field[x + y * 8], type_field[flip_diagonal_axis(x, y)]);

                        type_field[x + y * 8] = b;
                        type_field[flip_diagonal_axis(x, y)] = a;
                    }
                }

                fn flip_diagonal_axis(x: usize, y: usize) -> usize {
                    return y + x * 8;
                }
            }
        }
    }
}

pub fn check_syms(set: &Vec<Vec<[ColoredPieceType; 64]>>) -> bool {
    for v in set {
        for field in v {
            let mut c = generate_lowest_symmetry((*field).clone());

            for i in 0..64 {
                if c.0[i] != field[i] {
                    return false;
                }
            }
        }
    }

    return true;
}

//2.37 % vs 1%
pub fn gen_legal_boards(set: &Vec<Vec<[ColoredPieceType; 64]>>) -> Vec<Vec<BoardState>> {
 
    /* 
    for i in 0..1 {
        let start = i * 6;
        let end = (i + 1) * 6;

        //set[start..end].par_iter_mut().for_each(|v| {
        set.par_iter_mut().for_each(|v| {
            let mut rem = 0_u64;
            for i in (0..v.len()).rev() {
                let mut board = BitBoard::from_type_field(v[i]);
                if !board.is_valid() {
                    v.remove(i);
                    rem += 1;
                    
                }            
            }
                        
            println!("Removed: {}", rem);
            
        });
    }
    */
     
    let mut list = Vec::new();

    for kek in set {
        let mut local_list = Vec::new();

        for field in kek {
            let v = BoardState::get_all_legal_board_states(*field);

            for bs in v {
                local_list.push(bs);
            }
        }

        let types = get_type_list(kek[0]);
        for i in 0..(types.len() - 2) {
            print!("{} ", types[i].get_char());
        }
        println!(": {}", local_list.len());

        list.push(local_list);
    }

    return list;
}

pub fn get_type_set_index(sorted_set: Vec<ColoredPieceType>) -> u16 {
    let mut value = 0 as u16;

    for i in 0..(sorted_set.len() - 2) {
        value = value * 10 + (sorted_set[i] as u16) * 10;
    }

    return value;
}

pub fn get_type_field_index(type_field: [ColoredPieceType; 64]) -> u16 {
    //slet mut list: ArrayVec<ColoredPieceType, 5> = ArrayVec::new();
    let mut list = Vec::new();

    for cpt in type_field {
        if cpt != ColoredPieceType::None {
            list.push(cpt);
        }
    }

    list.sort_unstable();

    return get_type_set_index(list);
}

pub fn get_type_list(type_field: [ColoredPieceType; 64]) -> Vec<ColoredPieceType> {
    //slet mut list: ArrayVec<ColoredPieceType, 5> = ArrayVec::new();
    let mut list = Vec::new();

    for cpt in type_field {
        if cpt != ColoredPieceType::None {
            list.push(cpt);
        }
    }

    list.sort_unstable();

    return list;
}

#[derive(PartialEq, Eq)]
pub struct BoardState {
    pub type_field: [ColoredPieceType; 64],
    pub ep_square: Square,
    pub whites_turn: bool,
}

impl BoardState {
    pub fn new(type_field: [ColoredPieceType; 64], ep_square: Square, whites_turn: bool) -> BoardState {
        return BoardState { type_field, ep_square, whites_turn };
    }
    pub fn get_all_legal_board_states(type_field: [ColoredPieceType; 64]) -> Vec<BoardState> {
        let mut board = BitBoard::from_type_field(type_field);
        let (white_legal, black_legal) = board.get_valid_mover();
        let mut list = Vec::new();

        if white_legal {
            board.set_whites_turn(true);

            let eps = board.get_valid_ep_squares();
            for square in eps {
                list.push( BoardState::new(type_field, square, true));
            }
        }


        if black_legal {
            board.set_whites_turn(false);

            let eps = board.get_valid_ep_squares();
            for square in eps {
                list.push(BoardState::new(type_field, square, false));
            }
        }

        return list; 
    }

    pub fn get_lowest_symmetry(&self) -> BoardState {
        let (field, swap) = generate_lowest_symmetry(self.type_field);

        let mut square = self.ep_square;

        if swap && square != Square::None {
            let x = square.file();
            let y = square.rank();

            square = Square::from_u8(x + (7 - y) * 8);
        }

        return BoardState::new(field, square, self.whites_turn != swap);
    }
}

pub fn is_insufficient_material(list: &Vec<ColoredPieceType>) -> bool {
    if list.len() == 2 {
        return true;
    }

    if list.len() == 3 {
        for pt in list {
            match PieceType::from_cpt(*pt) {
                PieceType::Bishop => return true,
                PieceType::Knight => return true,
                _ => (),
            }
        }
    }

    return false;
}

pub const UNDEFINED: i8 = i8::MIN;
pub const WHITE_CHECKMATE: i8 = -127;
pub const BLACK_CHECKMATE: i8 = 127;
pub const DRAW: i8 = 0;
pub struct EndgameTable {
    table_map: HashMap<u64, i8>,
    pub max_piece_count: u8
}



impl EndgameTable {
    pub fn new(sorted_positions: &Vec<Vec<BoardState>>) -> EndgameTable {
        let mut table_map = HashMap::new(); 
        
        for set in sorted_positions {
            let mut check_mate_count = 0_u64;
            let mut draw_count = 0;
            
            let types = get_type_list(set[0].type_field);
            for i in 0..(types.len() - 2) {
                print!("{} ", types[i].get_char());
            }
            println!();

            for bs in set {
                let board = BitBoard::from_board_state(bs);
                
                let mut val = UNDEFINED;
                
                if is_insufficient_material(&types) {
                    val = DRAW;
                }
                else {
                    let moves = board.get_legal_moves();
                    if moves.len() == 0 {  
                        if board.in_check() {
                            val = if board.is_whites_turn() { WHITE_CHECKMATE } else { BLACK_CHECKMATE };
                            check_mate_count += 1;
                        }
                        else {
                            val = DRAW;
                            draw_count += 1;
                        }

                    }
                }

                table_map.insert(board.get_zoberist_hash(), val);
            }

            println!("Init cm: {}, draw: {}", check_mate_count, draw_count);
        }

        println!("Finished init");
        wait();

        let mut new_mate_counter = 1_u64;
        let mut max_check = 0;

        while new_mate_counter > 0 {    
            println!("New mate counter: {}", new_mate_counter);
            new_mate_counter = 0;
            for set in sorted_positions {
                let types = get_type_list(set[0].type_field);
                //for i in 0..(types.len() - 2) {
                //    print!("{} ", types[i].get_char());
                //}
                //println!();

                for bs in set {
                    let board = BitBoard::from_board_state(bs);

                    //println!("######################## Board: ");
                    //board.print();

                    let board_hash = board.get_zoberist_hash();
                    let val =  table_map[&board_hash];

                    let moves = board.get_legal_moves();

                    //BitBoard::print_moves(&moves);
                    //println!("Initial eval: {}", val);

                    if moves.len() == 0 {
                        continue;
                    }

                    let mut best_score = if board.is_whites_turn() { WHITE_CHECKMATE } else { BLACK_CHECKMATE };
                    
                    let mut all_defined = true;
                    let mut best_move = chess_move::NULL_MOVE;

                    for m in moves {  
                        let mut buffer = board.clone();
                        buffer.make_move(m);

                        let sym = buffer.get_board_state().get_lowest_symmetry();                       
                        buffer = BitBoard::from_board_state(&sym);
                        
                        let mut s = table_map[&buffer.get_zoberist_hash()];

                        if s == UNDEFINED {
                            all_defined = false;
                            //println!("Skipping");
                            continue;
                        }
                            
                        if bs.whites_turn == buffer.is_whites_turn() {
                            s = -s;
                        }


                        s -= s.signum();

                        if board.is_whites_turn() {
                            if s >= best_score {
                                //println!("Eval: {}", val);
                                //m.print();
                                //println!("-> {}", s);
                                //buffer.print();
                                //wait();

                                best_score = s;
                                best_move = m;
                            }
                        }
                        else {
                            if s <= best_score {
                                //println!("Eval: {}", val);
                                //m.print();
                                //println!("-> {}", s);
                                //buffer.print();
                                //wait();

                                best_score = s;
                                best_move = m;

                            }
                        }
                    }

                    if !all_defined {
                        if board.is_whites_turn() {
                            if best_score < 0 {
                                best_score = UNDEFINED;
                            }
                        }
                        else {
                            if best_score > 0 {
                                best_score = UNDEFINED;
                            }
                        }
                    }
                    
                    if best_score != val && best_score != UNDEFINED {
                        //if val != UNDEFINED {
                        //    println!("Found better cm {} -> {}", val, best_score);
                        //    board.print();
//
                        //    best_move.print();
                        //    println!();
//
                        //    wait();
                        //}                
                        
                        new_mate_counter += 1;
                        table_map.insert(board_hash, best_score);
                        
                        if best_score != DRAW {
                            
                            let b = 127 - best_score.abs();
                            if b > max_check {
                                println!("New Max: {}", b);
                                best_move.print();
                                println!();
                                board.print();
                                
                                max_check = b;
                            }
                        }
                    }
                }
            
                //println!("Finished set, max check: {}", max_check);
            }
        }
        
        return EndgameTable { table_map, max_piece_count: MAX_PIECE_COUNT };

        fn wait() {
            let ten_millis = time::Duration::from_millis(1000);
            let now = time::Instant::now();

            thread::sleep(ten_millis);
        }
    }

    pub fn store_data(&self) {
        let mut file = File::create("table_base.bin").unwrap();
        
        let mut buffer = Vec::with_capacity(self.table_map.len()); 

        for pair in &self.table_map {
            let bytes = pair.0.to_be_bytes();
            for b in bytes {
                buffer.push(b);
            }
            
            buffer.push(pair.1.to_be_bytes()[0]);
        }
        
        // Write a slice of bytes to the file
        file.write_all(&buffer).unwrap();
    }

    pub fn load(mut max_piece_count: u8) -> Self {
        if max_piece_count < 3 {
            return EndgameTable { table_map: HashMap::new(), max_piece_count: 0};
        }

        if max_piece_count > 4 {
            panic!("I cant handle this anymore");
        }


        let mut file = File::open("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\barschbot\\table_base_".to_owned() + &max_piece_count.to_string().to_owned() + ".bin").unwrap();
        // read the same file back into a Vec of bytes
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).unwrap();

        let count = buffer.len() / 9;

        if buffer.len() % 9 != 0 {
            panic!("Kekeke");
        }

        let mut table_map = HashMap::with_capacity(count);
        for i in 0..count {
            let start_index = i * 9;
            let mut bytes = [0_u8; 8];
            bytes.copy_from_slice(&buffer[start_index..(start_index + 8)]);

            let hash = u64::from_be_bytes(bytes);
            let score = i8::from_be_bytes([buffer[start_index + 8]; 1]);

            table_map.insert(hash, score);

            if i % 10000000 == 0 {
                println!("{} / {}", i, count);
            }
        }

        return EndgameTable { table_map, max_piece_count };        
    }

    pub fn get_score(&self, board: &BitBoard) -> i8 {
        let sym = BitBoard::from_board_state(& board.get_board_state().get_lowest_symmetry());      

        let hash = sym.get_zoberist_hash();

        if !self.table_map.contains_key(&hash) {
            //board.print();

            return 0;
        }
        let mut s = self.table_map[&hash];

        if s == UNDEFINED {
            println!("Undefined: ");
            board.print();

            return 0;
        }

        if sym.is_whites_turn() != board.is_whites_turn() {
            s = -s;
        }

        return s;
    }
}