use crate::evaluation::{static_eval, generate_eval_attributes};
use crate::perceptron;
use crate::{bit_board::BitBoard, barsch_bot, game::Game, perceptron::Perceptron};
use std::cmp;
use std::fs::{read_to_string, self};
use rand::{thread_rng, Rng};

pub struct EvalBoards {
    boards: Vec<BitBoard>,
    evals: Vec<f32>
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

            let mut board = BitBoard::from_fen(parts[0]);
            
            if !barsch_bot::is_quiet_pos(&mut board) {
                continue;
            }

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

            if eval.abs() > 1000 {
                continue;
            }
            boards.push(board);
            evals.push(eval as f32 / 100.0);

            if positions % 1_000_000 == 0{
                println!("{}", positions);
            }   
        }

        println!("Totol position count: {}, filtered count: {}", positions, boards.len());

        let mut min = f32::MAX;
        let mut max = f32::MIN;

        for i in &evals {
            min = f32::min(min, *i);
            max = f32::max(max, *i);
        }

        for i in 0..evals.len() {
            if evals[i] == min || evals[i] == max {
                boards[i].print();
                
                println!("Eval: {}", evals[i]);
                break;
            }
        }

        println!("Min eval: {} Max eval {}", min, max);

        return EvalBoards { boards, evals };
    } 

    pub fn create_input_set(&self) -> Vec<Vec<f32>> {
        let mut ret = Vec::new();
        println!("Creating input set");
        for i in 0..self.boards.len() {

            ret.push(generate_eval_attributes(&self.boards[i]).get_vector());

            if i % 1_000_000 == 0 {
                println!("{} -> {}%", i, (i as f64 / self.boards.len() as f64) * 100.0);
            }  
        }

        return ret;
    }

    pub fn create_output_set(&self) -> Vec<f32> {
        return self.evals.clone();

        //fn normalize(val: f64) -> f64 {
        //    return f64::max(f64::min(val, 10_000.0), -10_000.0) / 10_000.0;
        //}
    }
}