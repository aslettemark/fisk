use crate::engine::Board;

fn traverse_depth_first(board: &Board, depth: i32) -> u32 {
    if depth == 0 {
        return 1;
    }
    let mut n = 1;
    for s in &board.generate_successors() {
        n += traverse_depth_first(s, depth - 1);
    }
    n
}

pub fn bench_movegen_default(depth: i32) {
    let t1 = time::get_time();
    let nodes = traverse_depth_first(&Board::new(), depth);
    let t2 = time::get_time();
    let time = t2 - t1;
    println!("{} nodes within depth {} from starting position (took {}s {}ms)", nodes, depth, time.num_seconds(), time.num_milliseconds() % 1000);
    println!("{} nodes/sec", 1000 * (nodes as f64 / time.num_milliseconds() as f64) as i64);
}
