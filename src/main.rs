use crate::engine::Board;
use crate::fen::board_from_fen;

mod engine;
mod move_generation;
mod constants;
mod fen;

fn main() {
    println!("Hello, world!");
    let a: Board = Board::new();
    a.print();

    println!("--------------");
    let succ = a.generate_successors();
    for s in succ.iter() {
        //s.print();
        //println!();
    }
    let b = board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR a a a b b b b");
    b.print();
}
