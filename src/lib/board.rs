use tabled::{builder::Builder, Style, Width};

use super::{BoardError, Chip};

// Chips in a row needed to win
pub const CHIPS_IN_A_ROW: i32 = 4;

pub struct Board {
    state: Vec<Vec<Chip>>,
    pub size: usize,
    pub moves: u32,
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
        if col >= self.size {
            return Err(BoardError::InvalidColumn(col));
        }

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
        for i in (0.max(position.1 as i32 - CHIPS_IN_A_ROW - 1) as usize)
            ..self.size.min(position.1 + CHIPS_IN_A_ROW as usize)
        {
            if self.state[position.0][i] != chip {
                count = 0;
            } else {
                count += 1;
            }

            if count >= CHIPS_IN_A_ROW as u8 {
                return Some(chip);
            }
        }

        println!("Col count: {}", count);

        count = 0;

        //check row
        for i in (0.max(position.0 as i32 - CHIPS_IN_A_ROW - 1) as usize)
            ..self.size.min(position.0 + CHIPS_IN_A_ROW as usize)
        {
            if self.state[i][position.1] != chip {
                count = 0;
            } else {
                count += 1;
            }

            if count >= CHIPS_IN_A_ROW as u8 {
                return Some(chip);
            }
        }

        println!("Row count: {}", count);
        count = 0;

        let iposition = (position.0 as i32, position.1 as i32);
        //check diag
        for i in -(CHIPS_IN_A_ROW - 1)..CHIPS_IN_A_ROW {
            if iposition.0 + i < 0
                || iposition.0 + i >= self.size as i32
                || iposition.1 + i < 0
                || iposition.1 + i >= self.size as i32
            {
                continue;
            }

            if self.state[(iposition.0 + i) as usize][(iposition.1 + i) as usize] != chip {
                count = 0;
            } else {
                count += 1;
            }

            if count >= CHIPS_IN_A_ROW as u8 {
                return Some(chip);
            }
        }

        println!("Diag count: {}", count);
        count = 0;

        //check inverse diag
        for i in -(CHIPS_IN_A_ROW - 1)..CHIPS_IN_A_ROW {
            if iposition.0 + i < 0
                || iposition.0 + i >= self.size as i32
                || iposition.1 - i < 0
                || iposition.1 - i >= self.size as i32
            {
                continue;
            }

            if self.state[(iposition.0 + i) as usize][(iposition.1 - i) as usize] != chip {
                count = 0;
            } else {
                count += 1;
            }

            if count >= CHIPS_IN_A_ROW as u8 {
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

    pub fn to_string_from_bytes(bytes: Vec<u8>, size: usize) -> String {
        let tmp1: Vec<Chip> = bytes
            .into_iter()
            .map(|v| Chip::try_from(v).expect("Could not parse results"))
            .collect();
        let mut tmp: Vec<Vec<Chip>> = Vec::new();

        for i in 1..(size + 1) {
            tmp.push(tmp1[(i - 1) * size..i * size].to_vec());
        }

        let mut builder = Builder::default();

        for i in 0..size {
            let s = tmp.clone().into_iter().map(|v| v[i]).collect::<Vec<Chip>>();
            builder.add_record(s);
        }

        let mut indices = String::new();

        for i in 0..size {
            indices += &format!("  {} ", i);
        }

        return builder.build().with(Style::modern()).to_string() + "\n" + &indices;
    }

    pub fn as_1d(&mut self) -> Vec<u8> {
        let mut lol: Vec<u8> = Vec::new();

        self.state.clone().into_iter().for_each(|v| {
            v.into_iter()
                .for_each(|c| lol.push(c.try_into().expect("Could not convert chip to u8")))
        });

        return lol;
    }
}
