extern crate clap;
extern crate lazy_static;
extern crate time;

use std::io;
use std::io::Write;

use clap::{App, Arg, SubCommand};

use fisk::constants::*;
use fisk::engine::Board;
use fisk::fen::*;

use crate::bench::*;

mod bench;

fn main() {
    lazy_static::initialize(&KNIGHT_ATTACK);
    lazy_static::initialize(&FILE_ATTACK);
    lazy_static::initialize(&RANK_ATTACK);
    lazy_static::initialize(&KING_ATTACK);

    let opts = App::new("Fisk")
        .version("0.1.0")
        .author("Aksel Slettemark <akselslettemark@gmail.com>")
        .subcommand(
            SubCommand::with_name("bench")
                .about("Benchmark")
                .arg(
                    Arg::with_name("Depth")
                        .short("d")
                        .long("depth")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("Start board")
                        .short("s")
                        .default_value("default")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("debug").about("Debug"))
        .subcommand(SubCommand::with_name("interactive").about("Interactive"));
    let matches = opts.get_matches();

    match matches.subcommand_name() {
        Some("bench") => bench_movegen_default(
            matches
                .subcommand()
                .1
                .unwrap()
                .value_of("Depth")
                .unwrap_or("5")
                .parse::<i32>()
                .unwrap(),
        ),
        Some("debug") => debug(),
        Some("interactive") => interactive(),
        None => debug(),
        _ => unreachable!(),
    }
}

fn generate_and_print(fen_string: String) -> Result<(), ()> {
    let board = Board::from_fen(&*fen_string)?;

    let succ = board.generate_successors();
    for s in &succ {
        s.print();
        println!();
    }

    println!("{} succesors", succ.len());

    Ok(())
}

fn interactive() {
    loop {
        println!("> Generate successors from FEN string");
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match generate_and_print(input) {
                Ok(_) => (),
                Err(_) => println!("Could not parse FEN string"),
            },
            Err(e) => println!("error: {}", e),
        }
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
    let b = Board::from_fen(FEN_DEFAULT_BOARD).unwrap();
    b.print();
    println!("------------------------------------");
    let succ = b.generate_successors();
    for s in succ.iter() {
        s.print();
        println!();
    }

    let c = Board::from_fen("rnbqkbnr/pppppppp/8/8/1R6/8/PP4PP/RNBQKBNR w KQkq - 0 1").unwrap();
    c.print();
}
