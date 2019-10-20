use super::data::SudokuData;
use std::fmt;

#[derive(Clone, Debug, Default)]
pub struct SudokuBoard {
    d0: SudokuData,
    d1: SudokuData,
    d2: SudokuData,
    d3: SudokuData,
    d4: SudokuData,
    d5: SudokuData,
    d6: SudokuData,
    d7: SudokuData,
    d8: SudokuData,
}

impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┌───────┬───────┬───────┐");
        writeln!(f, "{}", self.d0);
        writeln!(f, "{}", self.d1);
        writeln!(f, "{}", self.d2);
        writeln!(f, "├───────┼───────┼───────┤");
        writeln!(f, "{}", self.d3);
        writeln!(f, "{}", self.d4);
        writeln!(f, "{}", self.d5);
        writeln!(f, "├───────┼───────┼───────┤");
        writeln!(f, "{}", self.d6);
        writeln!(f, "{}", self.d7);
        writeln!(f, "{}", self.d8);
        writeln!(f, "└───────┴───────┴───────┘")
    }
}
