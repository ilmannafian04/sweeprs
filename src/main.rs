mod sweeper;

fn main() {
    let board = sweeper::Board::new(sweeper::EASY_CONFIG);
    println!("{:?}", board);
}
