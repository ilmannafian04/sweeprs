mod sweeper;

fn main() {
    let board = sweeper::Board::new(4, 4);
    println!("{:?}", board);
}
