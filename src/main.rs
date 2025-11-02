use std::borrow::Cow;
use std::{env, process};

mod problems;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: cargo run -- <PROBLEM NUMBER>");
        process::exit(1);
    }
    let problem = problem(&args[1]);
    problems::solve(problem.as_ref());
}

fn problem(input: &str) -> Cow<'_, str> {
    if input.len() == 5 && input.chars().next().unwrap() == 'p' {
        return Cow::Borrowed(input);
    }
    let number: u16 = input.parse().unwrap();
    Cow::Owned(format!("p{number:0>4}"))
}
