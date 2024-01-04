#![ allow(unused)]
use bb_settings::{BBSettings, EvalFactors};
use bit_board::BitBoard;
use chess_move::ChessMove;
//use dataset::EvalBoards;
use game::{Game, GameState};

use visualizer::App;
//use game::Game;
use core::{time, panic};
use std::io::{Write, BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::ops::Shr;
use std::sync::atomic::AtomicU64;
use std::time::{Instant, Duration};
use std::{fs, io, thread, num};
use std::str;

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

use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    //check_all_perft_board();
    let table = EndgameTable::load(4);
    let book = OpeningBook::load_from_file("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\barschbot\\book.txt");
    //let mut app = App::new();
    //play_game_player();
    //play_game("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &table);

    //print_confidence(434, 337, 229);
    //print_confidence(332, 308, 320);

    //perceptron::test_perceptron();

    //panic!();

    //auto_tune(&table, bb_settings::STANDARD_SETTINGS);
    //optimize_value(&table, FactorName::PieceValueB, &bb_settings::STANDARD_SETTINGS);
    //test_eval_range(&table);

    play_game_player();

    //let mut game = Game::get_start_position();
    //game.make_move(ChessMove::new_move(Square::E2, Square::E4, ColoredPieceType::WhitePawn, ColoredPieceType::None));
//
    //barsch_bot::get_best_move(&mut game, &table, &bb_settings::STANDARD_SETTINGS, &book);
    


    println!("Done");
}

fn show_bot_game(start_position: &str, table: &EndgameTable, book: &OpeningBook, app: &mut App, bb_settings_a: &BBSettings , bb_settings_b: &BBSettings, flip: bool) -> GameState {
    println!("Playing fen: {}", start_position);
    let mut game = Game::from_fen(start_position);

    for i in 0..10 {
        app.render_board(&game.get_board().type_field, chess_move::NULL_MOVE, flip);    
    }
    //let ten_millis = time::Duration::from_millis(1000);
    //thread::sleep(ten_millis);

    let mut first_player = true;

    while game.get_game_state() == GameState::Undecided {
    
        let set = if first_player { bb_settings_a } else { bb_settings_b };
        first_player = !first_player;


        let cm = barsch_bot::get_best_move(&mut game, table, set, book);

        game.make_move(cm);

        for i in 0..10 {
            app.render_board(&game.get_board().type_field, cm, flip);
        }

        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
    }
    
    let ten_millis = time::Duration::from_millis(100);
    thread::sleep(ten_millis);
    
    println!("Result: {}", game.get_game_state().to_string());
    println!("{}", game.to_string());

    return game.get_game_state();
}

fn play_bot_game(start_position: &str, table: &EndgameTable, book: &OpeningBook, bb_settings_a: &BBSettings , bb_settings_b: &BBSettings) -> GameState {

    let mut game = Game::from_fen(start_position);
    let mut first_player = true;

    while game.get_game_state() == GameState::Undecided {
        let set = if first_player { bb_settings_a } else { bb_settings_b };

        let cm = barsch_bot::get_best_move(&mut game, table, set, book);
        first_player = !first_player;

        game.make_move(cm);
    }

    //if game.get_game_state().is_draw() {
    //    println!("{}", game.to_string());
    //}

    return game.get_game_state();
}

fn play_game_player() {
    let mut app = App::new();
    let mut game = Game::get_start_position(); 
    //game = Game::from_fen("7r/2pkB1pp/p3R3/8/3P4/8/P4PPP/1r2N1K1 w - - 0 1");
    let book = OpeningBook::load_from_file("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\barschbot\\book.txt");

    
    let flip = false;
    for i in 0..10 {
        app.render_board(&game.get_board().type_field, chess_move::NULL_MOVE, flip);    
    }

    let table = EndgameTable::load(0);
    println!("Loaded table");
    //app.read_move();

    while game.get_game_state() == GameState::Undecided {
        //let cm = barsch_bot::get_best_move(&mut game);
        //barsch_bot::better_move_sorter(&mut game.get_legal_moves(), &game.get_board(), chess_move::NULL_MOVE);
        
        let pair = app.read_move();

        if game.get_game_state() == GameState::Undecided {
            let list = game.get_legal_moves();

            for m in list {
                if m.start_square == pair.0 && 
                    m.target_square == pair.1 {
                    game.make_move(m);

                    for i in 0..10 {
                        app.render_board(&game.get_board().type_field, m, flip);
                    }
                    break;
                }
            }
        }

        game.get_board().print();

        let cm = barsch_bot::get_best_move(&mut game, &table, &bb_settings::STANDARD_SETTINGS, &book);      
        
        cm.print();
        println!(" is best move");
        
        game.make_move(cm);
        for i in 0..10 {
            app.render_board(&game.get_board().type_field, cm, flip);
        }    

        let ten_millis = time::Duration::from_millis(0);
        thread::sleep(ten_millis);
    }
    
    let ten_millis = time::Duration::from_millis(1000);
    thread::sleep(ten_millis);
    
    println!("Result: {}", game.get_game_state().to_string());
    println!("{}", game.to_string());
}

