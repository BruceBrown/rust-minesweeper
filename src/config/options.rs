use std::env;

pub const BEGINNER: &str = "beginner";
pub const INTERMEDIATE: &str = "intermediate";
pub const EXPERT: &str = "expert";

/**
 * Minesweeper configuration options.
 *
 * minesweeper level [beginner|intermediate|expert]
 *
 * Its a bit messy, but the goal is to have everything compile down to constants. This should improve the layout
 * engine, which also should compile down to constants.
 */

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Options {
    level: &'static str,
    pub rows: i16,
    pub columns: i16,
    mines: i16,
}

pub const BEGINNER_OPTIONS: Options = Options {
    level: BEGINNER,
    rows: 9,
    columns: 9,
    mines: 10,
};

pub const INTERMEDIATE_OPTIONS: Options = Options {
    level: INTERMEDIATE,
    rows: 16,
    columns: 16,
    mines: 40,
};

pub const EXPERT_OPTIONS: Options = Options {
    level: EXPERT,
    rows: 16,
    columns: 30,
    mines: 99,
};

impl Options {
    pub fn new() -> Options {
        let args: Vec<_> = env::args().collect();
        Options::new_with_args(args)
    }

    pub fn new_with_args(args: Vec<String>) -> Options {
        match args.get(1).as_ref() {
            None => BEGINNER_OPTIONS,
            Some(skill_level) => match skill_level.as_ref() {
                BEGINNER => BEGINNER_OPTIONS,
                INTERMEDIATE => INTERMEDIATE_OPTIONS,
                EXPERT => EXPERT_OPTIONS,
                _ => BEGINNER_OPTIONS,
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

#[cfg(test)]
mod tests {
    use super::Options;

    #[test]
    fn test_construction() {
        assert_eq!(
            super::BEGINNER_OPTIONS,
            Options {
                level: "beginner",
                rows: 9,
                columns: 9,
                mines: 10
            }
        );
        assert_eq!(
            super::INTERMEDIATE_OPTIONS,
            Options {
                level: "intermediate",
                rows: 16,
                columns: 16,
                mines: 40
            }
        );
        assert_eq!(
            super::EXPERT_OPTIONS,
            Options {
                level: "expert",
                rows: 16,
                columns: 30,
                mines: 99
            }
        );
    }

    #[test]
    fn test_command_line() {
        let mut args = vec!["minesweeper".to_string(), "beginner".to_string()];
        assert_eq!(Options::new_with_args(args), super::BEGINNER_OPTIONS);

        args = vec!["minesweeper".to_string(), "intermediate".to_string()];
        assert_eq!(Options::new_with_args(args), super::INTERMEDIATE_OPTIONS);

        args = vec!["minesweeper".to_string(), "expert".to_string()];
        assert_eq!(Options::new_with_args(args), super::EXPERT_OPTIONS);

        args = vec!["minesweeper".to_string()];
        assert_eq!(Options::new_with_args(args), super::BEGINNER_OPTIONS);

        args = vec!["minesweeper".to_string(), "wrong".to_string()];
        assert_eq!(Options::new_with_args(args), super::BEGINNER_OPTIONS);
    }

    #[test]
    fn test_attributes() {
        fn run(options: &Options) {
            assert_eq!(options.rows * options.columns, options.tiles());
            assert_eq!(options.mines, options.mines());
            assert_eq!(
                options.rows * options.columns - options.mines,
                options.blanks()
            );
        }

        assert_eq!(super::BEGINNER_OPTIONS.level(), "beginner");
        run(&super::BEGINNER_OPTIONS);
        assert_eq!(super::INTERMEDIATE_OPTIONS.level(), "intermediate");
        run(&super::INTERMEDIATE_OPTIONS);
        assert_eq!(super::EXPERT_OPTIONS.level(), "expert");
        run(&super::EXPERT_OPTIONS);
    }

    #[test]
    fn test_for_each_neighbor() {
        fn run(options: &Options, index: u16) -> Vec<u16> {
            let mut visit: Vec<u16> = Vec::new();
            let closure = |row, col| visit.push(options.index(row, col));
            options.for_each_neighbor(index, closure);
            visit
        }

        assert_eq!(run(&super::BEGINNER_OPTIONS, 0), vec![1, 9, 10]);
        assert_eq!(
            run(&super::BEGINNER_OPTIONS, 10),
            vec![0, 1, 2, 9, 11, 18, 19, 20]
        );
        assert_eq!(run(&super::BEGINNER_OPTIONS, 80), vec![70, 71, 79]);

        assert_eq!(run(&super::INTERMEDIATE_OPTIONS, 0), vec![1, 16, 17]);
        assert_eq!(
            run(&super::INTERMEDIATE_OPTIONS, 17),
            vec![0, 1, 2, 16, 18, 32, 33, 34]
        );
        assert_eq!(
            run(&super::INTERMEDIATE_OPTIONS, 16 * 16 - 1),
            vec![238, 239, 254]
        );

        assert_eq!(run(&super::EXPERT_OPTIONS, 0), vec![1, 30, 31]);
        assert_eq!(
            run(&super::EXPERT_OPTIONS, 31),
            vec![0, 1, 2, 30, 32, 60, 61, 62]
        );
        assert_eq!(
            run(&super::EXPERT_OPTIONS, 16 * 30 - 1),
            vec![448, 449, 478]
        );
    }
}
