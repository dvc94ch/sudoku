use std::num::ParseIntError;
use thiserror::Error;

pub struct Sudoku([Cell; 81]);

impl Sudoku {
    pub fn new() -> Self {
        Self([Default::default(); 81])
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        let i = x * 9 + y;
        &self.0[i]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        let i = x * 9 + y;
        &mut self.0[i]
    }

    fn validate_group(&self, iter: impl Iterator<Item = (usize, usize)>) -> bool {
        let mut check = 0;
        for (x, y) in iter {
            let cell = self.get(x, y);
            if !cell.is_final() {
                return false;
            }
            check |= cell.0
        }
        check == 0b1_1111_1111
    }

    pub fn valid(&self) -> bool {
        for i in 0..9 {
            let row = self.validate_group(row_iter(i));
            let col = self.validate_group(col_iter(i));
            let block = self.validate_group(block_iter(i));
            if !(row && col && block) {
                return false;
            }
        }
        true
    }
}

fn row_iter(row: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..9).map(move |field| (row, field))
}

fn col_iter(col: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..9).map(move |field| (field, col))
}

fn block_iter(block: usize) -> impl Iterator<Item = (usize, usize)> {
    let offset = ((block % 3) * 3, (block / 3) * 3);
    std::iter::once(0)
        .cycle()
        .zip(0..3)
        .chain(std::iter::once(1).cycle().zip(0..3))
        .chain(std::iter::once(2).cycle().zip(0..3))
        .map(move |(x, y)| (x + offset.0, y + offset.1))
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl std::str::FromStr for Sudoku {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut sudoku = Sudoku::new();
        for (x, row) in string.split('\n').enumerate() {
            for (y, c) in row.chars().enumerate() {
                let value = c.to_string().parse()?;
                sudoku.get_mut(x, y).set(value);
            }
        }
        Ok(sudoku)
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for x in 0..9 {
            for y in 0..9 {
                write!(f, "{}", self.get(x, y))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

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

impl Default for Cell {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn test_block_iter() {
        let indices: Vec<_> = block_iter(0).collect();
        assert_eq!(
            indices,
            &[
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 1),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ]
        );
    }

    #[test]
    fn test_validate_sudoku() {
        let sudoku: Sudoku = "534678912\n\
             672195348\n\
             198342567\n\
             859761423\n\
             426853791\n\
             713924856\n\
             961537284\n\
             287419635\n\
             345286179"
            .parse()
            .unwrap();
        println!("{}", sudoku);
        assert!(sudoku.valid());
    }
}
