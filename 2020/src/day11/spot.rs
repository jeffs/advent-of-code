use crate::error::ParseError;
use std::fmt;

/// The state of some position in a Grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Spot {
    Floor,
    Empty,
    Occupied,
}

impl Spot {
    fn from_char(c: char) -> Result<Spot, ParseError> {
        match c {
            '.' => Ok(Spot::Floor),
            'L' => Ok(Spot::Empty),
            '#' => Ok(Spot::Occupied),
            _ => Err(ParseError::new(format!("{}: bad spot", c))),
        }
    }

    /// Returns the next value of this spot, given the specified number of
    /// neighbors.
    pub fn next1(&self, n: usize) -> Spot {
        match self {
            Spot::Empty if n == 0 => Spot::Occupied,
            Spot::Occupied if n > 3 => Spot::Empty,
            _ => *self,
        }
    }

    /// Returns the next value of this spot, given the specified number of
    /// neighbors.
    pub fn next2(&self, n: usize) -> Spot {
        match self {
            Spot::Empty if n == 0 => Spot::Occupied,
            Spot::Occupied if n > 4 => Spot::Empty,
            _ => *self,
        }
    }

    pub fn parse_line(line: &str) -> Result<Vec<Spot>, ParseError> {
        line.chars().map(Spot::from_char).collect()
    }
}

impl fmt::Display for Spot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Spot::Floor => write!(f, "."),
            Spot::Empty => write!(f, "L"),
            Spot::Occupied => write!(f, "#"),
        }
    }
}
