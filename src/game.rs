use std::fmt;
use std::str::FromStr;

pub trait Game: Send + Sync + Clone {
    type State;
    type Move: FromStr + Send + fmt::Display + Copy;
    type InvalidMove: fmt::Debug;

    fn play(&mut self, move_: Self::Move) -> Result<Option<Player>, Self::InvalidMove>;
    fn winner(&self) -> Option<Player>;
    fn valid_moves(&self) -> Vec<Self::Move>;
    fn state(&self) -> &Self::State;
    fn current_player(&self) -> Player;
    fn next_player(&mut self);
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
