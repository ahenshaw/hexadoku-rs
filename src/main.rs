use hexadoku::{HexPuzzle, Possible};
use std::env;
use std::path::PathBuf;
use std::process;
use std::time::Instant;

fn main() {
    // get puzzle file name from command line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Expected a puzzle file name");
        process::exit(1);
    }
    let filename = PathBuf::from(&args[1]);

    match HexPuzzle::new_from_file(&filename) {
        Ok(puzzle) => {
            println!("\nInput: {}", filename.display());
            println!("{puzzle}");
            let start = Instant::now();
            if let Some(solution) = solve(&puzzle) {
                let elapsed = Instant::now() - start;
                println!("Solution:\n{}", solution);
                println!("Solved in {elapsed:?}");
            } else {
                println!("Puzzle appears to be unsolveable")
            }
        }
        Err(e) => eprintln!("File error: {}", e),
    }
}

enum State {
    Backtrack,
    Iterate((HexPuzzle, Possible)),
    Search(HexPuzzle),
}

fn solve(puzzle: &HexPuzzle) -> Option<HexPuzzle> {
    let mut stack: Vec<(HexPuzzle, Possible)> = vec![];
    let mut state = State::Search(puzzle.clone());

    loop {
        state = match state {
            State::Search(puzzle) => match puzzle.get_best() {
                None => return Some(puzzle.clone()),
                Some(possible) => State::Iterate((puzzle, possible)),
            },
            State::Iterate((mut puzzle, mut possible)) => match possible.vals.pop() {
                None => State::Backtrack,
                Some(val) => {
                    stack.push((puzzle.clone(), possible.clone()));
                    puzzle.set(possible.row, possible.col, val);
                    State::Search(puzzle)
                }
            },
            State::Backtrack => {
                match stack.pop() {
                    None => return None, // unsolveable
                    Some((puzzle, possible)) => State::Iterate((puzzle, possible)),
                }
            }
        };
    }
}
