
mod gui;

fn printBoardFancy(&board: &Board) {
    // Note: UCI ignores unrecognized commands, and so we can print by sending junk commands
    // ♔♕♖♗♘♙♚♛♜♝♞♟︎🦆
    uci::sendCommand("╔═╦═╦═╦═╦═╦═╦═╦═╗");
    for row in 1..8 {
        let row_out : String = String::from("║");
        for column in 1..8 {
            row_out = format!("{}{}║", row_out, board.getSquare(8 * row + column).get_ASCII());
        }
        uci::sendCommand(row_out);
        if row != 8 {uci::sendCommand("╠═╬═╬═╬═╬═╬═╬═╬═╣")}
    }
    uci::sendCommand("╚═╩═╩═╩═╩═╩═╩═╩═╝")    
}

fn genFEN() {

    
}

fn genPGN() {

}