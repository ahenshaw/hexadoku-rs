use anyhow::Result;
use bit_set::BitSet;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const EMPTY: u8 = 255;

#[derive(Clone, Debug)]
pub struct Possible {
    pub vals: Vec<usize>,
    pub row: usize,
    pub col: usize,
}

#[derive(Clone, Debug)]
pub struct HexPuzzle {
    pub grid: [[u8; 16]; 16],
    pub cols: [BitSet; 16],
    pub rows: [BitSet; 16],
    pub squares: [BitSet; 16],
}

impl HexPuzzle {
    pub fn new() -> Self {
        let cols: [BitSet; 16] = (0..16)
            .map(|_| BitSet::with_capacity(16))
            .collect::<Vec<BitSet>>()
            .try_into()
            .unwrap();
        let rows = cols.clone();
        let squares = cols.clone();

        Self {
            grid: [[EMPTY; 16]; 16],
            rows,
            cols,
            squares,
        }
    }

    pub fn new_from_file(filename: &Path) -> Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut index = 0;
        let mut puzzle = HexPuzzle::new();
        for line in reader.lines() {
            match line {
                Ok(line) if line.len() > 0 => {
                    for val in line.split_ascii_whitespace() {
                        match val {
                            "*" => (),
                            _ => {
                                let val = u8::from_str_radix(val, 16)?;
                                let row = index / 16;
                                let col = index & 0xf;
                                puzzle.set(row, col, val as usize);
                            }
                        };
                        index += 1;
                    }
                }
                _ => (),
            }
            if index == 256 {
                break;
            }
        }
        Ok(puzzle)
    }

    pub fn set(&mut self, row: usize, col: usize, value: usize) -> bool {
        assert!(value < 16);
        if !self.rows[row].insert(value as usize) {
            return false;
        };
        if !self.cols[col].insert(value as usize) {
            return false;
        };
        let sq_index = (row / 4) * 4 + col / 4;
        if !self.squares[sq_index].insert(value as usize) {
            return false;
        };

        self.grid[row][col] = value as u8;
        true
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.grid[row][col]
    }

    pub fn get_possible(&self, row: usize, col: usize) -> Vec<usize> {
        let sq_index = (row / 4) * 4 + col / 4;
        let mut active = self.rows[row].clone();
        active.union_with(&self.cols[col]);
        active.union_with(&self.squares[sq_index]);
        let mut possible = BitSet::from_bytes(&[0xff; 2]);
        possible.difference_with(&active);
        possible.iter().collect()
    }

    pub fn get_best(&self) -> Option<Possible> {
        let mut possibles: Vec<Possible> = vec![];
        for row in 0..16 {
            for col in 0..16 {
                if self.get(row, col) == EMPTY {
                    possibles.push(Possible {
                        row,
                        col,
                        vals: self.get_possible(row, col),
                    })
                }
            }
        }
        if possibles.len() == 0 {
            None
        } else {
            possibles.sort_by_key(|p| p.vals.len());
            Some(possibles.first().unwrap().clone())
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut squares: [BitSet; 16] = (0..16)
            .map(|_| BitSet::with_capacity(16))
            .collect::<Vec<BitSet>>()
            .try_into()
            .unwrap();
        let mut cols = squares.clone();

        for row in 0..16 {
            let mut row_set = BitSet::with_capacity(16);
            let big_row = row / 4;
            for col in 0..16 {
                let value = self.get(row, col) as usize;
                if value != EMPTY.into() {
                    let sq_index = big_row * 4 + col / 4;
                    if !row_set.insert(value) {
                        return false;
                    }
                    if !cols[col].insert(value) {
                        return false;
                    }
                    if !squares[sq_index].insert(value) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl fmt::Display for HexPuzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, " {}", "┌─────────┬─────────┬─────────┬─────────┐")?;
        for row in 0..16 {
            for col in 0..16 {
                let c = match self.get(row, col) {
                    EMPTY => "·".to_string(),
                    value => format!("{value:x}"),
                };
                if col.rem_euclid(4) == 0 {
                    write!(f, " │")?;
                }
                write!(f, " {c}")?;
            }
            writeln!(f, " │")?;
            if (row + 1).rem_euclid(4) == 0 {
                if row != 15 {
                    writeln!(f, " {}", "├─────────┼─────────┼─────────┼─────────┤")?
                } else {
                    writeln!(f, " {}", "└─────────┴─────────┴─────────┴─────────┘")?
                }
            }
        }
        Ok(())
    }
}
