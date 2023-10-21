use std::os::windows;
use rand::prelude::*;
use rand_distr::StandardNormal;

pub struct Perceptron {
    weights: Vec<f32>
}

pub fn test_perceptron() {
    const DIMENSIONS: usize = 6;
    const SAMPLE_COUNT: usize = 12_000_00;
    let mut rng = thread_rng();

    let target_weights = vec![3.14159274, -2.71828175, -42.0, 1337.0, 69.0, -31.0];

    let teacher = Perceptron { weights: target_weights };

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    for i in 0..SAMPLE_COUNT {
        let mut vec: Vec<f32> = Vec::new();
        for d in 0..DIMENSIONS {
            vec.push(rng.sample(StandardNormal));
        }
        
        let str: f32 =  rng.sample(StandardNormal);
        outputs.push(teacher.calc_output(&vec) + str * 10.0);
        inputs.push(vec);
    }

    let mut nn = Perceptron::new(DIMENSIONS);
    nn.randomize_weights();

    println!("Started gdc");
    nn.gradient_descent(&inputs, &outputs);
}

impl Perceptron {
    pub fn new(weight_count: usize) -> Self {
        return Perceptron { weights: vec![0.0; weight_count]  }
    }

    pub fn print(&self) {
        println!("Weights: ");
        for w in &self.weights {
            print!("{} ", w);
        }
        println!();
    }

    pub fn randomize_weights(&mut self) {
        let mut rng = thread_rng();

        for i in 0..self.weights.len() {
            self.weights[i] = rng.sample(StandardNormal);
        }
    }

    pub fn calc_squares_error(&self, input_set: &Vec<Vec<f32>>, output_set: &Vec<f32>) -> f32 {
        let mut sum = 0.0;
        for input_index in 0..input_set.len() {
            let error = output_set[input_index] - self.calc_output(&input_set[input_index]);

            sum += error * error;
        }

        return sum;
    }

    pub fn calc_output(&self, input: &Vec<f32>) -> f32 {
        assert_eq!(input.len(), self.weights.len());

        let mut sum = 0.0;
        for i in 0..input.len() {
            sum += input[i] * self.weights[i];
        }

        return sum;
    }

    pub fn calc_gradient(&self, input_set: &Vec<Vec<f32>>, output_set: &Vec<f32>) -> Vec<f32> {
        let mut gradient = vec![0.0; input_set[0].len()];

        for input_index in 0..input_set.len() {
            let error = output_set[input_index] - self.calc_output(&input_set[input_index]);

            for w in 0..gradient.len() {
                gradient[w] += input_set[input_index][w] * error;
            }
        }

        return gradient;
    }

    pub fn calc_stochastic_gradient(&self, input_set: &Vec<Vec<f32>>, output_set: &Vec<f32>, index_set: &Vec<usize>) -> Vec<f32> {
        let mut gradient = vec![0.0; input_set[0].len()];


        for input_index in index_set {
            let error = output_set[*input_index] - self.calc_output(&input_set[*input_index]);

            for w in 0..gradient.len() {
                gradient[w] += input_set[*input_index][w] * error;
            }
        }

        return gradient;
    }

    pub fn gradient_descent(&mut self, input_set: &Vec<Vec<f32>>, output_set: &Vec<f32>) {

        const MAX_IT: usize = 100_000;
        const SAMPLE_COUNT: usize = 100000;
        let mut indices: Vec<usize> = Vec::new();
        let mut rng = thread_rng();

        for i in 0..MAX_IT {
            indices.clear();
            for _ in 0..SAMPLE_COUNT {
                indices.push(rng.gen_range(0..SAMPLE_COUNT));
            }

            let gradient = self.calc_stochastic_gradient(input_set, output_set, &indices);
            
            //println!("Gradient");
            //println!("{:?}", gradient);

            for i in 0..gradient.len() {
                self.weights[i] += (gradient[i] / SAMPLE_COUNT as f32) * 0.001;
            }


            if i % 1000 == 0 {
                println!("{} -> {}", i, self.calc_squares_error(input_set, output_set));
            }
        }
        println!("NSE: {}", self.calc_squares_error(input_set, output_set) / input_set.len() as f32);
    }
}
