use std::env;

pub const BEGINNER: &'static str = "beginner";
pub const INTERMEDIATE: &'static str = "intermediate";
pub const EXPERT: &'static str = "expert";

#[derive(Copy, Clone, Debug)]
pub struct Options {
    pub level: &'static str,
    pub rows: i16,
    pub columns: i16,
    mines: i16,
}

impl Options {
    fn beginner() -> Options {
        Options {
            level: BEGINNER,
            rows: 9,
            columns: 9,
            mines: 10,
        }
    }
    fn intermediate() -> Options {
        Options {
            level: INTERMEDIATE,
            rows: 16,
            columns: 16,
            mines: 40,
        }
    }
    fn expert() -> Options {
        Options {
            level: EXPERT,
            rows: 16,
            columns: 30,
            mines: 99,
        }
    }

    pub fn new() -> Options {
        let args: Vec<_> = env::args().collect();
        match args.get(1).as_ref() {
            None => Options::beginner(),
            Some(skill_level) => match skill_level.as_ref() {
                BEGINNER => Options::beginner(),
                INTERMEDIATE => Options::intermediate(),
                EXPERT => Options::expert(),
                _ => Options::beginner(),
            },
        }
    }

    pub fn level(&self) -> &str {
        self.level
    }

    pub fn tiles(&self) -> i16 {
        self.rows * self.columns
    }

    pub fn blanks(&self) -> i16 {
        self.tiles() - self.mines
    }

    pub fn mines(&self) -> i16 {
        self.mines
    }

    pub fn row_column(&self, index: u16) -> (i16, i16) {
        (index as i16 / self.columns, index as i16 % self.columns)
    }

    pub fn index(&self, row: i16, column: i16) -> u16 {
        (row * self.columns + column) as u16
    }

    pub fn for_each_neighbor<F>(&self, index: u16, mut closure: F)
    where
        F: FnMut(i16, i16),
    {
        let (row, column) = self.row_column(index);
        for r in row - 1..=row + 1 {
            for c in column - 1..=column + 1 {
                if r != row || c != column {
                    if r >= 0 && r < self.rows && c >= 0 && c < self.columns {
                        closure(r, c)
                    }
                }
            }
        }
    }
}
