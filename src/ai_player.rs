use std::cell::RefCell;
use std::iter::repeat;
use std::marker::PhantomData;

use rand::{Rng, RngCore, SeedableRng};
use rand_os::OsRng;
use rand_xoshiro::Xoshiro256StarStar;

#[cfg(not(feature = "noparallel"))]
use rayon::prelude::*;
#[cfg(not(feature = "noparallel"))]
use rayon::iter::{repeatn};

use super::*;

pub const SIMULATIONS: usize = 100_000;


#[derive(Copy, Clone)]
pub struct MonteCarloPlayer<G: Game> {
    _game: PhantomData<G>,
}

impl<G: Game> Default for MonteCarloPlayer<G> {
    fn default() -> MonteCarloPlayer<G> {
        MonteCarloPlayer {
            _game: PhantomData,
        }
    }
}

impl<G: Game> MonteCarloPlayer<G> {
    pub fn new() -> MonteCarloPlayer<G> {
        Self::default()
    }

    #[cfg(not(feature = "noparallel"))]
    fn simulate(&self, original_game: &G) -> Vec<(G::Move, i64)> {
        let me = original_game.current_player();

        original_game.valid_moves()
            .into_par_iter()
            .map_with(original_game.clone(), |ref mut initial_game, column| {
                initial_game.play(column).unwrap();
                let score = repeatn(initial_game.clone(), SIMULATIONS)
                    .map(|game| {
                        match simulate_game(game) {
                            Some(player) if player == me => 2,
                            Some(_) => -2,
                            _ => 1,
                        }
                    })
                    .sum();
                (column, score)
            })
            .collect()
    }

    #[cfg(feature = "noparallel")]
    fn simulate(&self, original_game: &G) -> Vec<(G::Move, i64)> {
        let me = original_game.current_player();

        original_game.valid_moves()
            .into_iter()
            .map(|column| {
                let mut initial_game = original_game.clone();
                initial_game.play(column).unwrap();
                let score = repeat(initial_game.clone()).take(SIMULATIONS)
                    .map(|game| {
                        match simulate_game(game) {
                            Some(player) if player == me => 2,
                            Some(_) => -2,
                            _ => 1,
                        }
                    })
                    .sum();
                (column, score)
            })
            .collect()
    }
}

impl<G: Game + 'static> PlayerTrait for MonteCarloPlayer<G> {
    type Game = G;

    fn make_move(&self, original_game: &G) -> G::Move {
        self.simulate(original_game).into_iter().max_by_key(|&(_, score)| score).unwrap().0
    }
}

pub fn simulate_game(game: impl Game) -> Option<Player> {
    thread_local!(static RNG: RefCell<Xoshiro256StarStar> = RefCell::new(new_rng()));

    RNG.with(|rng| {
        let mut rng = rng.borrow_mut();

        random_playout(&mut *rng, game)
    })
}

fn random_playout(rng: &mut impl Rng, mut game: impl Game) -> Option<Player> {
    let mut valid_moves = vec![];
    loop {
        game.valid_moves_fast(&mut valid_moves);
        if valid_moves.is_empty() {
            return None;
        }
        else {
            if let Some(winner) = game.play(*choose(rng, &valid_moves)).unwrap() {
                return Some(winner);
            }
        }
    }
}

fn choose<'a, R: Rng, T>(rng: &mut R, ts: &'a [T]) -> &'a T {
    &ts[rand_in_range(ts.len() as u32, rng) as usize]
}

/// Generate a random `r: usize` that satisfies `0 <= r < upper`.
///
/// The algorithm implemented in this function performs significantly better than the standard
/// modulo reduction algorithm, because it avoids the costly division operation.
/// It is not unbiased, but its bias is insignificant.
///
/// This algorithm was adapted from a C implementation given by Daniel Lemire in his blog:
/// https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
fn rand_in_range<R: Rng>(upper: u32, rng: &mut R) -> u32 {
    let random = rng.next_u32() as u64;
    ((upper as u64 * random) >> 32) as u32
}

fn new_rng() -> Xoshiro256StarStar {
    Xoshiro256StarStar::seed_from_u64(
        OsRng::new()
        .expect("could not gather entropy from the system")
        .next_u64()
    )
}


#[derive(Clone)]
pub struct TreeSearchPlayer<G: Game> {
    search_tree: PhantomData<RefCell<SearchTree<G>>>,
}

impl<G: Game> TreeSearchPlayer<G> {
    pub fn new(_game: &G) -> TreeSearchPlayer<G> {
        TreeSearchPlayer {
            search_tree: PhantomData::default(),
        }
    }
}

