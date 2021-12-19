use fisk::board::Board;

#[test]
fn default_board_has_symmetric_eval() {
    let e = Board::default().eval();
    assert_eq!(e, 0);
}
