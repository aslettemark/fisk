use crate::engine::Board;
use crate::fen::*;

#[macro_use]
extern crate lazy_static;

mod engine;
mod move_generation;
mod constants;
mod fen;

fn main() {
    println!("Hello, world!");
    let a: Board = Board::new();
    a.print();

    println!("------------------------------------");
    /*let succ = a.generate_successors();
    for s in succ.iter() {
        s.print();
        println!();
    }*/
    let b = board_from_fen(FEN_DEFAULT_BOARD);
    b.print();
    println!("------------------------------------");
    let succ = b.generate_successors();
    for s in succ.iter() {
        s.print();
        println!();
    }
}
