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
        if p == 1 {
            Player(2)
        }
        else {
            Player(1)
        }
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
        let Player(p) = self.current_player();
        self.current_player = if p == 1 { Player(2) } else { Player(1) };
    }
}


pub trait State : fmt::Display + Clone {
    fn new(columns: usize, rows: usize) -> Self;
    fn size(&self) -> (usize, usize);
    fn row(&self, row: usize) -> Option<&Box<[Player]>>;
    fn column(&self, column: usize) -> Option<Box<[Player]>>;
    fn set(&mut self, row: usize, column: usize, player: Player);
    fn get(&self, column: usize, row: usize) -> Player;

    fn play(&mut self, column_number: usize, player: Player) -> Result<Self, InvalidMove> {
        if let Some(column) = self.column(column_number) {
            match column.iter().rposition(|&Player(p)| p == 0) {
                Some(position) => {
                    self.set(position, column_number, player);
                    Ok(self.clone())
                },
                None => Err(InvalidMove::ColumnFull(column_number))
            }
        }
        else {
            Err(InvalidMove::InvalidColumn(column_number))
        }
    }

    fn has_won(&self, player: Player) -> bool {
        let (columns, rows) = self.size();

        let winner_in = |stones: &[Player]| stones.windows(4).any(|window| {
            window.iter().all(|p| *p == player)
        });

        // TODO: Use iterators.
        let check_diagnoals = || {
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
        || check_diagnoals()
    }
}


#[derive(Clone, Debug)]
pub struct ArrayState {
    state: Box<[Box<[Player]>]>
}

impl State for ArrayState {
    fn new(columns: usize, rows: usize) -> Self {
        ArrayState {
            state: vec![vec![Player(0); columns].into_boxed_slice(); rows].into_boxed_slice()
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.state[0].len(), self.state.len())
    }

    fn row(&self, row: usize) -> Option<&Box<[Player]>> {
        self.state.get(row)
    }

    fn column(&self, column: usize) -> Option<Box<[Player]>> {
        if column >= self.size().0 {
            None
        }
        else {
            Some(self.state.iter().map(|row| row[column]).collect::<Vec<_>>().into_boxed_slice())
        }
    }

    fn set(&mut self, row: usize, column: usize, player: Player) {
        self.state[row][column] = player;
    }

    fn get(&self, column: usize, row: usize) -> Player {
        self.state[row][column]
    }
}

impl fmt::Display for ArrayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let columns = self.size().0;
        let fill_row = |left, joiner, right| {
            format!("{}{}{}", left, vec!["─"; columns].join(joiner), right)
        };

        let rows: Vec<_> = self.state.iter().map(|row| {
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
