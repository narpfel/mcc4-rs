use rand::Rng;

use mcc4::*;

fn main() {
    let game = ConnectFour::new(7, 6).unwrap();
    let human_player = MonteCarloPlayer::new();
    let ai_player = MonteCarloPlayer::new();
    let mut players: Vec<MonteCarloPlayer> = vec![human_player, ai_player];
    rand::thread_rng().shuffle(&mut players);

    println!("\x1B[2J\x1B[H");
    println!("{}", game.state());
    for (state, player, move_, winner) in game.iter(players) {
        print!("\x1B[2J\x1B[H");
        println!("Player {} has moved {}", player, move_);
        println!("{}", state);
        match winner {
            Winner::Winner(winner) => println!("Player {} has won.", winner),
            Winner::Draw => println!("Draw."),
            Winner::NotFinishedYet => {}
        };
    }
}
