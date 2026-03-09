use fisk::board::Board;

fn count_nodes(board: &Board, depth: i32) -> u32 {
    if depth == 0 {
        return 1;
    }
    let mut n = 1;
    for s in &board.generate_successors() {
        n += count_nodes(s, depth - 1);
    }
    n
}

fn count_nodes_iterator(board: &Board, depth: i32) -> u32 {
    if depth == 0 {
        return 1;
    }
    let mut n = 1;
    for s in board.iter_successors() {
        n += count_nodes_iterator(&s, depth - 1);
    }
    n
}

pub fn bench_movegen(depth: i32, use_iterator: bool, starting_board: Option<&str>) {
    let count_fn = if use_iterator {
        count_nodes_iterator
    } else {
        count_nodes
    };

    println!(
        "Benchmarking movegen with depth={}, use_iterator={}",
        depth, use_iterator
    );

    if let Some(fen) = starting_board {
        let board = Board::from_fen(fen).expect("Invalid starting board");
        bench_movegen_single("custom", &board, depth, count_fn);
    } else {
        let mut total_nodes = 0u64;
        let t1 = time::get_time();
        for (name, fen) in BENCH_POSITIONS {
            let board = Board::from_fen(fen).expect("Invalid bench FEN");
            total_nodes += bench_movegen_single(name, &board, depth, count_fn);
        }
        let t2 = time::get_time();
        let total_ms = (t2 - t1).num_milliseconds();
        println!("  {:<12} {:>12} nodes  {:>6}ms", "TOTAL", total_nodes, total_ms);
    }
}

fn bench_movegen_single(
    name: &str,
    board: &Board,
    depth: i32,
    count_fn: fn(&Board, i32) -> u32,
) -> u64 {
    let t1 = time::get_time();
    let nodes = count_fn(board, depth);
    let t2 = time::get_time();
    let ms = (t2 - t1).num_milliseconds();
    println!("  {:<12} {:>12} nodes  {:>6}ms", name, nodes, ms);
    nodes as u64
}

const BENCH_POSITIONS: &[(&str, &str)] = &[
    ("startpos", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
    ("kiwipete", "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -"),
    ("pos3", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -"),
    ("pos5", "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"),
    ("pos6", "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10"),
];

pub fn bench_search(depth: i32, starting_board: Option<&str>) {
    println!("Benchmarking search with depth={}", depth);

    if let Some(fen) = starting_board {
        let board = Board::from_fen(fen).expect("Invalid starting board");
        let t1 = time::get_time();
        let (_eval, _mov, nodes) = board.best_move_counted(depth as usize);
        let t2 = time::get_time();
        let ms = (t2 - t1).num_milliseconds();
        println!("  {:<12} {:>12} nodes  {:>6}ms", "custom", nodes, ms);
    } else {
        let mut total_nodes = 0u64;
        let t1 = time::get_time();
        for (name, fen) in BENCH_POSITIONS {
            let board = Board::from_fen(fen).expect("Invalid bench FEN");
            let t_start = time::get_time();
            let (_eval, _mov, nodes) = board.best_move_counted(depth as usize);
            let t_end = time::get_time();
            let ms = (t_end - t_start).num_milliseconds();
            total_nodes += nodes;
            println!("  {:<12} {:>12} nodes  {:>6}ms", name, nodes, ms);
        }
        let t2 = time::get_time();
        let total_ms = (t2 - t1).num_milliseconds();
        println!("  {:<12} {:>12} nodes  {:>6}ms", "TOTAL", total_nodes, total_ms);
    }
}
