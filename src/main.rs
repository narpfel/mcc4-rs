use rand::{thread_rng, seq::SliceRandom};

use mcc4::*;

fn main() {
    env_logger::init();
    let game = ConnectFour::<BitState>::new(7, 6).unwrap();
    let human_player = HumanPlayer::new();
    let ai_player = TreeSearchPlayer::new(&game);
    let mut players: Vec<Box<PlayerTrait<Game=_>>> = vec![Box::new(human_player), Box::new(ai_player)];
    players.shuffle(&mut thread_rng());

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
