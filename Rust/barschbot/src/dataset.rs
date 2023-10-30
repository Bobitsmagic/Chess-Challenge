use crate::delphin_bot;
use crate::{bit_board::BitBoard, barsch_bot, game::Game, perceptron::Perceptron};
use std::cmp;
use std::fs::{read_to_string, self};
use neuroflow::FeedForward;
use neuroflow::data::DataSet;
use neuroflow::io;
use neuroflow::activators::Type::Tanh;
use rand::{thread_rng, Rng};

pub struct EvalBoards {
    boards: Vec<BitBoard>,
    evals: Vec<i32>
}

impl EvalBoards {
    pub fn load(path: &str) -> EvalBoards {
        println!("Loading dataset at {}", path);

        let mut boards = Vec::new();
        let mut evals = Vec::new();

        let mut positions = 0;
        for line in read_to_string(path).unwrap().lines().skip(1) {
            let parts = line.split(",").collect::<Vec<_>>();
            
            positions += 1;

            //skipping all checkmates for now
            if parts[1].contains('#') {
                continue;
            }

            let board = BitBoard::from_fen(parts[0]);
            
            let mut skip = 0;
            for c in parts[1].chars() {
                if c == '+' || c == '-' || c == '0' {
                    break;
                }

                skip += 1;
            }


            let val_string: String = parts[1].chars().skip(skip).collect();
            let opt = val_string.parse();
            if opt.is_err() {
                println!("Error at {}[{}]", val_string.len(), val_string);
            }
            
            let eval: i32 = opt.unwrap();

            boards.push(board);
            evals.push(eval);


            if positions % 1_000_000 == 0{
                println!("{}", positions);
            }   
        }

        println!("Totol position count: {}, filtered count: {}", positions, boards.len());

        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for i in &evals {
            min = i32::min(min, *i);
            max = i32::max(max, *i);

        }
        println!("Min eval: {} Max eval {}", min, max);

        return EvalBoards { boards, evals };
    } 

    pub fn get_square_error(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.boards.len() {

            let mut game = Game::from_board(self.boards[i]);
            let eval = barsch_bot::negation_max(&mut game, 2).1 
                * (if game.is_whites_turn() { 1 } else { -1 }) as f64;



            let dif = eval / 10.0 - self.evals[i] as f64; 
            
            sum += (dif * dif) as f64;

            if i % 10_000 == 0 {
                println!("{} -> {}%", i, (i as f64 / self.boards.len() as f64) * 100.0);
            }  
        }

        return sum / self.boards.len() as f64;
    }

    pub fn train_modell(&self) {
        const SAMPLE_COUNT: usize = 100000;
        
        let mut nn = FeedForward::new(&[773, 600, 400, 200, 100, 1]);       
        nn.activation(Tanh);
        nn.learning_rate(0.001);
        nn.momentum(0.05);
        
        println!("Started gdc");
        let mut cap = 1;
        for i in 0..10000 {       
            let (input_set, output_set) = self.create_random_training_set(SAMPLE_COUNT);

            let mut data = DataSet::new();

            for i in 0..input_set.len() {
                data.push(&input_set[i], &[output_set[i]]);
            }

            //println!("Started training");
            nn.train(&data, 1000);

            if i == cap {
                cap *= 2;
                println!("Calc abs error");
                let tuple = Self::calc_abs_error(&input_set, &output_set, &mut nn);
                println!("It: {} Error mean: {}, min : {}, max {}", i, tuple.0, tuple.1, tuple.2);
            }
        }

        io::save(&mut nn, "raw_board.flow").unwrap();
    }

    pub fn calc_abs_error(input_set: &Vec<Vec<f64>>, output_set: &Vec<f64>, nn: &mut FeedForward) -> (f64, f64, f64) {
        let mut sum = 0.0;
        let mut max = f64::MIN;
        let mut min = f64::MAX;

        let mut max_output = f64::MIN;
        let mut min_output = f64::MAX;

        for input_index in 0..input_set.len() {
            let output = nn.calc(&input_set[input_index])[0];
            let error = (output_set[input_index] - output).abs();

            sum += error;

            if error > max {
                max = error;
            }

            if error < min {
                min = error;
            }

            if output > max_output {
                max_output = output;
            }

            if output < min_output {
                min_output = output;
            }
        } 

        println!("Min output: {}, Max output {}", min_output, max_output);
        return (sum / input_set.len() as f64, min, max);
    }

    pub fn create_input_set(&self) -> Vec<Vec<f64>> {
        let mut ret = Vec::new();

        for i in 0..self.boards.len() {

            let eval = delphin_bot::get_neutral_vector(&self.boards[i]);
            
            let mut converted = Vec::new();
            for v in eval {
                converted.push(v as f64);
            }

            ret.push(converted);

            if i % 1_000_000 == 0 {
                println!("{} -> {}%", i, (i as f64 / self.boards.len() as f64) * 100.0);
            }  
        }

        return ret;
    }

    pub fn create_random_training_set(&self, size: usize) -> (Vec<Vec<f64>>, Vec<f64>) {
        let mut inpt = Vec::new();
        let mut outp = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..size {

            let i: usize =  rng.gen_range(0..self.boards.len());
            
            let eval = delphin_bot::get_neutral_vector(&self.boards[i]);
            
            inpt.push(eval);

            outp.push(normalize(self.evals[i] as f64));
            //if i % 1_000_000 == 0 {
            //    println!("{} -> {}%", i, (i as f64 / self.boards.len() as f64) * 100.0);
            //}  
        }

        return (inpt, outp);

        fn normalize(val: f64) -> f64 {
            return f64::max(f64::min(val, 10_000.0), -10_000.0) / 10_000.0;
        }
    }

    pub fn create_output_set(&self) -> Vec<f64> {
        let mut ret = Vec::new();

        for v in &self.evals {
            ret.push((*v as f64));
        }

        return ret;

        //fn normalize(val: f64) -> f64 {
        //    return f64::max(f64::min(val, 10_000.0), -10_000.0) / 10_000.0;
        //}
    }
}