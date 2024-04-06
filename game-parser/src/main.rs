fn main() {
    let file_path = "/home/scipex/Downloads/lichess_db_standard_rated_2024-02.pgn";

    let mut file = std::fs::File::open(file_path).unwrap();
    let mut buffer = vec![0; 1 << 16];
    let mut leftover: Box<[u8]> = Box::new([]);
}
