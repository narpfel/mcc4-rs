use std::fmt;

#[derive(Debug, Clone)]
pub struct Game {
    current_player: Player,
    state: ArrayState,
}

impl Game {
    pub fn new(columns: usize, rows: usize) -> Game {
        Game {
            current_player: Player(1),
            // TODO: Make generic over `state`s type‽
            state: ArrayState::new(columns, rows)
        }
    }

    pub fn size(&self) -> (usize, usize) {
        self.state.size()
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn other_player(&self) -> Player {
        let Player(p) = self.current_player();
        Player(3 - p)
    }

    pub fn winner(&self) -> Option<Player> {
        if self.state.has_won(self.current_player()) {
            Some(self.current_player())
        }
        else if self.state.has_won(self.other_player()) {
            Some(self.other_player())
        }
        else {
            None
        }
    }

    pub fn play(&mut self, column_number: usize) -> Result<ArrayState, InvalidMove> {
        let player = self.current_player();
        let new_state = self.state.play(column_number, player);
        if new_state.is_ok() {
            self.next_player();
        }
        new_state
    }

    pub fn state(&self) -> &ArrayState {
        &self.state
    }

    fn next_player(&mut self) {
        self.current_player = self.other_player();
    }
}


pub trait State : fmt::Display + Clone {
    fn new(columns: usize, rows: usize) -> Self;
    fn size(&self) -> (usize, usize);
    fn row(&self, row: usize) -> Option<&[Player]>;
    fn column(&self, column: usize) -> Option<&[Player]>;
    fn set(&mut self, column: usize, row: usize, player: Player);
    fn get(&self, column: usize, row: usize) -> Player;

    fn play(&mut self, column_number: usize, player: Player) -> Result<Self, InvalidMove> {
        let row = match self.column(column_number) {
            Some(column) => match column.iter().rposition(|&Player(p)| p == 0) {
                Some(row) => row,
                None => return Err(InvalidMove::ColumnFull(column_number))
            },
            None => return Err(InvalidMove::InvalidColumn(column_number))
        };
        self.set(column_number, row, player);
        Ok(self.clone())
    }

    fn has_won(&self, player: Player) -> bool {
        let (columns, rows) = self.size();

        let winner_in = |stones: &[Player]| stones.windows(4).any(|window| {
            window.iter().all(|p| *p == player)
        });

        // TODO: Use iterators.
        let winner_in_diagonals = || {
            for row in 3..rows {
                for column in 3..columns {
                    if (0..4).map(|i| self.get(column - i, row - i)).all(|p| p == player) {
                        return true;
                    }
                }
            }
            for row in 3..rows {
                for column in 0..columns - 3 {
                    if (0..4).map(|i| self.get(column + i, row - i)).all(|p| p == player) {
                        return true;
                    }
                }
            }
            return false;
        };

        (0..rows).any(|row_number| {
            winner_in(self.row(row_number).unwrap())
        })
        || (0..columns).any(|column_number| {
            winner_in(&*self.column(column_number).unwrap())
        })
        || winner_in_diagonals()
    }
}


#[derive(Clone, Debug)]
pub struct ArrayState {
    state: Vec<Player>,
    state_t: Vec<Player>,
    columns: usize,
    rows: usize,
}

impl State for ArrayState {
    fn new(columns: usize, rows: usize) -> Self {
        ArrayState {
            state: vec![Player(0); rows * columns],
            state_t: vec![Player(0); columns * rows],
            columns: columns,
            rows: rows,
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.columns, self.rows)
    }

    fn row(&self, row: usize) -> Option<&[Player]> {
        self.state.chunks(self.columns).nth(row)
    }

    fn column(&self, column: usize) -> Option<&[Player]> {
        self.state_t.chunks(self.rows).nth(column)
    }

    fn set(&mut self, column: usize, row: usize, player: Player) {
        self.state[row * self.columns + column] = player;
        self.state_t[column * self.rows + row] = player;
    }

    fn get(&self, column: usize, row: usize) -> Player {
        self.state[row * self.columns + column]
    }
}

impl fmt::Display for ArrayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let columns = self.size().0;
        let fill_row = |left, joiner, right| {
            format!("{}{}{}", left, vec!["─"; columns].join(joiner), right)
        };

        let rows: Vec<_> = self.state.chunks(self.columns).map(|row| {
            let positions: Vec<_> = row.iter().map(|p| format!("{}", p)).collect();
            format!("│{}│\n", positions.join("│"))
        }).collect();

        try!(write!(
            f, "{top_row}{body}{bottom_row}",
            top_row = fill_row("┌", "┬", "┐\n"),
            body = rows.join(&fill_row("├", "┼", "┤\n")),
            bottom_row = fill_row("└", "┴", "┘\n")
        ));
        write!(f, " {}\n", (1..columns + 1).map(|n| format!("{}", n)).collect::<Vec<_>>().join(" "))
    }
}


#[derive(Debug)]
pub enum InvalidMove {
    InvalidColumn(usize),
    ColumnFull(usize),
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Player(pub u8);

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "{}\x1B[0m",
            if self.0 == 1 { "\x1B[44;1mX" } else if self.0 == 2 { "\x1B[41;1mO" } else { " " }
        )
    }
}
