#![ allow(unused)]
use board::Board;
use chess_move::ChessMove;
use attack_board::AttackBoard;
use game::Game;
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
mod game;
mod barsch_bot;

fn main() {
    //benchmark_moves();
    
    //let mut b: Board = Board::start_position();
    //Benchmark fen
    //let mut b: Board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ");
    //let mut game: Game = Game::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ");

    let mut game: Game = Game::from_fen("rnbqkb1r/ppp2ppp/5n2/3pp3/8/4PQP1/PPPP1P1P/RNB1KBNR w KQkq - 0 4");
    let (m, value) = barsch_bot::iterative_deepening(&mut game, 1);
    print!("Best move: ");
    m.print();
    println!(" -> {}", value);



    //b.print_attackers();
    //return;
    //let mut b: Board = Board::start_position();
    //let mut b: Board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/1B1PN3/1p2P3/2N2Q1p/PPPB1PPP/R3K2R b KQkq - 1 1");    
    //let list = b.get_legal_moves();
    //Board::print_moves(&list);

    //println!("{}", b.get_fen());
    //Board::print_moves(&b.get_legal_moves());
    //return;
    //
    //let max_depth = 1;
    //print_tree(b, max_depth, max_depth);


    //b.make_move(&b.get_legal_moves()[47]); 
    //benchmark_moves(b);
    //benchmark_moves_game(&mut game);
}



fn benchmark_moves(b: Board) {
    let mut start = Instant::now();

    Board::print_moves(&b.get_legal_moves());

    for i in 0..100 {

        let mut s = "".to_owned(); 
        let mut res = PerftRes::new();

        let pair = dfs_board(b, i, &mut res);
        //let pair = dfs_fast(b, b.generate_pseudo_legal_moves(), i);
        //fs::write("rust.txt", &s).expect("Unable to write file");

        print!("Depth: {} ", i);
        res.print();
        let duration = start.elapsed();
        println!("{:?}", duration);
    }
}

fn benchmark_moves_game(game: &mut Game) {
    let mut start = Instant::now();

    for i in 0..100 {

        let mut s = "".to_owned(); 
        let mut res = PerftRes::new();

        let pair = dfs_game(game, i, &mut res);
        //let pair = dfs_fast(b, b.generate_pseudo_legal_moves(), i);
        //fs::write("rust.txt", &s).expect("Unable to write file");

        print!("Depth: {} ", i);
        res.print();
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

struct PerftRes {
    pub positions: u64,
    pub captures: u64,
    pub ep: u64,
    pub castles: u64,
    pub promotions: u64,
    pub checks: u64,
    pub double_checks: u64
}
impl PerftRes {
    pub fn new() -> Self {
        return PerftRes { positions: 0, captures: 0, ep: 0, castles: 0, promotions: 0, checks: 0, double_checks: 0 };
    }

    pub fn print(&self) {
        println!("Pos: {} caps: {} ep: {} castles: {} prom: {} checks: {} double checks: {}", self.positions, self.captures, self.ep, self.castles, self.promotions, self.checks, self.double_checks);
    }
}

fn dfs_board(board: Board, depth_left: u8, res: &mut PerftRes) {
    //list.sort_unstable_by(|a, b| { return a.get_uci().cmp(&b.get_uci())});
    if depth_left == 0 {
        res.positions += 1;
        return;
    }
    
    let mut list = board.get_legal_moves();

    for m in list {
        let mut kek = board.clone();
        kek.make_move(&m);

        dfs_board(kek, depth_left - 1, res);
    }
    
}

fn dfs_game(game: &mut Game, depth_left: u8, res: &mut PerftRes) {
    if depth_left == 0 {
        res.positions += 1;
        return;
    }
    
    let list = game.get_legal_moves();

    for m in list {
        game.make_move(m);

        dfs_game(game, depth_left - 1, res);

        game.undo_move();
    }
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