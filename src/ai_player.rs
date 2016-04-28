use rand::{StdRng, Rng};

use super::*;

const SIMULATIONS: usize = 100_000;


pub struct AiPlayer {
    rng: StdRng,
}

impl AiPlayer {
    pub fn new() -> AiPlayer {
        AiPlayer {
            rng: StdRng::new().expect("Could not create random number generator"),
        }
    }

    fn simulate_game(&mut self, game: &mut Game) -> Option<Player> {
        loop {
            let valid_moves = self.valid_moves(game);
            if valid_moves.is_empty() {
                return game.winner();
            }
            game.play(*self.rng.choose(&valid_moves).unwrap()).unwrap();
            let winner = game.winner();
            if winner.is_some() {
                return winner;
            }
        }
    }

    fn valid_moves(&self, game: &Game) -> Vec<usize> {
        let columns = game.size().0;
        (0..columns).filter(|&column| game.state().column(column).unwrap()[0] == Player(0)).collect()
    }
}

impl PlayerTrait for AiPlayer {
    fn make_move(&mut self, original_game: &Game) -> usize {
        let valid_moves = self.valid_moves(original_game);
        let initial_games: Vec<_> = valid_moves.iter().map(|column| {
            let mut game = original_game.clone();
            game.play(*column).unwrap();
            game
        }).collect();

        let simulated: Vec<_> = initial_games.iter().map(|game| {
            (0..SIMULATIONS).map(|_| {
                let mut game = game.clone();
                match self.simulate_game(&mut game) {
                    Some(player) => if player == original_game.current_player() { 2 } else { -2 },
                    _ => 1
                }
            }).fold(0, |x, y| x + y)
        }).collect();
        let max = simulated.iter().max().unwrap();
        valid_moves[simulated.iter().position(|x| x == max).unwrap()]
    }
}

impl Clone for AiPlayer {
    fn clone(&self) -> AiPlayer {
        AiPlayer::new()
    }
}
