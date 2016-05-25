use std::fmt;

use super::{Game, Player};

#[derive(Debug, Clone)]
pub struct ConnectFour<S: State> {
    state: S,
    current_player: Player,
    winner: Option<Player>,
}

impl<S: State> ConnectFour<S> {
    pub fn new(columns: usize, rows: usize) -> Result<ConnectFour<S>, ()> {
        Ok(
            ConnectFour {
                current_player: Player(1),
                state: try!(S::new(columns, rows)),
                winner: None,
            }
        )
    }

    pub fn size(&self) -> (usize, usize) {
        self.state.size()
    }

    // Not in trait, because it assumes two players.
    pub fn other_player(&self) -> Player {
        let Player(p) = self.current_player();
        Player(3 - p)
    }
}

impl<S: State> Game for ConnectFour<S> {
    type State = S;
    type Move = usize;
    type InvalidMove = InvalidMove;

    fn play(&mut self, column_number: Self::Move) -> Result<Option<Player>, InvalidMove> {
        let player = self.current_player();
        try!(self.state.play(column_number, player));
        self.next_player();
        if self.state.has_just_won() {
            self.winner = Some(player);
            Ok(self.winner)
        }
        else {
            Ok(None)
        }
    }

    fn winner(&self) -> Option<Player> {
        self.winner
    }

    fn valid_moves(&self) -> Vec<Self::Move> {
        let columns = self.size().0;
        (0..columns).filter(|&column| self.state().get(column, 0) == Player(0)).collect()
    }

    fn state(&self) -> &S {
        &self.state
    }

    fn current_player(&self) -> Player {
        self.current_player
    }

    fn next_player(&mut self) {
        self.current_player = self.other_player();
    }
}


pub trait State : fmt::Display + Clone + Send + Sync {
    fn new(columns: usize, rows: usize) -> Result<Self, ()>;
    fn size(&self) -> (usize, usize);
    fn set(&mut self, column: usize, row: usize, player: Player);
    fn get(&self, column: usize, row: usize) -> Player;
    fn last_move(&self) -> (usize, usize);

    fn row(&self, row: usize) -> Option<Box<[Player]>> {
        if row >= self.size().1 {
            return None;
        }
        Some(
            (0..self.size().0)
            .map(|column| self.get(column, row))
            .collect::<Vec<_>>().into_boxed_slice()
        )
    }

    fn column(&self, column: usize) -> Option<Box<[Player]>> {
        if column >= self.size().0 {
            return None;
        }
        Some(
            (0..self.size().1)
            .map(|row| self.get(column, row))
            .collect::<Vec<_>>().into_boxed_slice()
        )
    }


    fn play(&mut self, column_number: usize, player: Player) -> Result<(), InvalidMove> {
        try!(self.validate_move(column_number));
        let max_row = self.size().1;

        let mut row = 0;
        while row < max_row && self.get(column_number, row) == Player(0) {
            row += 1;
        }

        self.set(column_number, row - 1, player);
        Ok(())
    }

