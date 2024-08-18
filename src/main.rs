// use std::io;
mod uci;
mod chess;
mod board;
mod util;
mod options;

fn main() {
    let mut engine = uci::UCIEngine::new("pieby", "Nolan Heffner and Matthew Burger", "v0.1");
    engine.uci_loop();
}