mod sweeper;

fn main() {
    let mut board = sweeper::Board::new(sweeper::EASY_CONFIG);
    println!("{:?}", board);
    let x = board.open(4, 4);
    println!("{:?}", x);
    println!("{:?}", board);
}
