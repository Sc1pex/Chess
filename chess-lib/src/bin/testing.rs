use chess_lib::{bot::Bot, game::Game, wasm::GameState};
use std::time::Duration;

fn main() {
    // let game = Game::from_fen("8/8/8/8/8/1k2N3/2q5/K7 w - - 0 1");
    let mut game = Game::from_fen("1R3K2/2R5/8/8/8/8/k7/8 w - - 0 1");
    println!("{}", game.board);

    // let mut bot = Bot::new(7, 10_000_000, 1000);
    // let m = bot.make_move(game.board.clone(), false);
    // for m in m.iter() {
    //     println!("{}: {}", m.0, m.1)
    // }
    // println!("------");
    loop {
        let mut bot = Bot::new(10, 10_000_000, 1000);
        let m = bot.make_move(game.board.clone(), true);
        for m in m.iter() {
            println!("{}: {}", m.0, m.1)
        }
        game.make_move(m[0].0);
        println!("{}", game.board);
        if game.game_state != GameState::InProgress {
            break;
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}
