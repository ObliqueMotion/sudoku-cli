use super::data::SudokuData;
use std::fmt;

#[derive(Clone, Debug, Default)]
pub struct SudokuBoard {
    state: [SudokuData; 9],
}

impl SudokuBoard {
    pub fn insert(&self, value: u64, row: usize, col: usize) {
        assert!((1..=9).contains(&value));
        assert!((0..=8).contains(&row));
        assert!((0..=8).contains(&col));
    }
}

impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┌───────┬───────┬───────┐")?;
        writeln!(f, "{}", self.state[0])?;
        writeln!(f, "{}", self.state[1])?;
        writeln!(f, "{}", self.state[2])?;
        writeln!(f, "├───────┼───────┼───────┤")?;
        writeln!(f, "{}", self.state[3])?;
        writeln!(f, "{}", self.state[4])?;
        writeln!(f, "{}", self.state[5])?;
        writeln!(f, "├───────┼───────┼───────┤")?;
        writeln!(f, "{}", self.state[6])?;
        writeln!(f, "{}", self.state[7])?;
        writeln!(f, "{}", self.state[8])?;
        writeln!(f, "└───────┴───────┴───────┘")
    }
}