fn print_confidence(wins: i32, losses: i32, draws: i32) -> f64 {
    let sum = wins + losses + draws;
    let score = wins * 2 + draws;
    let n = sum * 2;
    
    println!("\tScored {} out of {}", score, sum * 2);

    println!("\tApprox winrate: {:.2} %", 100.0 * score as f64 / (sum * 2) as f64);

    let mut pos_sum = BigInt::zero();

    for i in score..(sum * 2 + 1) {
        pos_sum += binom_pdf(sum * 2, i);
    }
    

    let mut max_div = 0;
    for i in 1..(n + 1) {
        if pos_sum.clone() % (1 << i) == BigInt::zero() {
            max_div = i;
        }
        else {
            break;
        }
    }

    pos_sum >>= max_div;

    while n - max_div > 100 {
        pos_sum >>= 1;
        max_div += 1;
    }

    let denom = BigInt::one() << (n - max_div);

    let prob = pos_sum.to_f64().unwrap() / denom.to_f64().unwrap();

    println!("\tLikelyhood of superiority: {:.3}", (1.0 - prob) * 100.0);

    return (1.0 - prob);

    fn binom_pdf(n: i32, k: i32) -> BigInt {

        let mut numerator = BigInt::one();


        for i in (k + 1)..(n + 1) {
            numerator *= i;
        }

        for i in 2..(n - k + 1) {
            numerator /= i;
        }        

        return numerator;
    }
}

fn auto_tune(table: &EndgameTable, mut start_settings: BBSettings) {
    let mut it: usize = FactorName::SafeMobilityP as usize;
    loop {
        let f = bb_settings::ALL_NAMES[it % bb_settings::ALL_NAMES.len()];
        let init = start_settings.eval_factors.get_value(f);
        let better = optimize_value(table, f, &start_settings.clone());

        start_settings.eval_factors.set_value(f, better);

        println!("Changed {:?}: {} -> {}\n", f, init, better);

        if init != better {
            start_settings.eval_factors.print_all();
        }

        it += 1;
    }
}

fn optimize_value(table: &EndgameTable, factor_name: bb_settings::FactorName, start_settings: &BBSettings) -> f32 {
    let mut best_settings = start_settings.clone();

    //start_settings.eval_factors.print_all();
    println!("Optimizing: {:?} initial value: {}", factor_name, best_settings.eval_factors.get_value(factor_name));
    let mut it: i32 = 0;
    loop {

        println!("iteration: {}", it);
        it += 1;

        let (val, sup) = test_eval_range(table, factor_name, &best_settings);
        
        if sup < 0.60 {
            break;
        }

        best_settings.eval_factors.set_value(factor_name, val);
    } 

    return best_settings.eval_factors.get_value(factor_name);
}

fn test_eval_range(table: &EndgameTable, factor_name: bb_settings::FactorName, start_settings: &BBSettings) -> (f32, f64) {
    
    const STEP_COUNT: i32 = 5;
    const RANGE_DIV: f32 = 0.1;

    let mut improv = start_settings.clone();

    let init_value = start_settings.eval_factors.get_value(factor_name);
    let mut start_val =  init_value - RANGE_DIV * init_value;
    let mut end_val = init_value + RANGE_DIV * init_value;

    if init_value.abs() < 0.00001 {
        start_val = -RANGE_DIV;
        end_val = RANGE_DIV;
    }

    let mut max_score = 0;
    let mut best_val = 0.0;

    let mut results = Vec::new();

    for i in 0..(STEP_COUNT + 1) {
        let val = start_val + (end_val - start_val) * (i as f32 / STEP_COUNT as f32);
        println!("Trying value: {}", val);
        
        improv.eval_factors.set_value(factor_name, val);

        let (wins, losses, draws) = play_all_fens_parallel(table, &improv, &start_settings);

        results.push((val, print_confidence(wins, losses, draws)));

        if wins * 2 + draws > max_score {
            println!("\tNew best value: {}", val);

            best_val = val;
            max_score = wins * 2 + draws;
        }
    }

    println!("Final scores: ");
    results.sort_unstable_by(|a, b| { return b.1.partial_cmp(&a.1).unwrap() });

    for r in &results {
        println!("Value: {} -> {}", r.0, r.1);
    }

    return results[0];
}

