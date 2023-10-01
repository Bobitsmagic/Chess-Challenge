#![ allow(unused)]
use board::Board;
use std::time::{Instant, Duration};

mod zoberist_hash;

mod bitboard_helper;
mod board;
mod piece_type;
mod uci_move;
mod piece_list;
mod chess_move;


fn main() {
    benchmark_moves();
    
    let mut b: Board = Board::start_position();

    b.print();
    b.make_move(&b.get_legal_moves()[13]);
    b.print();
    b.make_move(&b.get_legal_moves()[4]);
    b.print();
    b.make_move(&b.get_legal_moves()[13]);
    b.print();
    b.make_move(&b.get_legal_moves()[9]);
    b.print();

    println!("Hash: {}", b.gen_zoberist_hash());
    
    Board::print_moves(b.get_legal_moves());
    
}

fn benchmark_moves() {
    let mut b: Board = Board::start_position();

    let mut start = Instant::now();
    for i in 0..10 {
        println!("Depth: {} -> {}", i, dfs(b, i));
        let duration = start.elapsed();
        println!("{:?}", duration);
    }
}

fn dfs(board: Board, depth_left: u8) -> u64 {
    if depth_left == 0 {
        return 1;
    }

    let mut sum = 0 as u64;
    for m in board.get_legal_moves() {
        let mut kek = board.clone();
        kek.make_move(&m);
        sum += dfs(kek, depth_left - 1);
    }

    return sum;
}
