use std::num::ParseIntError;
use thiserror::Error;

pub struct Sudoku([Cell; 81]);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell(u16);

impl Cell {
    pub fn new() -> Self {
        Self(0b1_1111_1111)
    }

    pub fn is_final(&self) -> bool {
        self.0 & (self.0 - 1) == 0
    }

    pub fn set(&mut self, value: Value) {
        self.0 = value.mask();
    }

    pub fn contains(&self, value: Value) -> bool {
        self.0 & value.mask() != 0
    }

    pub fn remove(&mut self, value: Value) {
        if !self.is_final() {
            self.0 &= !value.mask();
        }
    }

    pub fn value(&self) -> Option<Value> {
        if !self.is_final() {
            return None;
        }
        let n = match self.0 {
            1 => 1,
            2 => 2,
            4 => 3,
            8 => 4,
            16 => 5,
            32 => 6,
            64 => 7,
            128 => 8,
            256 => 9,
            _ => return None,
        };
        Some(Value::new(n).expect("valid value; qed"))
    }
}

impl std::str::FromStr for Cell {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut cell = Self::new();
        if string == " " {
            return Ok(cell);
        }
        let value: Value = string.parse()?;
        cell.set(value);
        Ok(cell)
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(value) = self.value() {
            write!(f, "{}", value)
        } else {
            write!(f, " ")
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Value(u8);

impl Value {
    pub fn new(value: u8) -> Result<Self, Error> {
        if value >= 1 && value <= 9 {
            Ok(Self(value - 1))
        } else {
            Err(Error::ValueOutOfRange)
        }
    }

    pub fn mask(&self) -> u16 {
        0b1 << self.0
    }
}

impl std::str::FromStr for Value {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::new(string.parse()?)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("value out of range")]
    ValueOutOfRange,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell() {
        let mut cell = Cell::new();
        for i in 2..=9 {
            let value = Value::new(i).unwrap();
            assert!(!cell.is_final());
            assert!(cell.value().is_none());
            assert!(cell.contains(value));
            cell.remove(value);
            assert!(!cell.contains(value));
        }
        assert!(cell.is_final());
        assert_eq!(cell.value(), Some(Value::new(1).unwrap()));

        let mut cell = Cell::new();
        cell.set(Value::new(3).unwrap());
        assert!(cell.is_final());
        assert_eq!(cell.value(), Some(Value::new(3).unwrap()));
    }
}
