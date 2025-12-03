/*
** src/driver.rs
*/

use crate::itertools::*;
use crate::puzzles::{Answer, Puzzle, PuzzleIterator, PuzzleModules};
use crate::utils;

use log::{debug, info};

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::time::Instant;

const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn load_input(puzzle: Puzzle, sample: bool) -> String {
    let prefix = if sample { "sample" } else { "full" };
    let local_path = Path::new("input")
        .join(puzzle.year.to_string())
        .join(format!("{}/{}.txt", prefix, puzzle.day + 1));
    let full_path = Path::new(PROJECT_DIR).join(&local_path);
    debug!(
        "Loading {} input from: {}",
        puzzle,
        local_path.to_string_lossy()
    );
    utils::read_file(&full_path).unwrap_or_else(|_| panic!("no input for {}", puzzle))
}

fn print_benchmark(start_time: Instant) {
    let duration_ms = start_time.elapsed().as_secs_f64() * 1000.0;
    info!("Time to solve: {:.03}ms", duration_ms);
}

fn answer_to_string(answer: Option<Answer>) -> String {
    match answer {
        None => "no answer".to_string(),
        Some(answer) => answer.to_string(),
    }
}

pub fn run_puzzles(year: Option<usize>, day: Option<usize>, sample: bool) {
    for puzzle in PuzzleIterator::new(year, day) {
        info!("{}", puzzle);
        let input = load_input(puzzle, sample);
        let solver = PuzzleModules::dispatch(puzzle.year, puzzle.day);

        // Solve the puzzle and benchmark
        let t = Instant::now();
        let solution = (solver)(input);
        print_benchmark(t);
        info!("Part A solution: {}", answer_to_string(solution.part_a));
        info!("Part B solution: {}", answer_to_string(solution.part_b));
    }
}

pub fn run_benchmark(iterations: usize) {
    // Run puzzle benchmarks
    let mut benchmark = HashMap::new();
    for _ in 0..iterations {
        for puzzle in PuzzleIterator::all() {
            let input = load_input(puzzle, false);
            let solver = PuzzleModules::dispatch(puzzle.year, puzzle.day);

            let t = Instant::now();
            (solver)(input);
            let elapsed = t.elapsed().as_secs_f64() * 1000.0;

            let entry = benchmark.entry(puzzle).or_insert(Vec::new());
            entry.push(elapsed);
        }
    }

    // Print benchmark results
    println!("## Results\n");
    println!(
        "Results are benchmarked with {} executions through each solution.\n",
        iterations
    );
    for year in PuzzleIterator::all().map(|puzzle| puzzle.year).dedup() {
        println!("### {}\n", year);
        println!("| Puzzle | Time (ms) |");
        println!("|:---|---:|");
        for puzzle in PuzzleIterator::all() {
            if puzzle.year != year {
                continue;
            }
            let t = benchmark[&puzzle].iter().sum::<f64>() / benchmark[&puzzle].len() as f64;
            println!("| {} | {:.3} |", puzzle.day + 1, t);
        }
        println!();
    }
}
