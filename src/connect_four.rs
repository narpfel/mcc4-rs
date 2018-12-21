#[derive(Debug, Clone)]
pub struct ConnectFour {
    pub state: BitState,
    pub current_player: Player,
    pub winner: Option<Player>,
}

impl ConnectFour {
    pub fn new(columns: usize, rows: usize) -> Result<ConnectFour, ()> {
        Ok(ConnectFour {
            current_player: Player(1),
            state: BitState::new(columns, rows)?,
            winner: None,
        })
    }

    pub fn play(&mut self, column_number: Move) -> Result<Option<Player>, InvalidMove> {
        let player = self.current_player;
        self.state.play(column_number, player)?;
        self.next_player();
        if self.state.has_just_won() {
            self.winner = Some(player);
            Ok(self.winner)
        }
        else {
            Ok(None)
        }
    }

    fn next_player(&mut self) {
        self.current_player = self.other_player();
    }

    pub fn other_player(&self) -> Player {
        let Player(p) = self.current_player;
        Player(3 - p)
    }
}

pub type Move = usize;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Player(pub u8);


#[derive(Debug)]
pub enum Winner {
    Winner(Player),
    Draw,
    NotFinishedYet,
}

#[derive(Clone, Debug)]
pub struct BitState {
    state: [BitBoard; 2],
    empty_per_column: [u8; 12],
    columns: u8,
    rows: u8,
    last_player: Player,
    last_column: u8,
}

impl BitState {
    fn new(columns: usize, rows: usize) -> Result<Self, ()> {
        if columns * (rows + 1) > 64 || columns > 12 {
            Err(())
        }
        else {
            Ok(
                BitState {
                    state: [BitBoard(0), BitBoard(0)],
                    empty_per_column: [rows as u8; 12],
                    columns: columns as u8,
                    rows: rows as u8,
                    // last_move: (0, 0),
                    last_player: Player(0),
                    last_column: 255,
                }
            )
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.columns as usize, self.rows as usize)
    }

    fn play(&mut self, column: usize, player: Player) -> Result<(), InvalidMove> {
        self.validate_move(column)?;
        let row = self.empty_per_column[column] as usize - 1;
        self.set(column, row, player);
        self.empty_per_column[column] -= 1;
        self.last_player = player;
        self.last_column = column as u8;
        Ok(())
    }

    fn has_just_won(&self) -> bool {
        let Player(p) = self.last_player;
        self.state[p as usize - 1].has_winner(self.rows)
    }

    fn set(&mut self, column: usize, row: usize, Player(player): Player) {
        self.state[player as usize - 1].set_bit(column as u8, row as u8, self.rows);
    }

    fn get(&self, column: usize, row: usize) -> Player {
        Player(
            2 * self.state[1].get_bit(column as u8, row as u8, self.rows)
            + self.state[0].get_bit(column as u8, row as u8, self.rows)
        )
    }

    #[inline(always)]
    pub fn valid_moves_fast(&self, valid_moves: &mut Vec<usize>) {
        valid_moves.clear();
        self.empty_per_column[..self.columns as usize].iter()
            .enumerate()
            .filter(|&(_, &empty)| empty != 0)
            .for_each(|(i, _)| valid_moves.push(i));
    }

    pub fn valid_moves(&self) -> Vec<usize> {
        let columns = self.size().0;
        let mut moves = Vec::with_capacity(columns);
        self.valid_moves_fast(&mut moves);
        moves
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
}

#[derive(Copy, Clone, Debug)]
pub struct BitBoard(u64);

impl BitBoard {
    #[inline(always)]
    fn has_winner(&self, rows: u8) -> bool {
        let x = self.0;
        let vertical = x & x >> 1;
        let horizontal = x & x >> rows + 1;
        let diagonal_ud = x & x >> rows;
        let diagonal_du = x & x >> rows + 2;

        (vertical & vertical >> 2)
        | (horizontal & horizontal >> 2 * (rows + 1))
        | (diagonal_ud & diagonal_ud >> 2 * rows)
        | (diagonal_du & diagonal_du >> 2 * (rows + 2))
        != 0
    }


    #[inline(always)]
    fn get_bit(&self, column: u8, row: u8, rows: u8) -> u8 {
        ((self.0 >> (column * (rows + 1) + row)) & 1) as u8
    }

    #[inline(always)]
    fn set_bit(&mut self, column: u8, row: u8, rows: u8) {
        self.0 |= 1 << (column * (rows + 1) + row);
    }
}


#[derive(Debug)]
pub enum InvalidMove {
    InvalidColumn(usize),
    ColumnFull(usize),
}
