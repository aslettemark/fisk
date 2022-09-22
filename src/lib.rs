#![feature(let_chains)]

#[macro_use]
extern crate lazy_static;

extern crate vampirc_uci;

pub mod board;
pub mod constants;
pub mod engine;
pub mod eval;
pub mod fen;
pub mod move_representation;
pub mod movegen_movelist;
pub mod perft;
pub mod search;
pub mod uci;
pub mod flags;
