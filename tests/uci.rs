use fisk::uci::UciState;

fn uci_test(state: &mut UciState, input: &str, expected_output: &str) {
    let mut out_buf: Vec<u8> = Vec::new();
    state
        .run_uci_input(&mut input.as_bytes(), &mut out_buf)
        .unwrap();

    assert_eq!(&out_buf, expected_output.as_bytes());
}

#[test]
fn engine_replies_to_uci() {
    uci_test(
        &mut UciState::new(),
        &"uci\n",
        "id name fisk\nid author Aksel Slettemark\nuciok\n",
    );
}

#[test]
fn engine_finds_obvious_move() {
    uci_test(
        &mut UciState::new(),
        &"position fen 6k1/8/6K1/8/8/8/8/4R3 w - - 0 1\ngo\n",
        "bestmove e1e8\n",
    );
}

#[test]
fn engine_interprets_castling() {
    let mut state = UciState::new();
    let mut out_buf: Vec<u8> = Vec::new();
    state.run_uci_input(&mut "position startpos moves e2e3 d7d5 b1c3 g8f6 g1f3 e7e6 f1b5 b8d7 f3e5 f8e7 d2d4 e8g8 e1g1 f6e4 d1f3 a7a6 f3f7 f8f7\n".as_bytes(), &mut out_buf).unwrap();
}
