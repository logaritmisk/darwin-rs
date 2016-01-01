// This example implements the queens problem:
// https://en.wikipedia.org/wiki/Eight_queens_puzzle
// using an evolutionary algorithm.
//
// It's just to show you how ot use the libraray, if you really want to solve the problem
// you'd be better off using the back tracking algorithm.
//
// Note that evolutionary algorithms do no guarantee to always find the optimal solution.
// But they can get very close

extern crate rand;

// internal crates
extern crate darwin_rs;

use rand::Rng;

// internal modules
use darwin_rs::{Individual, SimulationBuilder, BuilderResult};

fn initialize_queens() -> Queens {
    Queens {
        // Start with all queens in one row
        board: vec![
            1,1,1,1,1,1,1,1,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,
        ]
    }
}

#[derive(Debug,Clone)]
struct Queens {
    board: Vec<u8>
}

// Chech one straight line in one specific direction
fn one_trace(board: &Vec<u8>, row: usize, col: usize, dy: i8, dx: i8) -> u8 {
    let mut num_of_collisions = 0;
    let mut x: i16 = col as i16;
    let mut y: i16 = row as i16;

    loop {
        x = x + dx as i16;
        if (x < 0) || (x > 7) { break; }

        y = y + dy as i16;
        if (y < 0) || (y > 7) { break; }

        if board[((y * 8) + x) as usize] == 1 {
            num_of_collisions = num_of_collisions + 1;
        }
    }

    num_of_collisions
}

// Check all eight directions:
fn find_collisions(board: &Vec<u8>, row: usize, col: usize) -> u8 {
    let mut num_of_collisions = 0;

    // up
    num_of_collisions = num_of_collisions + one_trace(board, row, col, -1, 0);

    // up right
    num_of_collisions = num_of_collisions + one_trace(board, row, col, -1, 1);

    // right
    num_of_collisions = num_of_collisions + one_trace(board, row, col, 0, 1);

    // right down
    num_of_collisions = num_of_collisions + one_trace(board, row, col, 1, 1);

    // down
    num_of_collisions = num_of_collisions + one_trace(board, row, col, 1, 0);

    // down left
    num_of_collisions = num_of_collisions + one_trace(board, row, col, 1, -1);

    // left
    num_of_collisions = num_of_collisions + one_trace(board, row, col, 0, -1);

    // left top
    num_of_collisions = num_of_collisions + one_trace(board, row, col, -1, -1);

    num_of_collisions
}

// implement trait functions mutate and calculate_fittness:
impl Individual for Queens {
    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();

        let mut index1: usize = rng.gen_range(1, self.board.len());
        let mut index2: usize = rng.gen_range(1, self.board.len());

        // Pick a position where a queen is placed.
        // Try random position until it finds a queen
        while self.board[index1] != 1 {
            index1 = rng.gen_range(1, self.board.len());
        }

        // Pick a position where no queen is placed and this index is different from the other
        while (index1 == index2) && (self.board[index2] != 0) {
            index2 = rng.gen_range(1, self.board.len());
        }

        // Move queen onto an empty position
        self.board.swap(index1, index2);
    }

    // fittness means here: how many queens are colliding
    fn calculate_fittness(&self) -> f64 {
        let mut num_of_collisions = 0;

        for row in 0..8 {
            for col in 0..8 {
                // Found a queen, so check for collisions
                if self.board[(row * 8) + col] == 1 {
                    num_of_collisions = num_of_collisions + find_collisions(&self.board, row, col);
                }
            }
        }

        num_of_collisions as f64
    }
}

fn main() {
    println!("Darwin test: queens problem");

    let queens_builder = SimulationBuilder::<Queens>::new()
        .iterations(10000)
        .individuals(30)
        .one_individual(initialize_queens())
        .finalize();

    match queens_builder {
        BuilderResult::LowIterration => { println!("more than 10 iteratons needed") },
        BuilderResult::LowIndividuals => { println!("more than 2 individuals needed") },
        BuilderResult::Ok(mut queens_simulation) => {
            let total_run_time = queens_simulation.run();

            println!("total run time: {} ms", total_run_time);
            println!("improvement factor: {}", queens_simulation.improvement_factor);

            // A fittness of zero means a solution was found. Otherwise there are stll some collsisions
            // Just re-run the programm a few times or increase the number of iterations
            queens_simulation.print_fittness();

            // print solution
            for row in 0..8 {
                for col in 0..8 {
                    print!("{} | ", queens_simulation.population[0].individual.board[(row * 8) + col]);
                }
                println!("\n");
            }
        }
    }
}