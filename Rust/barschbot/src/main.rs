#![ allow(unused)]
use board::Board;
use chess_move::ChessMove;
use std::sync::atomic::AtomicU64;
use std::time::{Instant, Duration};
use std::fs;
mod zoberist_hash;

mod bitboard_helper;
mod board;
mod constants;
mod uci_move;
mod piece_list;
mod chess_move;


fn main() {
    //benchmark_moves();
    
    //let mut b: Board = Board::start_position();

    //Benchmark fen
    let mut b: Board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ");

    //let mut b: Board = Board::from_fen("r3k2r/p1ppqPb1/1n3np1/1b2N3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1");    
   
    //println!("{}", b.get_fen());
    //let max_depth = 3;
    //print_tree(b, max_depth, max_depth);

    //b.print();
    //Board::print_moves(&b.get_legal_moves());

    //b.make_move(&b.get_legal_moves()[47]); 
    benchmark_moves(b);
}

fn benchmark_moves(b: Board) {
    let mut start = Instant::now();

    Board::print_moves(&b.get_legal_moves());
    unsafe {
        board::MOVE_GEN_COUNTER = AtomicU64::new(0);;
        board::MAKE_MOVE_COUNTER = AtomicU64::new(0);;
    }
    for i in 0..10 {

        let mut s = "".to_owned();

        let pair = dfs(b, i);
        //let pair = dfs_fast(b, b.generate_pseudo_legal_moves(), i);
        //fs::write("rust.txt", &s).expect("Unable to write file");

        println!("Depth: {} -> Nodes: {:010}\t Captures: {}", i, pair.0, pair.1);
        let duration = start.elapsed();
        println!("{:?}", duration);

        unsafe {
            println!("MoveGens: {:?}, MakeMove: {:?}", board::MOVE_GEN_COUNTER, board::MAKE_MOVE_COUNTER);
        }
    }
}

fn print_tree(board: Board, depth_left: u8, max_depth: u8) {
    let mut list = board.get_legal_moves();
    list.sort_unstable_by(|a, b| { return a.get_uci().cmp(&b.get_uci())});

    if depth_left == 0 {
        print_prefix(max_depth - depth_left);

        print!("{}[", list.len());

        for m in list {
            if !m.is_castle() {
                continue;
            }

            m.print_uci();

            print!(" ");
        }

        println!("]");

        return;
    }    

    print_prefix(max_depth - depth_left);
    println!("{}[", list.len());

    for m in list {
        if depth_left == 1 && !m.is_castle() {
            continue;
        }

        let mut kek = board.clone();
        kek.make_move(&m);
        
        print_prefix(max_depth - depth_left);
        


        print!("\t");
        m.print_uci();

        print_tree(kek, depth_left - 1, max_depth);
    }

    print_prefix(max_depth - depth_left);
    println!("]");

    fn print_prefix(depth: u8) {
        for i in 0..depth {
            print!("\t");
        }
    }
}

fn dfs(board: Board, depth_left: u8) -> (u64, u64) {
    // s: &mut String
    if depth_left == 0 {
        return (1, 0);
    }

    let mut pos_count = 0_u64;
    let mut capture_count = 0 as u64;
    let mut list = board.get_legal_moves();
    //list.sort_unstable_by(|a, b| { return a.get_uci().cmp(&b.get_uci())});

    if depth_left == 1 {
        pos_count += list.len() as u64;

        for m in list {
            if (m.is_capture() || m.is_en_passant) {
                capture_count += 1;
            }
        }
    }
    else {
        for m in list {
            let mut kek = board.clone();
            kek.make_move(&m);

            let pair = dfs(kek, depth_left - 1);
            pos_count += pair.0;
            capture_count += pair.1;
        }
    }


    return (pos_count, capture_count);
}

fn dfs_fast(board: Board, pseudo_moves: Vec<ChessMove>, depth_left: u8) -> (u64, u64) {
    // s: &mut String
    if depth_left == 0 {
        return (1, 0);
    }

    let mut pos_count = 0_u64;
    let mut capture_count = 0 as u64;


    if depth_left == 1 {
        let mut list = board.get_legal_moves();
        
        pos_count += list.len() as u64;            
        for m in list {
            if (m.is_capture() || m.is_en_passant) {
                capture_count += 1;
                
            }
        }
    }
    else {
        let mut list = board.get_legel_boards(pseudo_moves);
    
        for (m, next_board, next_pseudo) in list {    
            let pair = dfs_fast(next_board, next_pseudo, depth_left - 1);
    
            pos_count += pair.0;
            capture_count += pair.1;
        }
    }

    //list.sort_unstable_by(|a, b| { return a.get_uci().cmp(&b.get_uci())});


    return (pos_count, capture_count);
}

pub fn print_int(value: u64, max_digits: u8) {
    let length = value.to_string().len();
    
    
    for i in 0..(max_digits as usize - length) {
        print!(" ");
        if (i + length) % 3 == 0 {
            print!(" ");
        }
    }
    
    let mut count = length % 3;
    for c in value.to_string().chars() {
        print!("{}", c);

        count += 1;

        if count % 3 == 0 {
            print!(" ");
        }

    }
}