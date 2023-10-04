#![ allow(unused)]
use board::Board;
use chess_move::ChessMove;
use attack_board::AttackBoard;
use std::sync::atomic::AtomicU64;
use std::time::{Instant, Duration};
use std::fs;
mod zoberist_hash;

mod bitboard_helper;
mod attack_board;
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

    //b.print_attackers();
    //return;
    //let mut b: Board = Board::start_position();
    //let mut b: Board = Board::from_fen("r3k2r/p1p1Npb1/bn1ppnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 2");    

    //println!("{}", b.get_fen());
    //Board::print_moves(&b.get_legal_moves());
    //return;
    //
    //let max_depth = 1;
    //print_tree(b, max_depth, max_depth);


    //b.make_move(&b.get_legal_moves()[47]); 
    benchmark_moves(b);
}



fn benchmark_moves(b: Board) {
    let mut start = Instant::now();

    Board::print_moves(&b.get_legal_moves());

    for i in 0..100 {

        let mut s = "".to_owned();

        let pair = dfs(b, i);
        //let pair = dfs_fast(b, b.generate_pseudo_legal_moves(), i);
        //fs::write("rust.txt", &s).expect("Unable to write file");

        println!("Depth: {} -> Nodes: {:010}\t Captures: {}", i, pair.0, pair.1);
        let duration = start.elapsed();
        println!("{:?}", duration);
    }
}

fn print_tree(board: Board, depth_left: u8, max_depth: u8) {
    let mut list = board.get_legal_moves();
    list.sort_unstable_by(|a, b| { return a.get_uci().cmp(&b.get_uci())});

    if depth_left == 0 {
        print_prefix(max_depth - depth_left);

        print!("{}[", list.len());

        for m in list {
            m.print_uci();

            print!(" ");
        }

        println!("]");

        return;
    }    

    print_prefix(max_depth - depth_left);
    println!("{}[", list.len());

    for m in list {
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