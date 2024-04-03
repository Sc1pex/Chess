use chess_lib::{
    bot::{v1::Bot, BotTrait},
    game::Game,
};

fn main() {
    // let game = Game::from_fen("8/k7/3p4/p2P1p2/P2P1P2/8/8/K7 w - - 0 1");
    // let mut game = Game::default();
    let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
    println!("{}", game.board);

    loop {
        let start = std::time::Instant::now();
        let mut bot = Bot::new(30, 1_000_000, 3000);
        let m = bot.make_move(game.board.clone());
        let elapsed = start.elapsed();
        println!(
            "Bot move: {} in {}ms at depth of {}",
            m,
            elapsed.as_millis(),
            bot.reached_depth
        );
        game.make_move(m);
        println!("{}", game.board);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}
