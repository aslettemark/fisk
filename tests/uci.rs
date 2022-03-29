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

#[test]
fn engine_interprets_promotion() {
    let mut state = UciState::new();
    let mut out_buf: Vec<u8> = Vec::new();
    state.run_uci_input(&mut "position startpos moves e2e4 d7d5 e4d5 d8d5 b1c3 d5e6 f1e2 b8c6 g1f3 g8f6 e1g1 e6d6 a2a3 a7a6 d2d4 c8g4 d4d5 c6b8 f3d2 d6d7 f2f3 g4h5 d2c4 b7b5 c4e5 d7d6 c1f4 b8d7 d1d4 d6b6 f1d1 b6d4 d1d4 g7g5 e5d7 f6d7 f4g5 h8g8 g5f4 e8c8 g2g4 h5g6 d4d1 g6c2 d1c1 c2f5 c3b5 e7e5 d5e6 f8c5 g1f1 f7e6 b5c7 e6e5 c7a6 f5g4 f3g4 g8f8 a6c5 f8f4 f1g1 d7c5 c1c5 c8b8 c5b5 b8a8 h2h3 d8d2 a1e1 a8a7 b2b4 e5e4 e1f1 f4f1 g1f1 d2a2 b5a5 a7b6 f1f2 b6c6 f2e3 a2c2 h3h4 c6d6 g4g5 c2c7 e2g4 c7c4 h4h5 c4c7 h5h6 c7c4 a5a7 d6d5 g5g6 c4c3 e3f2 c3c2 f2g3 h7g6 a7a5 d5d4 h6h7 c2c3 g3h4 c3c1 h7h8q\n".as_bytes(), &mut out_buf).unwrap();
}

#[test]
fn engine_promotes_uci() {
    uci_test(&mut UciState::new(),
    &"position startpos moves e2e3 b8c6 b1c3 e7e5 g1f3 g8f6 f1b5 d7d6 d2d3 c8d7 b5c4 f8e7 e1g1 e8g8 f3g5 h7h6 c4f7 f8f7 g5f7 g8f7 f2f4 e5f4 f1f4 f7g8 e3e4 c6e5 d3d4 e5g6 f4f2 f6g4 f2f3 g6h4 f3g3 h6h5 d1d3 d8f8 c1e3 g4e3 d3e3 f8f6 c3d5 f6f7 e3b3 d7e6 b3b7 a8f8 d5e7 f7e7 b7a7 e7f6 d4d5 e6g4 g3b3 g4e2 b3b8 f6f1 a1f1 e2f1 b8f8 g8f8 g1f1 h4g6 a7c7 g6e5 c7d6 f8f7 d6e5 g7g6 d5d6 g6g5 d6d7 g5g4\ngo\n",
    "bestmove d7d8q\n");
}
