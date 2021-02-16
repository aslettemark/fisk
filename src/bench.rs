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
    let t1 = time::get_time();
    if let Some(s) = starting_board {
        println!("{}", s);
    }
    let board = starting_board
        .map(|fen| Board::from_fen(fen).expect("Invalid starting board"))
        .unwrap_or_else(Board::default);
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
