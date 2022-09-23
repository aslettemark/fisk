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
    println!(
        "Benchmarking with depth={}, use_iterator={}",
        depth, use_iterator
    );
    if let Some(s) = starting_board {
        println!("{}", s);
    }
    let board = starting_board
        .map(|fen| Board::from_fen(fen).expect("Invalid starting board"))
        .unwrap_or_else(Board::default);
    let t1 = time::get_time();
    let nodes = if use_iterator {
        count_nodes_iterator(&board, depth)
    } else {
        count_nodes(&board, depth)
    };
    let t2 = time::get_time();
    let time = t2 - t1;
    println!(
        "{} nodes within depth {} from starting position (took {}s {}ms)",
        nodes,
        depth,
        time.num_seconds(),
        time.num_milliseconds() % 1000
    );
    println!(
        "{} nodes/sec",
        1000 * (nodes as f64 / (time.num_milliseconds() + 1) as f64) as i64
    );
}

pub fn bench_search(depth: i32, starting_board: Option<&str>) {
    println!("Benchmarking search with depth={}", depth);
    let board = starting_board
        .map(|fen| Board::from_fen(fen).expect("Invalid starting board"))
        .unwrap_or_else(Board::default);

    let t1 = time::get_time();
    board.best_move(depth as usize);
    let t2 = time::get_time();

    let time = t2 - t1;
    println!(
        "Depth {} searched in {}s {}ms",
        depth,
        time.num_seconds(),
        time.num_milliseconds() % 1000
    );
}
