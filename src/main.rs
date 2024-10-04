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
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    // parse command-line args
    let args = Args::parse();
    // initialize terminal logger
    let log_level = if args.debug {
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

    // run one or more puzzles, as specified
    driver::run_puzzles(args.year, args.day, args.sample);
}
