use std::{process::{Child, Stdio, Command}, io::{BufWriter, BufReader, Write, BufRead, self}, time::{Duration, Instant}};

use crate::{visualizer::Visualizer, square::{Square, self}, game::{Game, GameState}, chess_move::{ChessMove, self}, endgame_table::EndgameTable, opening_book::OpeningBook, bb_settings::BBSettings, barsch_bot};


pub fn get_human_move(app: &mut Visualizer, game: &mut Game) -> ChessMove {
    let mut moves = game.get_legal_moves();
    loop {
        let (s1, s2) = app.read_move();

        for m in &moves {
            if s1 == m.start_square && s2 == m.target_square {
                return *m;
            }
        }
        
        if s1 == Square::A1 && s2 == Square::A1 {
            return chess_move::NULL_MOVE;
        }

        println!("Illegal move!: {} {}", s1.to_string(), s2.to_string());
    }
}

pub fn get_stock_fish_process() -> Child {
    return Command::new("C:\\Users\\hmart\\Desktop\\stockfish\\stockfish-windows-x86-64-avx2.exe")
    .stdout(Stdio::piped())
    .stdin(Stdio::piped())
    .spawn()
    .unwrap();
}

fn get_stock_fish_move(game: &mut Game, cmd: &mut Child) -> ChessMove {
    const DEPTH: u8 = 4;
    
    let stdin = cmd.stdin.as_mut().unwrap();
    let mut stdin_writer = BufWriter::new(stdin);
    {
        stdin_writer.write_all(format!("position fen {}\n", game.get_board().get_fen()).as_bytes());
        stdin_writer.flush();
        stdin_writer.write_all(format!("go depth {}\n", DEPTH).as_bytes());
        stdin_writer.flush();
    }

    let mut stdout = cmd.stdout.as_mut().unwrap();   

    let mut stdout_reader = BufReader::new(stdout);

    
    loop {
        let mut s = String::new();
        let bc = stdout_reader.read_line(&mut s).expect("error");
        
        if s.starts_with("bestmove") {
            
            //println!("{}", s);


            let parts = s.split(" ").collect::<Vec<_>>();
            let length = parts[1].len() - 2;


            let bms = if parts.len() == 2 {
                &parts[1][..length]
            }
            else {
                parts[1]
            };
            
            //println!("SF: [{}]", bms);
            let list = game.get_legal_moves();
            
            for m in list {
                
                if m.get_uci() == bms {
                    return m;
                }
            }

            break;
        }
    }
        
    panic!("Stockfish made an illegal move?");
}

fn create_process_and_get_sf_move(game: &mut Game) -> ChessMove {
    let mut cmd = get_stock_fish_process();

    return get_stock_fish_move(game, &mut cmd);
}

fn get_barschbot_move(game: &mut Game, table: &EndgameTable, settings: &BBSettings, book: &OpeningBook) -> ChessMove {
    return barsch_bot::get_best_move(game, table, settings, book);
}

pub fn play_game_player(game: &mut Game, mut human_turn: bool, settings: &BBSettings, table: &EndgameTable, book: &OpeningBook) { 
    let mut app = Visualizer::new();
    let flip = false;

    app.render_board(&game.get_board().type_field, chess_move::NULL_MOVE, flip);    
    
    while game.get_game_state() == GameState::Undecided {
        
        let mut cm = if human_turn {
            get_human_move(&mut app, game)
        }
        else {
            get_barschbot_move(game, table, settings, book)
        };

        if cm == chess_move::NULL_MOVE {
            cm = get_barschbot_move(game, table, settings, book);
        }

        human_turn = !human_turn;

        game.make_move(cm);

        app.render_board(&game.get_board().type_field, cm, flip);
    }
    
    println!("Result: {}", game.get_game_state().to_string());
    println!("{}", game.to_string());
}

pub fn play_bot_game(game: &mut Game, table: &EndgameTable, book: &OpeningBook, bb_settings_a: &BBSettings , bb_settings_b: &BBSettings) -> (GameState, Duration, Duration) {
    let mut first_player = true;
    let mut duration_1 = Duration::ZERO;
    let mut duration_2 = Duration::ZERO;

    while game.get_game_state() == GameState::Undecided {
        let set = if first_player { bb_settings_a } else { bb_settings_b };
        let start = Instant::now();
        let cm = barsch_bot::get_best_move(game, table, set, book);
        
        if first_player {
            duration_1 += start.elapsed();
        }
        else {
            duration_2 += start.elapsed();
        }

        first_player = !first_player;

        game.make_move(cm);
    }

    return (game.get_game_state(), duration_1, duration_2);
}

pub fn barsch_vs_sf(game: &mut Game, bb_setting: &BBSettings, book: &OpeningBook, table: &EndgameTable, mut barsch_turn: bool, cmd: &mut Child) -> (GameState, Duration, Duration) {
    let mut duration_1 = Duration::ZERO;
    let mut duration_2 = Duration::ZERO;

    while game.get_game_state() == GameState::Undecided {
        
        let start = Instant::now();
        let cm = if barsch_turn {
            barsch_bot::get_best_move(game, table, bb_setting, book)
        } else {
            get_stock_fish_move(game, cmd)
        };
        
        if barsch_turn {
            duration_1 += start.elapsed();
        }
        else {
            duration_2 += start.elapsed();
        }

        barsch_turn = !barsch_turn;

        game.make_move(cm);
    }

    return (game.get_game_state(), duration_1, duration_2);
}

fn uci_loop() {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer);

    println!("Reading: [{}]", buffer);
}