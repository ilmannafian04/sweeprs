mod sweeper;

fn main() {
    let board = sweeper::Board::new(r#mod::EASY_CONFIG);
    println!("{:?}", board);
}