fn play_all_fens(table: &EndgameTable, a: &BBSettings, b: &BBSettings) -> (i32, i32, i32) {
    let mut fens = Vec::new();
    
    let mut file = fs::File::open("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\data\\Fens.txt").unwrap();
    //let mut file = fs::File::open("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\data\\chessData.csv").unwrap();
    let book = OpeningBook::load_from_file("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\barschbot\\book.txt");
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        fens.push(line.split(",").collect::<Vec<_>>()[0]);
    }

    //println!("[{}]", fens[0]);

    const SHOW: bool = true;    
    let mut app = App::new();

    let mut a_wins = 0;
    let mut b_wins = 0;
    let mut draws = 0;

    let mut count = 0;
    for fen in fens {
        //println!("Playing fen: {}", fen);

        let mut res = GameState::Undecided;
        if SHOW {
            res = show_bot_game(fen, table, &book, &mut app, &a, &b, false);
        }
        else {
            res = play_bot_game(fen, table, &book, &a, &b);       
        }
        
        let white_start = Game::from_fen(fen).is_whites_turn();

        if res.is_draw() {
            draws += 1;
        }
        else {
            if white_start == (res == GameState::WhiteCheckmate) {
                b_wins += 1;
            }
            else {
                a_wins += 1;
            }
        }

        if SHOW {
            res = show_bot_game(fen, table, &book, &mut app, &b, &a, true);
        }
        else {
            res = play_bot_game(fen, table, &book,&b, &a);       
        }

        if res.is_draw() {
            draws += 1;
        }
        else {
            if white_start != (res == GameState::WhiteCheckmate) {
                b_wins += 1;
            }
            else {
                a_wins += 1;
            }
        }

        count += 1;
        if count % 50 == 0 {
            println!("Sum: W {} L {} D {}", a_wins, b_wins, draws); 
        }
    }

    return (a_wins, b_wins, draws);
}

fn play_all_fens_parallel(table: &EndgameTable, a: &BBSettings, b: &BBSettings) -> (i32, i32, i32) {
    let mut fens = Vec::new();
    
    let mut file = fs::File::open("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\data\\Fens.txt").unwrap();
    //let mut file = fs::File::open("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\data\\chessData.csv").unwrap();
    let book = OpeningBook::load_from_file("C:\\Users\\hmart\\Documents\\GitHub\\Chess-Challenge\\Rust\\barschbot\\book.txt");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        fens.push(line.split(",").collect::<Vec<_>>()[0]);
    }

    const THREAD_COUNT: usize = 10;

    if fens.len() % THREAD_COUNT != 0 {
        panic!("Fen count not divisible by thread count");
    }

    let mut threads = Vec::new();
    let fens_per_thread = fens.len() / THREAD_COUNT;
    for t in 0..THREAD_COUNT {
        let mut list = Vec::new();
        for i in (t * fens_per_thread)..((t + 1) * fens_per_thread) {
            list.push(i);
        }

        threads.push(list);
    }

    //for list in &threads {
    //    println!("Thread: {} -> {}", list[0], list[list.len() - 1]);
    //}

    
    threads.par_iter_mut().for_each(|list| {
        let mut a_wins = 0;
        let mut b_wins = 0;
        let mut draws = 0;

        let mut count = 0;
        for i in 0..list.len() {
            let fen = fens[list[i]];
            let mut res = play_bot_game(fen, table, &book, &a, &b);       
            
            let white_start = Game::from_fen(fen).is_whites_turn();
            
            if res.is_draw() {
                draws += 1;
            }
            else {
                if white_start == (res == GameState::WhiteCheckmate) {
                    b_wins += 1;
                }
                else {
                    a_wins += 1;
                }
            }
            
            res = play_bot_game(fen, table, &book, &b, &a);       
            
            if res.is_draw() {
                draws += 1;
            }
            else {
                if white_start != (res == GameState::WhiteCheckmate) {
                    b_wins += 1;
                }
                else {
                    a_wins += 1;
                }
            }

            //count += 1;
            //if count % 50 == 0 {
            //    println!("Sum: W {} L {} D {}", a_wins, b_wins, draws); 
            //}
        }
        
        //println!("Chunk done Sum: W {} L {} D {}", a_wins, b_wins, draws);
        list[0] = a_wins;
        list[1] = b_wins;
        list[2] = draws;
    });

    let mut sum_a = 0;
    let mut sum_b = 0;
    let mut sum_d = 0;

    for list in threads {
        sum_a += list[0];
        sum_b += list[1];
        sum_d += list[2];
    }

    return (sum_a as i32, sum_b as i32, sum_d as i32);
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