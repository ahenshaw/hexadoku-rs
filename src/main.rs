use hexadoku::HexPuzzle;
use std::env;
use std::path::PathBuf;
use std::process;
use std::time::Instant;

fn solve(puzzle: &HexPuzzle) -> Option<HexPuzzle> {
    // find the cell with the least number of possibles
    match puzzle.get_best() {
        // No cells left, puzzle solved
        None => return Some(puzzle.clone()),
        Some(possible) => {
            if possible.vals.len() == 0 {
                // there is a cell with no possible right answers
                // therefore, we need to backtrack
                return None;
            }

            for val in possible.vals {
                let mut puzzle = puzzle.clone();
                puzzle.set(possible.row, possible.col, val);
                match solve(&puzzle) {
                    None => (),
                    Some(puzzle) => {
                        let puzzle = puzzle.clone();
                        return Some(puzzle);
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Expected a puzzle file name");
        process::exit(1);
    }
    let filename = PathBuf::from(&args[1]);

    match HexPuzzle::new_from_file(&filename) {
        Ok(puzzle) => {
            println!("Input: {}", filename.display());
            println!("{puzzle}");
            let start = Instant::now();
            let solution = solve(&puzzle).unwrap();
            println!("Solved in {:?}", Instant::now() - start);
            println!("{}", solution);
        }
        Err(e) => eprintln!("File error: {}", e),
    }
}
