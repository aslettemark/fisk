use fisk::uci::UciState;

#[test]
fn engine_replies_to_uci() {
    let mut output: Vec<u8> = Vec::new();
    let mut uci_state = UciState::new();
    uci_state
        .run_uci_input(&mut "uci".as_bytes(), &mut output)
        .unwrap();

    assert_eq!(
        &output,
        b"id name fisk\nid author Aksel Slettemark\nuciok\n"
    );
}
