use chess_lib::{
    board::{Board, DEFAULT_FEN},
    movegen::legal_moves,
};

fn perft(b: Board, depth: i32) -> u64 {
    let moves = legal_moves(&b);
    let mut nodes = 0;

    if depth == 1 {
        return moves.len() as u64;
    }

    for m in moves.iter() {
        let mut b = b.clone();
        b.make_move(m);
        nodes += perft(b, depth - 1);
    }

    nodes
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Expected depth");
    }

    let depth: i32 = args[1].parse().unwrap();
    let board = Board::from_fen(DEFAULT_FEN);

    let start = std::time::Instant::now();
    let count = perft(board, depth);
    let elapsed = start.elapsed().as_secs_f64();

    let nps = count as f64 / elapsed;
    println!("{:.2} nodes per second; total {:.2}s", nps, elapsed);
}
