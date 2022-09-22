pub mod client;
mod errors;
pub mod server;

use std::fmt::{self, Display};

pub use errors::BoardError;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Chip {
    YELLOW,
    RED,
    NONE,
}

impl Display for Chip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Chip::YELLOW => write!(f, "Y"),
            Chip::RED => write!(f, "R"),
            Chip::NONE => write!(f, " "),
        }
    }
}

pub struct Board {
    state: Vec<Vec<Chip>>,
    size: usize,
    moves: u32,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            state: vec![vec![Chip::NONE; size]; size],
            size,
            moves: 0,
        }
    }

    pub fn drop_chip(&mut self, col: usize, chip: Chip) -> Result<Option<Chip>, BoardError> {
        if self.state[col][0] != Chip::NONE {
            return Err(BoardError::FullColumn);
        }

        self.moves += 1;
        let mut dropped_at: Option<(usize, usize)> = None;

        for i in 0..(self.size - 1){
            // println!("Checking if cell ({}, {}) is not NONE. Value: {:?}", col, i+1, self.state[col][i+1]);
            // println!("i+1 == self.size --> {}+1 == {} --> {}", i, self.size, i+1 == self.size);

            if self.state[col][i+1] != Chip::NONE {
                self.state[col][i] = chip.clone();

                println!("New state: {}", self.to_string());

                dropped_at = Some((col, i));
                break;
            }
        }

        if dropped_at.is_none() {
            self.state[col][self.size-1] = chip.clone();

            println!("New state: {}", self.to_string());

            dropped_at = Some((col, self.size-1));
        }

        return Ok(self.check_state(dropped_at.unwrap(), chip));
    }

    /// Returns either None or a winner
    pub fn check_state(&self, position: (usize, usize), chip: Chip) -> Option<Chip> {
        //check col
        for i in 0..self.size {
            if self.state[position.0][i] != chip {
                break;
            }
            if i == self.size - 1 {
                return Some(chip);
            }
        }

        //check row
        for i in 0..self.size {
            if self.state[i][position.1] != chip {
                break;
            }
            if i == self.size - 1 {
                return Some(chip);
            }
        }

        //check diag
        if position.0 == position.1 {
            //we're on a diagonal
            for i in 0..self.size {
                if self.state[i][i] != chip {
                    break;
                }
                if i == self.size - 1 {
                    return Some(chip);
                }
            }
        }

        //check draw
        if self.moves == (self.size * self.size - 1) as u32 {
            return Some(Chip::NONE);
        }

        return None;
    }

    pub fn to_string(&mut self) -> String {
        let mut out = String::new();

        for j in 0..self.size {
            for i in 0..self.size {
                out += &format!("| {} ", self.state[i][j]);
            }

            out += "|\n";
        }

        for _ in 0..self.size {
            out += "----";
        }

        return out + "-\n";
    }
}