    // This method is inspired by Petter Strandmark’s Connect Four winning condition checking
    // code in https://github.com/PetterS/monte-carlo-tree-search/blob/master/games/connect_four.h,
    // licensed under the MIT License.
    #[inline(never)]
    fn has_just_won(&self) -> bool {
        let (last_column, last_row) = self.last_move();
        let player = self.get(last_column, last_row);
        let (last_column, last_row) = (last_column as isize, last_row as isize);
        let (max_column, max_row) = self.size();
        let (max_column, max_row) = (max_column as isize, max_row as isize);

        {
            let (mut left, mut right) = (0, 0);
            let mut column = last_column - 1;
            while column >= 0 && self.get(column as usize, last_row as usize) == player {
                left += 1;
                column -= 1;
            }
            column = last_column + 1;
            while column < max_column
                    && self.get(column as usize, last_row as usize) == player {
                right += 1;
                column += 1;
            }
            if left + right + 1 >= 4 {
                return true;
            }
        }

        {
            let (mut up, mut down) = (0, 0);
            let mut row = last_row - 1;
            while row >= 0 && self.get(last_column as usize, row as usize) == player {
                down += 1;
                row -= 1;
            }
            row = last_row + 1;
            while row < max_row && self.get(last_column as usize, row as usize) == player {
                up += 1;
                row += 1;
            }
            if up + down + 1 >= 4 {
                return true;
            }
        }

        {
            let (mut up, mut down) = (0, 0);
            let mut row = last_row - 1;
            let mut column = last_column - 1;
            while row >= 0
                    && column >= 0
                    && self.get(column as usize, row as usize) == player {
                down += 1;
                column -= 1;
                row -= 1;
            }
            row = last_row + 1;
            column = last_column + 1;
            while row < max_row
                    && column < max_column
                    && self.get(column as usize, row as usize) == player {
                up += 1;
                column += 1;
                row += 1;
            }
            if up + down + 1 >= 4 {
                return true;
            }
        }

        {
            let (mut up, mut down) = (0, 0);
            let mut row = last_row + 1;
            let mut column = last_column - 1;
            while row < max_row
                    && column >= 0
                    && self.get(column as usize, row as usize) == player {
                up += 1;
                column -= 1;
                row += 1;
            }
            row = last_row - 1;
            column = last_column + 1;
            while row >= 0
                    && column < max_column
                    && self.get(column as usize, row as usize) == player {
                down += 1;
                column += 1;
                row -= 1;
            }
            if up + down + 1 >= 4 {
                return true;
            }
        }

        return false;
    }

    fn validate_move(&self, column_number: usize) -> Result<(), InvalidMove> {
        let max_column = self.size().0;
        if column_number >= max_column {
            Err(InvalidMove::InvalidColumn(column_number))
        }
        else if self.get(column_number, 0) != Player(0) {
            Err(InvalidMove::ColumnFull(column_number))
        }
        else {
            Ok(())
        }
    }

    fn _fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (n_columns, n_rows) = self.size();
        let fill_row = |left, joiner, right| {
            format!("{}{}{}", left, vec!["─"; n_columns].join(joiner), right)
        };

        let rows: Vec<_> = (0..n_rows).map(|i| {
            let row = self.row(i).unwrap_or_else(|| unreachable!());
            let positions: Vec<_> = row.iter().map(|p| format!("{}", p)).collect();
            format!("│{}│\n", positions.join("│"))
        }).collect();

        try!(write!(
            f, "{top_row}{body}{bottom_row}",
            top_row = fill_row("┌", "┬", "┐\n"),
            body = rows.join(&fill_row("├", "┼", "┤\n")),
            bottom_row = fill_row("└", "┴", "┘\n")
        ));
        write!(f, " {}\n", (0..n_columns).map(|n| format!("{}", n)).collect::<Vec<_>>().join(" "))
    }
}


#[derive(Clone, Debug)]
pub struct VecState {
    state: Vec<Player>,
    columns: usize,
    rows: usize,
    last_move: (usize, usize),
}

impl State for VecState {
    fn new(columns: usize, rows: usize) -> Result<Self, ()> {
        Ok(
            VecState {
                state: vec![Player(0); rows * columns],
                columns: columns,
                rows: rows,
                last_move: (0, 0)
            }
        )
    }

    fn size(&self) -> (usize, usize) {
        (self.columns, self.rows)
    }

    fn set(&mut self, column: usize, row: usize, player: Player) {
        self.state[row * self.columns + column] = player;
        self.last_move = (column, row);
    }

    fn get(&self, column: usize, row: usize) -> Player {
        unsafe {
            *self.state.get_unchecked(row * self.columns + column)
        }
    }

    fn last_move(&self) -> (usize, usize) {
        self.last_move
    }
}

impl fmt::Display for VecState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self._fmt(f)
    }
}


#[derive(Debug)]
pub enum InvalidMove {
    InvalidColumn(usize),
    ColumnFull(usize),
}
