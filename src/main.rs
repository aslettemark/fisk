use std::io;
use std::io::Write;
use std::mem::size_of;

use clap::{App, Arg, SubCommand};

use bench::*;
use fisk::board::*;
use fisk::constants::*;
use fisk::fen::*;
use fisk::perft::perft_command;
use fisk::uci::UciState;

mod bench;

fn main() {
    lazy_static::initialize(&KNIGHT_ATTACK_LIST);
    lazy_static::initialize(&KNIGHT_ATTACK_MASKS);
    lazy_static::initialize(&FILE_ATTACK);
    lazy_static::initialize(&RANK_ATTACK);
    lazy_static::initialize(&KING_ATTACK);
    lazy_static::initialize(&KING_ATTACK_MASK);

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
                .arg(Arg::with_name("Start board").short("s").takes_value(true))
                .arg(
                    Arg::with_name("Use iterator")
                        .long("iterator")
                        .takes_value(false)
                        .help("Run benchmark using iterator internally"),
                ),
        )
        .subcommand(SubCommand::with_name("debug").about("Debug"))
        .subcommand(
            SubCommand::with_name("perft")
                .arg(Arg::with_name("Run").long("run").takes_value(true))
                .arg(Arg::with_name("All").long("all"))
                .arg(Arg::with_name("Print available perft tests").long("print"))
                .arg(Arg::with_name("Debug").long("debug").takes_value(true)),
        )
        .subcommand(SubCommand::with_name("interactive").about("Interactive"))
        .subcommand(SubCommand::with_name("uci").about("UCI"));
    let matches = opts.get_matches();

    match matches.subcommand_name() {
        Some("bench") => bench_movegen(
            matches
                .subcommand()
                .1
                .unwrap()
                .value_of("Depth")
                .unwrap_or("5")
                .parse::<i32>()
                .unwrap(),
            matches.subcommand().1.unwrap().is_present("Use iterator"),
            matches.subcommand().1.unwrap().value_of("Start board"),
        ),
        Some("debug") => debug(),
        Some("perft") => perft_command(matches.subcommand().1.unwrap()),
        Some("interactive") => interactive(),
        Some("uci") => {
            let mut uci_state = UciState::new();
            uci_state
                .run_uci_input(&mut io::stdin(), &mut io::stdout())
                .unwrap();
        }
        None => debug(),
        _ => unreachable!(),
    }
}

fn generate_and_print(fen_string: String) -> Result<(), ()> {
    let board = Board::from_fen(&*fen_string).ok_or(())?;

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
    let a: Board = Board::default();
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

    println!("Type sizes in bytes");
    println!(
        "Board: {}\nBitBoard: {}\nColor: {}",
        size_of::<Board>(),
        size_of::<BitBoard>(),
        size_of::<Color>(),
    );
    println!("PieceKind {}", size_of::<PieceKind>());

    let mut b = Board::default();
    println!("Board eval: {}", b.eval());
    println!("Play game:");
    b.print();
    for _ in 0..1000 {
        let (eval, mov) = b.best_move(6);
        match mov {
            Some(m) => {
                println!("Eval {}", eval);
                b = b.make_move(&m);
                b.print();
            }
            None => {
                println!("Finished");
                if eval == 0 {
                    println!("Stalemate");
                } else if eval > 0 {
                    println!("White won");
                } else {
                    println!("Black won");
                }
                break;
            }
        }
    }
}
