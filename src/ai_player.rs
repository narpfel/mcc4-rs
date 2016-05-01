use std::thread;
use std::sync::mpsc;

use rand::{StdRng, Rng, SeedableRng};

use super::*;

pub const SIMULATIONS: usize = 100_000;


#[derive(Clone, Copy)]
pub struct AiPlayer<'a> {
    seed: Option<&'a [usize]>,
}


impl<'a> AiPlayer<'a> {
    pub fn new() -> AiPlayer<'a> {
        AiPlayer { seed: None }
    }

    pub fn with_seed(seed: &'a [usize]) -> AiPlayer<'a> {
        AiPlayer { seed: Some(seed) }
    }

    fn new_rng(&self) -> StdRng {
        match self.seed {
            Some(seed) => StdRng::from_seed(seed),
            None => StdRng::new().expect(
                "Could not create random number generator, not enough entropy"
            ),
        }
    }
}


impl<'a> PlayerTrait for AiPlayer<'a> {
    fn make_move(&self, original_game: &Game) -> usize {
        let me = original_game.current_player();
        let valid_moves = find_valid_moves(original_game);

        let (tx, rx) = mpsc::channel();
        for column in &valid_moves {
            let (mut initial_game, tx) = (original_game.clone(), tx.clone());
            let column = *column;
            let mut rng = self.new_rng();

            initial_game.play(column).unwrap();
            thread::spawn(move || {
                let mut score = 0;
                for _ in 0..SIMULATIONS {
                    let mut game = initial_game.clone();
                    score += match simulate_game(&mut rng, &mut game) {
                        Some(player) => if player == me { 2 } else { -2 },
                        _ => 1
                    };
                }
                tx.send((column, score)).unwrap();
            });
        }

        let mut scores = Vec::with_capacity(valid_moves.len());
        for _ in &valid_moves {
            let (column, score) = rx.recv().unwrap();
            scores.push((column, score));
        }

        scores.iter().max_by_key(|&&(_, score)| score).unwrap().0
    }
}


pub fn simulate_game<R: Rng>(rng: &mut R, game: &mut Game) -> Option<Player> {
    loop {
        let valid_moves = find_valid_moves(game);
        if valid_moves.is_empty() {
            return game.winner();
        }
        game.play(*rng.choose(&valid_moves).unwrap()).unwrap();
        let winner = game.winner();
        if winner.is_some() {
            return winner;
        }
    }
}


pub fn find_valid_moves(game: &Game) -> Vec<usize> {
    let columns = game.size().0;
    (0..columns).filter(|&column| game.state().get(column, 0) == Player(0)).collect()
}