impl<G: Game> PlayerTrait for TreeSearchPlayer<G> {
    type Game = G;

    fn make_move(&self, game: &G) -> G::Move {
        let mut tree = SearchTree::new(game);
        let result = tree.select_move(game.current_player(), game);

        let wins = tree.wins;
        let visits = tree.visits;
        log::debug!("{} %, {}/{}", wins as f64 / visits as f64, wins, visits);
        for (move_, child) in tree.children.iter() {
            child
                .as_ref()
                .map(|child|
                    log::debug!(
                        "{}: {:.15} ({:9}/{:>9}), {:.15}",
                        move_,
                        child.wins as f64 / child.visits as f64,
                        child.wins,
                        child.visits,
                        ucb(tree.visits as f64, child.wins as f64, child.visits as f64),
                    )
                )
                .unwrap_or_else(|| log::debug!("{}: not visited", move_));
        }
        result
    }
}

#[derive(Clone, Debug)]
struct SearchTree<G: Game> {
    children: Vec<(G::Move, Option<SearchTree<G>>)>,
    visits: u64,
    wins: u64,
    draws: u64,
}

impl<G: Game> SearchTree<G> {
    fn new(game: &G) -> SearchTree<G> {
        SearchTree {
            children: game.valid_moves().into_iter().zip(repeat(None)).collect(),
            visits: 0,
            wins: 0,
            draws: 0,
        }
    }

    fn select_move(&mut self, me: Player, game: &G) -> G::Move {
        let mut rng = new_rng();
        for _ in 0..SIMULATIONS {
            self.step(me, game.clone(), &mut rng);
        }

        self.children.iter()
            .max_by_key(|(_, child)|
                child.as_ref().map(|child| child.visits).unwrap_or(0)
            )
            .expect("Could not find valid move")
            .0
    }

    fn step(&mut self, me: Player, mut game: G, rng: &mut impl Rng) -> Option<Player> {
        if game.has_ended() {
            self.visited(me, game.winner());
            return game.winner();
        }

        let expandable_moves: Vec<_> = self.children.iter()
            .enumerate()
            .filter(|(_, (_, child))| child.is_none())
            .map(|(index, (move_, _))| (index, *move_))
            .collect();
        if !expandable_moves.is_empty() {
            let &(index, random_move) = choose(rng, &expandable_moves);
            game.play(random_move).unwrap_or_else(|err| panic!("tried to play invalid move: {:?}", err));
            self.children[index].1.replace(SearchTree::new(&game));
            let result = random_playout(rng, game);
            self.children[index].1.as_mut().map(|child| child.visited(me, result));
            self.visited(me, result);
            return result;
        }

        let best_move = self.best_move(me, &game);
        let result = self.children.iter_mut()
            .filter(|(move_, _)| *move_ == best_move)
            .next()
            .map(|(move_, child)| {
                game.play(*move_).unwrap_or_else(|err| panic!("tried to play invalid move {:?}", err));
                child
                    .as_mut()
                    .expect("cannot be `None` because this code only runs on fully expanded trees")
                    .step(me, game, rng)
            })
            .expect("fully expanded trees must contain the move selected as best move");

        self.visited(me, result);
        result
    }

    fn best_move(&self, me: Player, game: &G) -> G::Move {
        self.children.iter()
            .filter(|(_, child)| child.is_some())
            .max_by_key(|(_, child)| {
                let child = child.as_ref().expect("child must be Some");
                let wins = if game.current_player() == me {
                    child.wins + child.draws / 2
                }
                else {
                    child.visits - child.wins - child.draws / 2
                };
                (ucb(self.visits as f64, wins as f64, child.visits as f64) * 1e15) as u64
            })
            .expect("tree does not have any children")
            .0
    }

    fn visited(&mut self, me: Player, winner: Option<Player>) {
        self.visits += 1;
        if winner.is_none() {
            self.draws += 1;
        }
        else if winner == Some(me) {
            self.wins += 1;
        }
    }
}

fn ucb(parent_visits: f64, wins: f64, child_visits: f64) -> f64 {
    wins / child_visits + (5. * parent_visits.ln() / child_visits).sqrt()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_correct_number_of_times_before_it_recurses() {
        let game = ConnectFour::<BitState>::new(7, 6).unwrap();
        let mut tree = SearchTree::new(&game);
        for _ in 0..7 {
            tree.step(game.current_player(), game.clone());
        }
        for (_, child) in tree.children.iter() {
            assert!(child.is_some());
        }
        assert_eq!(
            tree.children.iter().map(|(move_, _)| *move_).collect::<Vec<_>>(),
            (0..7).collect::<Vec<_>>()
        );
    }
}
