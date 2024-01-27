use std::time::Duration;

use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{endgame_table::EndgameTable, bb_settings::{BBSettings, FactorName, self}, opening_book::OpeningBook, game::{GameState, Game}, match_handler::{play_bot_game, barsch_vs_sf, self}};

const THREAD_COUNT: usize = 14;

pub fn print_confidence(wins: i32, losses: i32, draws: i32) -> f64 {
    let sum = wins + losses + draws;
    let score = wins * 2 + draws;
    let n = sum * 2;
    
    println!("\tW: {} D: {} L: {}", wins, draws, losses);
    println!("\tScored {} out of {}", score, sum * 2);
    println!("\tApprox winrate: {:.2} %", 100.0 * score as f64 / (sum * 2) as f64);
    println!("\tDraw ration {:.2} %", draws as f64 / sum as f64 * 100.0);

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

fn auto_tune(fens: &&Vec<String>, book: &OpeningBook, table: &EndgameTable, mut start_settings: BBSettings) {
    let mut it: usize = FactorName::SafeMobilityP as usize;
    loop {
        let f = bb_settings::ALL_NAMES[it % bb_settings::ALL_NAMES.len()];
        let init = start_settings.eval_factors.get_value(f);
        let better = optimize_value_self_play(fens,  book, table, f, &start_settings.clone());

        start_settings.eval_factors.set_value(f, better);

        
        if init != better {
            println!("Changed {:?}: {} -> {}\n", f, init, better);
            start_settings.eval_factors.print_all();
        }

        it += 1;
    }
}

fn optimize_value_self_play(fens: &&Vec<String>, book: &OpeningBook, table: &EndgameTable, factor_name: bb_settings::FactorName, start_settings: &BBSettings) -> f32 {
    let mut best_settings = start_settings.clone();

    //start_settings.eval_factors.print_all();
    println!("Optimizing: {:?} initial value: {}", factor_name, best_settings.eval_factors.get_value(factor_name));
    let mut it: i32 = 0;
    loop {

        println!("iteration: {}", it);
        it += 1;

        //let (val, sup) = test_eval_range_self_play(table, factor_name, &best_settings);
        let (val, sup) = test_eval_range_stock_fish(fens, book, table, factor_name, &best_settings);
        
        //if sup < 0.60 {
        //    break;
        //}

        best_settings.eval_factors.set_value(factor_name, val);

        break;
    } 

    return best_settings.eval_factors.get_value(factor_name);
}

fn test_eval_range_self_play(fens: &&Vec<String>, book: &OpeningBook, table: &EndgameTable, factor_name: bb_settings::FactorName, start_settings: &BBSettings) -> (f32, f64) {
    
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

        let (wins, losses, draws) = play_sf_parallel(fens, book, table, &improv);

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

fn test_eval_range_stock_fish(fens: &&Vec<String>, book: &OpeningBook, table: &EndgameTable, factor_name: bb_settings::FactorName, start_settings: &BBSettings) -> (f32, f64) {
    
    const STEP_COUNT: i32 = 4;
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

        let (wins, losses, draws) = play_sf_parallel(fens, book, table, &improv);

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

fn play_sf_parallel(fens: &&Vec<String>, book: &OpeningBook, table: &EndgameTable, settings: &BBSettings) -> (i32, i32, i32) {
    let mut threads = Vec::new();
    let fens_per_thread = fens.len() / THREAD_COUNT;
    let mut reisdue = fens.len() % THREAD_COUNT;

    let mut fen_index = 0;
    for t in 0..THREAD_COUNT {
        let mut list = Vec::new();
        for i in 0..fens_per_thread {
            list.push(fen_index);
            fen_index += 1;
        }

        if reisdue > 0 {
            list.push(fen_index);
            fen_index += 1;
            reisdue -= 1;
        }

        threads.push(list);
    }
    
    threads.par_iter_mut().for_each(|list| {
        let mut barsch_wins = 0;
        let mut sf_wins = 0;
        let mut draws = 0;
        let mut barsch_duration = Duration::ZERO;
        let mut sf_duration = Duration::ZERO;

        let mut cmd = match_handler::get_stock_fish_process();

        let mut count = 0;
        for i in 0..list.len() {
            let fen = &fens[list[i]];
            let white_start = Game::from_fen(&fen).is_whites_turn();
            
            let (res, dur_a, dur_b) = barsch_vs_sf(&mut Game::from_fen(&fen), &settings, book, table, true, &mut cmd);       
            barsch_duration += dur_a;
            sf_duration += dur_b;

            if res.is_draw() {
                draws += 1;
            }
            else {
                if white_start == (res == GameState::WhiteCheckmate) {
                    sf_wins += 1;
                }
                else {
                    barsch_wins += 1;
                }
            }
            
            let (res, dur_a, dur_b) = barsch_vs_sf(&mut Game::from_fen(&fen), &settings, book, table, false, &mut cmd);       

            barsch_duration += dur_a;
            sf_duration += dur_b;

            if res.is_draw() {
                draws += 1;
            }
            else {
                if white_start != (res == GameState::WhiteCheckmate) {
                    sf_wins += 1;
                }
                else {
                    barsch_wins += 1;
                }
            }

            count += 1;
            if count % 10 == 0 {
                println!("Sum: W {} L {} D {}", barsch_wins, sf_wins, draws); 
            }
        }
        
        //println!("Chunk done Sum: W {} L {} D {}", a_wins, b_wins, draws);
        list[0] = barsch_wins;
        list[1] = sf_wins;
        list[2] = draws;
        list[3] = barsch_duration.as_millis() as usize;
        list[4] = sf_duration.as_millis() as usize;
    });

    let mut sum_a = 0;
    let mut sum_b = 0;
    let mut sum_d = 0;
    let mut sum_dur_a = 0;
    let mut sum_dur_b = 0;

    for list in threads {
        sum_a += list[0];
        sum_b += list[1];
        sum_d += list[2];
        sum_dur_a += list[3];
        sum_dur_b += list[4];
    }

    println!("Time: {:?}, {:?}", Duration::from_millis(sum_dur_a as u64), Duration::from_millis(sum_dur_b as u64));

    return (sum_a as i32, sum_b as i32, sum_d as i32);
}

pub fn compare_settings_parallel(fens: &Vec<String>, book: &OpeningBook, table: &EndgameTable, a: &BBSettings, b: &BBSettings) -> (i32, i32, i32) {
    let mut threads = Vec::new();
    let fens_per_thread = fens.len() / THREAD_COUNT;
    let mut reisdue = fens.len() % THREAD_COUNT;

    let mut fen_index = 0;
    for t in 0..THREAD_COUNT {
        let mut list = Vec::new();
        for i in 0..fens_per_thread {
            list.push(fen_index);
            fen_index += 1;
        }

        if reisdue > 0 {
            list.push(fen_index);
            fen_index += 1;
            reisdue -= 1;
        }

        threads.push(list);
    }
    
    threads.par_iter_mut().for_each(|list| {
        let mut a_wins = 0;
        let mut b_wins = 0;
        let mut draws = 0;
        let mut duration_a = Duration::ZERO;
        let mut duration_b = Duration::ZERO;

        let mut count = 0;
        for i in 0..list.len() {
            let fen = &fens[list[i]];
            let white_start = Game::from_fen(&fen).is_whites_turn();
            
            let (res, dur_p1, dur_p2) = play_bot_game(&mut Game::from_fen(&fen), table, book, &a, &b);       
            duration_a += dur_p1;
            duration_b += dur_p2;

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
            
            let (res, dur_p1, dur_p2) = play_bot_game(&mut Game::from_fen(&fen), table, book, &b, &a);       
            duration_a += dur_p2;
            duration_b += dur_p1;

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
            if count % 5 == 0 {
                println!("Sum: W {} L {} D {}", a_wins, b_wins, draws); 
            }
        }
        
        //println!("Chunk done Sum: W {} L {} D {}", a_wins, b_wins, draws);
        list[0] = a_wins;
        list[1] = b_wins;
        list[2] = draws;
        list[3] = duration_a.as_millis() as usize;
        list[4] = duration_b.as_millis() as usize;
    });

    let mut sum_a = 0;
    let mut sum_b = 0;
    let mut sum_d = 0;
    let mut sum_dur_a = 0;
    let mut sum_dur_b = 0;

    for list in threads {
        sum_a += list[0];
        sum_b += list[1];
        sum_d += list[2];
        sum_dur_a += list[3];
        sum_dur_b += list[4];
    }

    println!("Time: {:?}, {:?}", Duration::from_millis(sum_dur_a as u64), Duration::from_millis(sum_dur_b as u64));

    return (sum_a as i32, sum_b as i32, sum_d as i32);
}