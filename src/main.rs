#[macro_use]
extern crate lazy_static;

extern crate clap;
extern crate time;

mod engine;
mod move_generation;
mod constants;
mod fen;
mod bench;

use clap::{App, Arg, SubCommand};

use crate::constants::*;
use crate::engine::Board;
use crate::fen::*;
use crate::bench::*;

fn main() {
    lazy_static::initialize(&KNIGHT_ATTACK);

    let opts = App::new("Fisk")
        .version("0.1.0")
        .author("Aksel Slettemark <akselslettemark@gmail.com>")
        .subcommand(SubCommand::with_name("bench")
            .about("Benchmark")
            .arg(Arg::with_name("Depth")
                .short("d")
                .long("depth")
                .takes_value(true))
            .arg(Arg::with_name("Start board")
                .short("s")
                .default_value("default")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("debug")
            .about("Debug")
        );
    let matches = opts.get_matches();

    match matches.subcommand_name() {
        Some("bench") => bench_movegen_default(matches.subcommand().1.unwrap().value_of("Depth").unwrap_or("5").parse::<i32>().unwrap()),
        Some("debug") => debug(),
        None => debug(),
        _ => { unreachable!() }
    }
}

fn debug() {
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

    let c = board_from_fen("rnbqkbnr/pppppppp/8/8/1R6/8/PP4PP/RNBQKBNR w KQkq - 0 1");
    c.print();

}
