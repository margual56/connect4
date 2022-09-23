pub mod client;
mod errors;
pub mod server;

use std::fmt::{self, Display};
use tabled::{builder::Builder, Style, Tabled};

pub use errors::BoardError;

// Chips in a row needed to win
pub const CHIPS_IN_A_ROW: u8 = 3;

#[derive(Clone, Copy, PartialEq, Debug, Tabled)]
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

        for i in 0..(self.size - 1) {
            // println!("Checking if cell ({}, {}) is not NONE. Value: {:?}", col, i+1, self.state[col][i+1]);
            // println!("i+1 == self.size --> {}+1 == {} --> {}", i, self.size, i+1 == self.size);

            if self.state[col][i + 1] != Chip::NONE {
                self.state[col][i] = chip.clone();

                dropped_at = Some((col, i));
                break;
            }
        }

        if dropped_at.is_none() {
            self.state[col][self.size - 1] = chip.clone();

            dropped_at = Some((col, self.size - 1));
        }

        return Ok(self.check_state(dropped_at.unwrap(), chip));
    }

    /// Returns either None or a winner
    pub fn check_state(&self, position: (usize, usize), chip: Chip) -> Option<Chip> {
        let mut count: u8 = 0;
        //check col
        for i in (0.max(position.1 as i32 - 2) as usize)..self.size.min(position.1 + 3) {
            if self.state[position.0][i] != chip {
                count = 0;
            } else {
                count += 1;
            }

            if count >= CHIPS_IN_A_ROW {
                return Some(chip);
            }
        }

        println!("Col count: {}", count);

        count = 0;

        //check row
        for i in (0.max(position.0 as i32 - 2) as usize)..self.size.min(position.0 + 3) {
            if self.state[i][position.1] != chip {
                count = 0;
            } else {
                count += 1;
            }

            if count >= CHIPS_IN_A_ROW {
                return Some(chip);
            }
        }

        println!("Row count: {}", count);
        count = 0;

        let iposition = (position.0 as i32, position.1 as i32);
        //check diag
        for i in -2..3 {
            if iposition.0 + i < 0
                || iposition.0 + i >= self.size as i32
                || iposition.1 + i < 0
                || iposition.1 + i >= self.size as i32
            {
                continue;
            }

            if self.state[(iposition.0+i) as usize][(iposition.1+i) as usize] != chip {
                count = 0;
            }else{
                count += 1;
            }

            if count >= CHIPS_IN_A_ROW {
                return Some(chip);
            }
        }

        println!("Diag count: {}", count);
        count = 0;

        //check inverse diag
        for i in -2..3 {
            if iposition.0 + i < 0
                || iposition.0 + i >= self.size as i32
                || iposition.1 - i < 0
                || iposition.1 - i >= self.size as i32
            {
                continue;
            }

            if self.state[(iposition.0+i) as usize][(iposition.1-i) as usize] != chip {
                count = 0;
            }else{
                count += 1;
            }

            if count >= CHIPS_IN_A_ROW {
                return Some(chip);
            }
        }

        println!("Inverse diag count: {}", count);

        //check draw
        if self.moves == (self.size * self.size - 1) as u32 {
            return Some(Chip::NONE);
        }

        return None;
    }

    pub fn to_string(&mut self) -> String {
        let mut builder = Builder::default();

        for i in 0..self.size {
            let s = self
                .state
                .clone()
                .into_iter()
                .map(|v| v[i])
                .collect::<Vec<Chip>>();
            builder.add_record(s);
        }

        return builder.build().with(Style::modern()).to_string();
    }
}
