mod sweeper;

fn main() {
    let mut board = sweeper::Board::new(sweeper::EASY_CONFIG).unwrap();
    println!("{:?}", board);
    let x = board.open(4, 4);
    println!("{:?}", x.unwrap());
    println!("{:?}", board);
}
