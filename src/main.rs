use std::{env, process};

mod problems;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: cargo run -- <PROBLEM NUMBER>");
        process::exit(1);
    }
    problems::solve(&args[1]);
}
