use std::fmt;
use std::str::FromStr;

use super::PlayerTrait;

pub trait Game: Send + Sync + Clone {
    type State: Clone;
    type Move: FromStr + Send + fmt::Display + fmt::Debug + Copy + PartialEq;
    type InvalidMove: fmt::Debug;

    fn play(&mut self, move_: Self::Move) -> Result<Option<Player>, Self::InvalidMove>;
    fn winner(&self) -> Option<Player>;
    fn valid_moves(&self) -> Vec<Self::Move>;
    fn valid_moves_fast(&self, valid_moves: &mut Vec<Self::Move>);
    fn state(&self) -> &Self::State;
    fn current_player(&self) -> Player;
    fn next_player(&mut self);

    /// Intended to be overwritten by implementors for better performance (e. g. storing a `bool`)
    fn has_ended(&self) -> bool {
        self.valid_moves().is_empty() || self.winner().is_some()
    }

    fn iter(self, players: Vec<Box<PlayerTrait<Game=Self>>>) -> Moves<Self> {
        Moves::new(self, players)
    }
}


pub struct Moves<G: Game> {
    game: G,
    players: Vec<Box<PlayerTrait<Game=G>>>,
    current_player_index: usize,
}

impl<G: Game> Moves<G> {
    fn new(game: G, players: Vec<Box<PlayerTrait<Game=G>>>) -> Moves<G> {
        Moves {
            game: game,
            players: players,
            current_player_index: 0,
        }
    }
}

impl<G: Game> Iterator for Moves<G> {
    type Item = (G::State, Player, G::Move, Winner);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.game.has_ended() || self.game.valid_moves().is_empty() {
            return None;
        }

        let num_players = self.players.len();
        let player = self.game.current_player();

        loop {
            let move_ = self.players[self.current_player_index].make_move(&self.game);
            match self.game.play(move_) {
                Ok(maybe_winner) => {
                    self.current_player_index = (self.current_player_index + 1) % num_players;
                    let winner = match maybe_winner {
                        Some(winner) => Winner::Winner(winner),
                        None => if self.game.has_ended() { Winner::Draw } else { Winner::NotFinishedYet }
                    };
                    return Some((self.game.state().clone(), player, move_, winner));
                },
                Err(invalid_move) => {
                    self.players[self.current_player_index].invalid_move(invalid_move);
                }
            }
        }
    }
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


#[derive(Debug)]
pub enum Winner {
    Winner(Player),
    Draw,
    NotFinishedYet,
}
