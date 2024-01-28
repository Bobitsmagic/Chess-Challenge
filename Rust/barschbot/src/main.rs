#![ allow(unused)]
use bb_settings::{BBSettings, EvalFactors};
use bit_board::BitBoard;
use chess_move::ChessMove;
//use dataset::EvalBoards;
use game::{Game, GameState};

use graphics::color::PURPLE;
use visualizer::Visualizer;
//use game::Game;
use core::{time, panic};
use std::io::{Write, BufRead, BufReader, Read, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::ops::Shr;
use std::sync::atomic::AtomicU64;
use std::time::{Instant, Duration};
use std::{fs, io, thread, num};
use std::str;
use rand::Rng;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive};

use crate::bb_settings::FactorName;
use crate::bitboard_helper::{RANK_MASKS, FILE_MASKS};
use crate::colored_piece_type::ColoredPieceType;
use crate::dataset::EvalBoards;
use crate::endgame_table::EndgameTable;
use crate::opening_book::OpeningBook;
use crate::perceptron::Perceptron;
use crate::square::Square;

use std::process::{Command, Stdio, Child};

use rayon::prelude::*;

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
mod dataset;
mod perceptron;
mod visualizer;
mod evaluation;
mod endgame_table;
mod bb_settings;
mod opening_book;
mod match_handler;
mod auto_tuning;
mod compact_hashmap;

use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let (table, book) = load_files();

    /* 
    let fens = load_fens("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\data\\Fens.txt");
    let a = bb_settings::STANDARD_SETTINGS;
    let mut b = bb_settings::STANDARD_SETTINGS;

    b.null_move_pruning = false;
    //b.max_depth = 4;

    let (w, d, l) = auto_tuning::compare_settings_parallel(&fens, &book, &table, &a, &b);
    auto_tuning::print_confidence(w, d, l);
    */
    
    play_all_puzzles(&book, &table);


    match_handler::play_game_player(&mut Game::get_start_position(), 
    true, 
        &bb_settings::STANDARD_SETTINGS, &table, &book);

    println!("Done");
}

fn load_files() -> (EndgameTable, OpeningBook) {
    let table = EndgameTable::load(4);

    let book = OpeningBook::load_from_file("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\barschbot\\book.txt");

    return (table, book);
}

fn load_fens(path: &str) -> Vec<String> {
    
    let contents = fs::read_to_string(path).unwrap();
    let ret: Vec<String> = contents
        .lines()
        .map(|line| line.split(",").next().unwrap().to_string())
        .collect();

    println!("Loaded {} fens from: {}", ret.len(), path.split("\\").last().unwrap());

    return ret;
}

fn load_lichess_puzzles() -> Vec<(String, Vec<ChessMove>)>{
    //PuzzleId, FEN, Moves, Rating, RatingDeviation, Popularity, NbPlays, Themes, GameUrl, OpeningTags

    let contents = fs::read_to_string("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\data\\lichess_db_puzzle.csv").unwrap();
    let ret: Vec<(String, Vec<ChessMove>)> = contents
        .lines()
        .skip(1)
        .map(|line| {
            let v: Vec<&str> = line.split(",").collect();
            
            return (v[1].to_string(), v[2].split(" ").map(|s| ChessMove::new_uci_move(s)).collect());
        })
        .collect();

    return ret;
}

//Depth 8: 928 / 1000 (92.8%)
//depth 7: 909 / 1000 (90.9)
//Depth 6: 887 / 1000 (88.7%)
//Depth 5: 857 / 1000 (85.7%)
//Depth 4: 823 / 1000 (82.3%)
//Depth 3: 783 / 1000 (78.3%)

//Depth 2: 2579431 / 3678110 (70.12925%)
//Depth 3: 2835433 / 3678110 (77.0894%)

fn play_all_puzzles(book: &OpeningBook, table: &EndgameTable ) {
    let mut puzzles = load_lichess_puzzles();


    println!("Loaded {} puzzles", puzzles.len());

    //let mut app = Visualizer::new();
    let mut rng = rand::thread_rng();
    const THREAD_COUNT: usize = 13;
    let counter = puzzles.len();

    puzzles.par_chunks_mut(counter / THREAD_COUNT).for_each(|slice| {
        let mut correct = 0;
        let mut counter = 0;
        for (fen, moves) in slice {
            counter += 1;
            let mut game = Game::from_fen(&fen);
            let mut all_correct = true;
            
            for i in 0..moves.len() {
                
                let cm = if i % 2 == 0 {
                    let fm = game.get_uci_move(moves[i].get_uci());
                    //println!("Puzzle move: {}", fm.get_board_name(&game.get_board()));
                    
                    fm
                }
                else {
                    //let ml = game.get_legal_moves();
                    //let bmove = ml[rng.gen_range(0..ml.len())];
    
                    let bmove = barsch_bot::get_best_move(&mut game, table, &bb_settings::STANDARD_SETTINGS, book);
                    
                    //println!("Expected: {} Barsch: {}", moves[i].get_uci(), bmove.get_uci());
                    
                    if bmove.get_uci() != moves[i].get_uci() {
                        all_correct = false;
                        
                        //println!("Index: {}, Fen: {}", counter, fen);
                        break;
                    }
    
                    bmove
                };         
    
                game.make_move(cm);
    
                //app.render_board(&game.get_board().type_field, cm, false);
            }
    

            *fen = all_correct.to_string();

            if all_correct {
                correct += 1;
            }

            if counter % 10000 == 0 {
                println!("{}", counter);
            }
        }
    

    });


    let mut cc = 0;
    
    for (fen, moves) in puzzles {
        if fen.to_string() == true.to_string() {
            cc += 1;
        }
    }
    println!("{} / {} ({}%)", cc, counter, cc as f32 * 100.0 / counter as f32);
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

            if count != target_res[d as usize] {
                print!("Depth: {} -> {}", d, count);
                println!(" should be: {}", target_res[d as usize]);
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

            if count != target_res[d as usize] {
                print!("Depth: {} -> {}", d, count);
                println!(" should be: {}", target_res[d as usize]);
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