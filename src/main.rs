extern crate rand;

extern crate mcc4;

use rand::Rng;

use mcc4::{Game, State, AiPlayer, HumanPlayer, PlayerTrait};

fn main() {
    let mut game = Game::new(7, 6);
    let human_player = HumanPlayer;
    let ai_player = AiPlayer;
    let mut players: Vec<&PlayerTrait> = vec![&human_player, &ai_player];
    rand::thread_rng().shuffle(&mut players);

    println!("\x1B[2J\x1B[H");
    println!("{}", game.state());
    'outer: loop {
        for player in &players {
            let move_ = player.make_move(&mut game);
            match game.play(move_) {
                Ok(state) => {
                    print!("\x1B[2J\x1B[H");
                    println!("Player {} has moved {}", game.other_player(), move_ + 1);
                    println!("{}", state);
                },
                Err(err) => println!("Invalid input: {:?}", err),
            }

            if let Some(winner) = game.winner() {
                println!("Player {} has won.", winner);
                break 'outer;
            }
        }
    }
}
