use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use clap::ArgMatches;
use rayon::prelude::*;

use crate::board::Board;
use crate::fen::FEN_DEFAULT_BOARD;

#[derive(Debug)]
struct PerftConfig {
    fen: &'static str,
    depth: usize,
    depth_level_results: Vec<usize>,
}

#[derive(Debug)]
struct PerftError {
    error_depth: usize,
    expected: usize,
    actual: usize,
}

impl Display for PerftError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Perft error at depth {}: expected {} but found {}",
            self.error_depth, self.expected, self.actual
        )
    }
}

impl Error for PerftError {}

lazy_static! {
    static ref PERFT_CONFIGS: HashMap<&'static str, PerftConfig> = init_perft_configs();
}

fn init_perft_configs() -> HashMap<&'static str, PerftConfig> {
    // https://www.chessprogramming.org/Perft_Results
    let mut map = HashMap::new();

    map.insert(
        "default",
        PerftConfig {
            fen: FEN_DEFAULT_BOARD,
            depth: 7,
            depth_level_results: vec![
                1,
                20,
                400,
                8902,
                197_281,
                4_865_609,
                119_060_324,
                3_195_901_860,
            ],
        },
    );
    map.insert(
        "short",
        PerftConfig {
            fen: FEN_DEFAULT_BOARD,
            depth: 4,
            depth_level_results: vec![1, 20, 400, 8902, 197_281],
        },
    );
    map.insert(
        "kiwipete",
        PerftConfig {
            fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
            depth: 5, // TODO 6
            depth_level_results: vec![1, 48, 2039, 97862, 4_085_603, 193_690_690, 8_031_647_685],
        },
    );
    map.insert(
        "pos3",
        PerftConfig {
            fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ",
            depth: 8,
            depth_level_results: vec![
                1, 14, 191, 2812, 43238, 674624, 11030083, 178633661, 3009794393,
            ],
        },
    );
    map.insert(
        "pos4",
        PerftConfig {
            fen: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            depth: 5, // TODO 6
            depth_level_results: vec![1, 6, 264, 9467, 422333, 15833292, 706045033],
        },
    );
    map.insert(
        "pos4mirror",
        PerftConfig {
            fen: "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
            depth: 5, // TODO 6
            depth_level_results: vec![1, 6, 264, 9467, 422333, 15833292, 706045033],
        },
    );
    map.insert(
        "pos5", // pos 5 seems kinda scuffed tbh
        PerftConfig {
            fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            depth: 2, // TODO 5
            depth_level_results: vec![1, 44, 1486, 62379, 2103487, 89941194],
        },
    );
    map.insert(
        "pos6",
        PerftConfig {
            fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            depth: 6, // TODO test 7, 8 and 9
            depth_level_results: vec![
                1,
                46,
                2079,
                89890,
                3894594,
                164075551,
                6_923_051_137,
                287_188_994_746,
                11_923_589_843_526,
                490_154_852_788_714,
            ],
        },
    );
    map.insert(
        "kiwidebug1",
        PerftConfig {
            fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q2/PPPB1P1P/R2BK2b w Qkq - 0 1",
            depth: 2,
            depth_level_results: vec![1, 38, 1767],
        },
    );
    map.insert(
        "kiwidebug2",
        PerftConfig {
            fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q2/PPPB1PpP/R2BK2R b KQkq - 0 1",
            depth: 3,
            depth_level_results: vec![1, 53, 1918, 97502],
        },
    );

    map
}

fn print_perft_configs() {
    println!("The following perft configs are available:");
    for (key, _) in PERFT_CONFIGS.iter() {
        println!("\t{}", key);
    }
}

fn perft_results(init_board: &Board, config: &PerftConfig) -> Vec<usize> {
    let mut results: Vec<usize> = vec![0; config.depth + 1];

    results[0] = 1;
    perft_recurse(init_board, config, 1, &mut results);

    results
}

