mod engine;

use self::engine::Board;

fn main() {
    println!("Hello, world!");
    let a: Board = Board::new();
    a.print();
}
