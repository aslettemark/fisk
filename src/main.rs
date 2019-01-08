use self::engine::Board;

mod engine;
mod move_generation;
mod constants;

fn main() {
    println!("Hello, world!");
    let a: Board = Board::new();
    a.print();

    println!("--------------");
    let succ = a.generate_successors();
    for s in succ.iter() {
        s.print();
        println!();
    }
}