fn perft_recurse(board: &Board, config: &PerftConfig, depth: usize, results: &mut Vec<usize>) {
    if depth > config.depth {
        return;
    }

    let successors = board.generate_successors();
    let mut count = 0usize;
    for s in successors {
        if s.is_in_check(!s.white_to_move()) {
            // illegal successor
            continue;
        }
        perft_recurse(&s, config, depth + 1, results);

        count += 1;
    }
    results[depth as usize] += count;
}

fn run_config(name: &str, config: &PerftConfig) -> Result<(), PerftError> {
    println!("Running perft config {}", name);
    let init_board = Board::from_fen(config.fen)
        .unwrap_or_else(|| panic!("Could not parse FEN string: {}", config.fen));

    let results = perft_results(&init_board, config);

    for (i, result) in results.iter().enumerate() {
        if *result != config.depth_level_results[i] {
            return Err(PerftError {
                error_depth: i,
                expected: config.depth_level_results[i],
                actual: *result,
            });
        }
    }

    Ok(())
}

fn print_pass(n: &str) {
    println!("PASS\tperft \"{}\"", n);
}

fn print_fail(n: &str, e: &PerftError) {
    eprintln!("FAILED\tperft \"{}\": {}", n, e);
}

fn test_all() {
    let results: Vec<(&str, Result<(), PerftError>)> = PERFT_CONFIGS
        .iter()
        .collect::<Vec<(&&str, &PerftConfig)>>()
        .par_iter()
        .map(|(key, config)| (**key, run_config(key, config)))
        .collect();

    let mut passed: Vec<&str> = Vec::new();
    let mut failed: Vec<(&str, PerftError)> = Vec::new();
    for (key, result) in results {
        match result {
            Ok(_) => passed.push(key),
            Err(e) => failed.push((key, e)),
        }
    }

    for n in passed {
        print_pass(n);
    }
    for (n, e) in &failed {
        print_fail(n, e);
    }
}

fn test_single(name: &str, config: &PerftConfig) {
    let result = run_config(name, config);
    match result {
        Ok(_) => print_pass(name),
        Err(e) => print_fail(name, &e),
    }
}

fn perft_debug(board: &Board, search_depth: usize) {
    let successors = board.generate_successors();
    let mut sum = 0;
    for s in successors {
        if s.is_in_check(!s.white_to_move()) {
            // illegal successor
            continue;
        }
        // Hack
        let config = PerftConfig {
            fen: "",
            depth: search_depth - 1,
            depth_level_results: vec![],
        };

        let results = perft_results(&s, &config);
        let count = results.last().unwrap();
        println!("{}", count);
        sum += count;
    }
    println!("Sum {}", sum);
}

pub fn perft_command(args: &ArgMatches) {
    if args.is_present("All") {
        println!();
        println!("Running all configs");
        let t1 = time::get_time();
        test_all();
        let t2 = time::get_time();
        let dur = t2 - t1;
        println!(
            "Took {}s {}ms",
            dur.num_seconds(),
            dur.num_milliseconds() % 1000
        );
        return;
    }

    if args.is_present("Print available perft tests") {
        print_perft_configs();
        return;
    }

    if args.is_present("Run") {
        let which = args.value_of("Run").unwrap();
        let config = PERFT_CONFIGS.get(which);
        if config.is_none() {
            print_perft_configs();
            return;
        }
        let t1 = time::get_time();
        test_single(which, config.unwrap());
        let t2 = time::get_time();
        let dur = t2 - t1;
        println!(
            "Took {}s {}ms",
            dur.num_seconds(),
            dur.num_milliseconds() % 1000
        );
        return;
    }

    if args.is_present("Debug") {
        let which = args.value_of("Debug").unwrap();
        let config = PERFT_CONFIGS.get(which);
        let fen = match config {
            None => which,
            Some(c) => c.fen,
        };

        let board = Board::from_fen(fen);
        if board.is_none() {
            println!("Bad FEN string: {}", which);
            return;
        }

        let board = board.unwrap();
        perft_debug(&board, 2); // TODO
        return;
    }

    print_perft_configs();
}
