use anyhow::Result;

#[derive(Clone, Copy)]
pub struct Sudoku {
    cells: [Value; 81],
}

impl Sudoku {
    pub fn new() -> Self {
        Self {
            cells: [Default::default(); 81],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Value {
        let i = x * 9 + y;
        self.cells[i]
    }

    pub fn set(&mut self, x: usize, y: usize, value: Value) {
        let i = x * 9 + y;
        self.cells[i] = value;
    }

    fn validate_group(&self, iter: impl Iterator<Item = (usize, usize)>) -> Solution {
        let mut values = [false; 10];
        for (x, y) in iter {
            let value = self.get(x, y);
            let index = value.value() as usize;
            if value.is_final() && values[index] {
                return Solution::Invalid;
            }
            values[index] = true;
        }
        if values[0] {
            Solution::Incomplete
        } else {
            Solution::Valid
        }
    }

    fn validate(&self) -> Solution {
        let mut incomplete = false;
        for i in 0..9 {
            match self.validate_group(row_iter(i)) {
                Solution::Invalid => return Solution::Invalid,
                Solution::Incomplete => incomplete = true,
                _ => {}
            }
            match self.validate_group(col_iter(i)) {
                Solution::Invalid => return Solution::Invalid,
                Solution::Incomplete => incomplete = true,
                _ => {}
            }
            match self.validate_group(block_iter(i)) {
                Solution::Invalid => return Solution::Invalid,
                Solution::Incomplete => incomplete = true,
                _ => {}
            }
        }
        if incomplete {
            Solution::Incomplete
        } else {
            Solution::Valid
        }
    }

    pub fn valid(&self) -> bool {
        self.validate() == Solution::Valid
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Solution {
    Valid,
    Invalid,
    Incomplete,
}

fn row_iter(row: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..9).map(move |field| (row, field))
}

fn col_iter(col: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..9).map(move |field| (field, col))
}

fn block_iter(block: usize) -> impl Iterator<Item = (usize, usize)> {
    let offset = ((block % 3) * 3, (block / 3) * 3);
    std::iter::repeat(0)
        .zip(0..3)
        .chain(std::iter::repeat(1).zip(0..3))
        .chain(std::iter::repeat(2).zip(0..3))
        .map(move |(x, y)| (x + offset.0, y + offset.1))
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl std::str::FromStr for Sudoku {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut sudoku = Sudoku::new();
        for (x, row) in string.split('\n').enumerate() {
            for (y, c) in row.chars().enumerate() {
                if c == ' ' {
                    continue;
                }
                let cell = c.to_string().parse()?;
                sudoku.set(x, y, cell);
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
pub struct Value(u8);

impl Value {
    pub fn new(value: u8) -> Result<Self> {
        if value > 9 {
            anyhow::bail!("invalid value");
        }
        Ok(Self(value))
    }

    pub fn is_final(&self) -> bool {
        self.0 > 0
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::new(0).unwrap()
    }
}

impl std::str::FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string == " " {
            return Ok(Self::default());
        } else {
            let value = string.parse()?;
            if value == 0 {
                anyhow::bail!("value out of range");
            }
            Self::new(value)
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_final() {
            write!(f, "{}", self.value())
        } else {
            write!(f, " ")
        }
    }
}

fn backtrack(root: Sudoku, mut level: usize) -> Option<Sudoku> {
    match root.validate() {
        Solution::Valid => return Some(root),
        Solution::Invalid => return None,
        Solution::Incomplete => {}
    }
    while root.cells[level].is_final() {
        level += 1;
    }
    for v in 1..=9 {
        let mut candidate = root;
        candidate.cells[level] = Value::new(v).unwrap();
        if let Some(solution) = backtrack(candidate, level + 1) {
            return Some(solution);
        }
    }
    None
}

pub fn solve(sudoku: Sudoku) -> Option<Sudoku> {
    backtrack(sudoku, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_solve_sudoku() {
        let sudoku: Sudoku = "53  7    \n\
             6  195   \n \
              98    6 \n\
             8   6   3\n\
             4  8 3  1\n\
             7   2   6\n \
              6    28 \n   \
                419  5\n    \
                 8  79"
            .parse()
            .unwrap();
        let solution = solve(sudoku).expect("unsat");
        println!("{}", solution);
        assert!(solution.valid());
    }
}
