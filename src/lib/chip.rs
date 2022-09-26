use std::fmt::{self, Display};
use tabled::Tabled;
use colored::Colorize;

#[derive(Clone, Copy, PartialEq, Debug, Tabled)]
pub enum Chip {
    YELLOW,
    RED,
    NONE,
}

impl Display for Chip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Chip::YELLOW => write!(f, "{}", "Y".yellow()),
            Chip::RED => write!(f, "{}", "R".red()),
            Chip::NONE => write!(f, "{}", " ".yellow()),
        }
    }
}   

impl TryFrom<Chip> for u8 {
    type Error = &'static str;

    fn try_from(value: Chip) -> Result<Self, Self::Error> {
        match value {
            Chip::NONE => Ok(0 as u8),
            Chip::YELLOW => Ok(1 as u8),
            Chip::RED => Ok(2 as u8),
        }
    }
}
impl TryFrom<u8> for Chip {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Chip::NONE),
            1 => Ok(Chip::YELLOW),
            2 => Ok(Chip::RED),
            _ => Err("Value does not correspond to any enum type"),
        }
    }
}
