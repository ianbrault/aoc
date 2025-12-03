/*
** src/main.rs
*/

mod driver;
mod itertools;
mod macros;
mod puzzles;
mod types;
mod utils;

use clap::Parser;
use simplelog::{LevelFilter, TermLogger};

#[derive(Parser)]
struct Args {
    /// Year, runs all if not provided
    year: Option<usize>,
    /// Day, runs all if not provided
    day: Option<usize>,
    /// Use sample puzzle input
    #[arg(short, long)]
    sample: bool,
    /// Benchmark puzzle solutions and produce a report
    #[arg(short, long)]
    benchmark: bool,
    /// Number of iterations to use when benchmarking
    #[arg(long, id = "N", default_value_t = 10)]
    benchmark_iterations: usize,
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
}

fn initialize_logger(debug: bool) {
    let log_level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    TermLogger::init(
        log_level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();
}

fn main() {
    let args = Args::parse();
    initialize_logger(args.debug);

    if args.benchmark {
        // Run benchmarks and produce a report
        driver::run_benchmark(args.benchmark_iterations);
    } else {
        // Run one or more puzzles, as specified
        driver::run_puzzles(args.year, args.day, args.sample);
    }
}
