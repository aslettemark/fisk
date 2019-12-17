#[macro_use]
extern crate lazy_static;

mod engine;
mod move_generation;
mod constants;
mod fen;

use crate::constants::*;
use crate::engine::Board;
use crate::fen::*;

fn main() {
    println!("Hello, world!");
    lazy_static::initialize(&KNIGHT_ATTACK);
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
