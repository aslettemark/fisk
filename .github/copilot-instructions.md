# Copilot Instructions for Fisk

Fisk is a Rust chess engine using bitboard representation, with a UCI protocol interface. It runs on Lichess as [Fisk_Bot](https://lichess.org/@/Fisk_Bot/).

## Build, Test, and Run

```sh
cargo build                          # Debug build
cargo build --release                # Optimized build (LTO + codegen-units=1)
cargo test                           # Run all tests
cargo test board::memsizes           # Run a single test by name
cargo test -- --nocapture            # Show println! output in tests

# Performance benchmarks (use native CPU features)
RUSTFLAGS="-C target-cpu=native" cargo run --release -- bench -d 7

# Perft verification
RUSTFLAGS="-C target-cpu=native" cargo run --release -- perft --all
```

Uses the **2024 edition** on **stable** Rust. CI runs `cargo build` and `cargo test`.

> **Keeping this file up to date:** When making changes that affect build requirements, architecture, or conventions described here, update this file as part of the same change.

## Architecture

The engine follows a standard chess engine pipeline: UCI protocol -> search -> evaluation, built on a bitboard-based board representation.

**Board representation** (`board.rs`): The `Board` struct is 216 bytes, verified by tests. It contains:
- `BitBoard`: 10 separate `u64` bitboards (one per piece type per color) using LERF encoding (bit 0 = a1, bit 63 = h8)
- Piece list: parallel `[u8; 32]` (tzcnt positions) and `[PieceKind; 32]` arrays for O(32) iteration
- `piecelist_index_by_square`: `[u8; 64]` reverse lookup from square to piece-list index (0xFF if empty), for O(1) piece lookup during make_move
- `Flags`: all game state (side to move, castling, en passant, halfmove/fullmove clocks) packed into a single `u64`

**Move representation** (`move_representation.rs`): Moves are encoded as 16-bit values. Bits 0-5: from square, 6-11: to square, 12-13: special flags, 14: capture, 15: promotion. This follows the standard chess programming encoding from chessprogramming.org.

**Move generation** (`movegen_movelist.rs`): Generates pseudo-legal moves only -- legality is checked later in search by filtering moves that leave the king in check. Functions append to a `Vec<Move>` passed by mutable reference.

**Search** (`search.rs`): Minimax with fail-hard alpha-beta pruning. Move ordering scores promotions (10) > captures (5) > quiet moves (0). Checkmate is scored as `NEGINF + depth_penalty` to prefer shorter mates.

**Evaluation** (`eval.rs`): Material counting (standard centipawn values) plus piece-square tables. Separate midgame/endgame king PSTs selected by a `is_probably_endgame()` heuristic (no queens on board). Bishop pair bonus of +10.

**UCI** (`uci.rs`): Uses the `vampirc-uci` crate for parsing. Some commands (`register`, `setoption`, `stop`, `ponderhit`) have `todo!()` stubs.

## Key Conventions

- **Color encoding**: `PieceKind` uses a `BLACK_BIT` (0b1000) -- black pieces are `white_piece | 0b1000`.
- **Position encoding**: Piece positions are stored as `tzcnt` values (trailing zero count of the u64 with the piece's bit set), used as indices into attack tables and PSTs.
- **Pre-computed tables**: Attack tables for knights and kings are `lazy_static` and initialized at startup. Access via `KNIGHT_ATTACK_LIST[pos_tzcnt]`, `KING_ATTACK[pos_tzcnt]`, etc.
- **Immutable move making**: `make_move()` returns a new `Board`; `make_move_in_place()` mutates. The immutable variant is used in search to preserve the game tree.
- **`#[inline]` on hot paths**: Bit manipulation functions in `flags.rs` and `constants.rs` are annotated `#[inline]` for performance.
- **Test patterns**: Integration tests in `tests/` use helper functions like `succ(fen)` to generate successors from a FEN string and validate invariants (no pawns on ranks 1/8, halfmove clock rules, board state changes).
