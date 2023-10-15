#![ allow(unused)]
use bit_board::BitBoard;
use chess_move::ChessMove;
//use game::Game;
use core::time;
use std::io::{Write, BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::AtomicU64;
use std::time::{Instant, Duration};
use std::{fs, io, thread};
use std::str;

use crate::square::Square;

mod zoberist_hash;

mod bitboard_helper;
//mod attack_board;
mod constants;
mod piece_list;
mod chess_move;
//mod game;
//mod barsch_bot;
mod bit_board;
mod piece_type;
mod colored_piece_type;
mod square;

use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    //println!("{:?}", false.cmp(&true));

    //com_frontend();
//4k3/8/8/2b5/3q4/4P3/4Q3/6K1 w - - 0 24
    //let mut game: Game = Game::from_fen("rnb1k3/8/1pp2q2/p1b3p1/3B2pN/1P2P1Pp/P1P1Q2P/R2N2K1 b q - 0 23");
    ////let mut game: Game = Game::from_fen("4k3/8/8/2b5/3q4/4P3/4Q3/6K1 w - - 0 24");
    //barsch_bot::get_best_move(&mut game);

    //let mut board = Board::from_fen("rnb1k3/8/1pp5/p1b3p1/3q2pN/1P2P1Pp/P1P1Q2P/R2N2K1 w q - 0 24");

    //for i in 0..8 {
    //    println!("Index: {}", i);
    //    bitboard_helper::print_bitboard(bitboard_helper::RIGHT_MOVE_MASK[i as usize]);
    //}

    
        check_all_perft();
    //return;
    let mut board = BitBoard::start_position();
    //board = BitBoard::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ");
    board = BitBoard::from_fen("8/8/8/1Ppp3r/1KR2p1k/8/4P1P1/8 w - c6");
    
    //print_tree(board, 3, 3);
    //benchmark_moves(board);
    board.print();
    //board.print_bitboards();
    let mut list = board.get_legal_moves();
//
    BitBoard::print_moves(&list);
    ////return;
//
    //board.make_move(list[9]);
//
    //board.print();
    ////board.print_bitboards();
//
    //list = board.get_legal_moves();
//
    //BitBoard::print_moves(&list);

    //board.make_move(&ChessMove::new_move(constants::E3, constants::D4, constants::WHITE_PAWN, ))

    //let list = board.get_legal_moves();

    //Board::print_moves(&list);

    //r1bqk2r/2p5/p1pb1p2/2Npp3/3P4/4P1NP/PPP3K1/R2QR3 b kq - 0 19
    //com_frontend();


}

/* 
fn com_frontend() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1337")?;

    loop {
        let mut fen_bytes = [0 as u8; 200];
        stream.read(&mut fen_bytes)?;
    
        //println!("{:?}", fen_bytes);
    
        let length = fen_bytes[0];

        let double = fen_bytes[1] == 0;
        if double {
            //println!("Loaded another load");
            stream.read(&mut fen_bytes);
            
            //println!("{:?}", fen_bytes);
        } 
                
        let mut v = fen_bytes.to_vec();

        let mut kek = 0;
        for i in 0..v.len() {
            if v[i] == 0 {
                kek = i;
                break;
            }
        }

        if kek as u8 > length {
            v.remove(0);
        }

        //println!("v: {:?}", v);
    
        let s = str::from_utf8(&v).unwrap();
    
        //println!("Recieved [{}]", s);
        
        let mut game: Game = Game::from_fen(s);
        let m = barsch_bot::get_best_move(&mut game);
        m.print_uci();
        println!();
    
        //let ten_millis = time::Duration::from_millis(500);
        //thread::sleep(ten_millis);
        
        let str = m.get_uci();
        stream.write_all(&[str.len() as u8; 1]);
        stream.write_all(str.as_bytes());
    
        println!("Done");
    }

    Ok(())
}
*/

fn check_all_perft() {
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

/* 
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
*/

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