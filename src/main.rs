use std::time::{Instant, Duration};

mod ai_player;
mod connect_four;

use ai_player::{SIMULATIONS, MonteCarloPlayer};
use connect_four::{ConnectFour, Player};

pub const NANOS_PER_SECOND: u64 = 1_000_000_000;


fn as_fractional_secs(duration: Duration) -> f64 {
    (duration.as_secs() * NANOS_PER_SECOND + duration.subsec_nanos() as u64) as f64
        / NANOS_PER_SECOND as f64
}


fn main() {
    let columns = 7;
    let game = ConnectFour::new(columns, 6).unwrap();
    let benchmark_player = MonteCarloPlayer;
    let now = Instant::now();
    benchmark_player.make_move(&game);
    let seconds = as_fractional_secs(now.elapsed());
    println!("{} seconds elapsed", seconds);
    println!("{:?} games per second", (columns * SIMULATIONS) as f64 / seconds);
}
