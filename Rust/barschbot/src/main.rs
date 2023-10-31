#![ allow(unused)]
use bit_board::BitBoard;
use chess_move::ChessMove;
//use dataset::EvalBoards;
use game::{Game, GameState};
use visualizer::App;
//use game::Game;
use core::time;
use std::io::{Write, BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::AtomicU64;
use std::time::{Instant, Duration};
use std::{fs, io, thread};
use std::str;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use crate::endgame_table::EndgameTable;
use crate::square::Square;

mod zoberist_hash;

mod bitboard_helper;
//mod attack_board;
mod constants;
mod piece_list;
mod chess_move;
mod game;
mod barsch_bot;
mod bit_board;
mod piece_type;
mod colored_piece_type;
mod square;
//mod dataset;
mod perceptron;
mod delphin_bot;
mod visualizer;
mod evaluation;
mod endgame_table;

use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    
    EndgameTable::generate(4);
    
    println!("Done");
}

fn play_game() {
    let mut app = App::new();
    let mut game = Game::get_start_position();
    //game = Game::from_fen("3qkb1r/1pp2p2/n3p2p/1N1pP1p1/r7/2Q1P1B1/PPP2PPP/R3K2R w KQk - 4 16");

    for i in 0..10 {
        app.render_board(&game, chess_move::NULL_MOVE);    
    }

    //app.read_move();

    while game.get_game_state() == GameState::Undecided {
        let cm = barsch_bot::get_best_move(&mut game);
        //app.read_move();

        game.make_move(cm);

        for i in 0..10 {
            app.render_board(&game, cm);

        }

    }

    println!("Result: {}", game.to_string());
}

fn check_all_perft_board() {
    println!("Checking all fens");
    const FENS: [&str; 6] = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ", 
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ",
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 "
    ];

    const MAX_DEPTHS: [u8; 6] = [8, 7, 9, 7, 6, 7];
    const RESULTS: [&[u64]; 6] = [
        &[1, 20, 400, 8902, 197281, 4865609, 119060324, 3195901860, 84998978956],
        &[1, 48, 2039, 97862, 4085603, 193690690, 8031647685],
        &[1, 14, 191, 2812, 43238, 674624, 11030083, 178633661, 3009794393],
        &[1, 6, 264, 9467, 422333, 15833292, 706045033],
        &[1, 44, 1486, 62379, 2103487, 89941194],
        &[1, 46, 2079, 89890, 3894594, 164075551, 6923051137],
    ];

    for i in 0..6 {
        println!("Index: {}", i + 1);
        let bb = BitBoard::from_fen(FENS[i]);
        //bb.print();
        let target_res = RESULTS[i];
        let mut sum = 0;

        let mut start = Instant::now();
        for d in 0..MAX_DEPTHS[i] {
            
            let mut res = PerftRes::new();
            
            let pair = dfs_board(bb, d, &mut res);

            let count = res.positions;    
            sum += count;

            print!("Depth: {} -> {}", d, count);
            if count != target_res[d as usize] {
                println!(" should be: {}", target_res[d as usize]);
            }
            else {
                println!();
            }
            
        }

        let duration = start.elapsed();
        println!("Time: {:?} Ratio: {} k boards per second", duration, (sum as u128) / duration.as_millis());
    }
}


fn check_all_perft_game() {
    println!("Checking all fens");
    const FENS: [&str; 6] = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ", 
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ",
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 "
    ];

    const MAX_DEPTHS: [u8; 6] = [8, 7, 9, 7, 6, 7];
    const RESULTS: [&[u64]; 6] = [
        &[1, 20, 400, 8902, 197281, 4865609, 119060324, 3195901860, 84998978956],
        &[1, 48, 2039, 97862, 4085603, 193690690, 8031647685],
        &[1, 14, 191, 2812, 43238, 674624, 11030083, 178633661, 3009794393],
        &[1, 6, 264, 9467, 422333, 15833292, 706045033],
        &[1, 44, 1486, 62379, 2103487, 89941194],
        &[1, 46, 2079, 89890, 3894594, 164075551, 6923051137],
    ];

    for i in 0..6 {
        println!("Index: {}", i + 1);
        //let bb = BitBoard::from_fen(FENS[i]);
        let mut game = Game::from_fen(FENS[i]);
        //bb.print();
        let target_res = RESULTS[i];
        let mut sum = 0;

        let mut start = Instant::now();
        for d in 0..MAX_DEPTHS[i] {
            
            let mut res = PerftRes::new();
            
            let pair = dfs_game(&mut game, d, &mut res);

            let count = res.positions;    
            sum += count;

            print!("Depth: {} -> {}", d, count);
            if count != target_res[d as usize] {
                println!(" should be: {}", target_res[d as usize]);
            }
            else {
                println!();
            }
            
        }

        let duration = start.elapsed();
        println!("Time: {:?} Ratio: {} k boards per second", duration, (sum as u128) / duration.as_millis());
    }
}

fn benchmark_moves(b: BitBoard) {
    let mut start = Instant::now();

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

//fn benchmark_moves_game(game: &mut Game) {
//    let mut start = Instant::now();
//
//    for i in 0..100 {
//
//        let mut s = "".to_owned(); 
//        let mut res = PerftRes::new();
//
//        let pair = dfs_game(game, i, &mut res);
//        //let pair = dfs_fast(b, b.generate_pseudo_legal_moves(), i);
//        //fs::write("rust.txt", &s).expect("Unable to write file");
//
//        print!("Depth: {} ", i);
//        res.print();
//        let duration = start.elapsed();
//        println!("{:?}", duration);
//    }
//}


fn print_tree(board: BitBoard, depth_left: u8, max_depth: u8) {
    let mut list = board.get_legal_moves();
    list.sort_unstable_by(|a, b| { return a.get_uci().cmp(&b.get_uci())});

    if depth_left == 0 {
        print_prefix(max_depth - depth_left);

        print!("{}[", list.len());

        for m in list {
            print!("{}", m.get_uci());

            print!(" ");
        }

        println!("]");

        return;
    }    

    print_prefix(max_depth - depth_left);
    println!("{}[", list.len());

    for m in list {
        let mut kek = board.clone();
        kek.make_move(m.clone());
        
        print_prefix(max_depth - depth_left);

        print!("\t");
        print!("{}", m.get_uci());

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

fn dfs_board(board: BitBoard, depth_left: u8, res: &mut PerftRes) {
    //list.sort_unstable_by(|a, b| { return a.get_uci().cmp(&b.get_uci())});
    if depth_left == 0 {
        res.positions += 1;
        return;
    }
    
    let mut list = board.get_legal_moves();

    for m in list {
        let mut kek = board.clone();
        kek.make_move(m);

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