use super::data::SudokuData;
use std::fmt;

#[derive(Clone, Debug, Default)]
pub struct SudokuBoard {
    state: [SudokuData; 9],
}

fn box_index(row: usize, col: usize) -> usize {
    match row {
        0 | 1 | 2 => match col {
            0 | 1 | 2 => 0,
            3 | 4 | 5 => 1,
            _________ => 2,
        },
        3 | 4 | 5 => match col {
            0 | 1 | 2 => 3,
            3 | 4 | 5 => 4,
            _________ => 5,
        },
        _________ => match col {
            0 | 1 | 2 => 6,
            3 | 4 | 5 => 7,
            _________ => 8,
        },
    }
}

impl SudokuBoard {
    pub fn insert(&self, value: u64, row: usize, col: usize) {
        assert!((1..=9).contains(&value));
        assert!((0..=8).contains(&row));
        assert!((0..=8).contains(&col));
        let bx = box_index(row, col);
        self.state[row].set_cell(value, col);
        self.state[row].mark_in_row(value);
        self.state[col].mark_in_col(value);
        self.state[bx ].mark_in_box(value);
    }
}

impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  ╔═══════════╦═══════════╦═══════════╗")?;
        writeln!(f, "  {}", self.state[0])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.state[1])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.state[2])?;
        writeln!(f, "  ╠═══════════╬═══════════╬═══════════╣")?;
        writeln!(f, "  {}", self.state[3])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.state[4])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.state[5])?;
        writeln!(f, "  ╠═══════════╬═══════════╬═══════════╣")?;
        writeln!(f, "  {}", self.state[6])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.state[7])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.state[8])?;
        writeln!(f, "  ╚═══════════╩═══════════╩═══════════╝")
    }
}