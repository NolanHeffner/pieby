
use crate::board::board::Board;

fn printBoardFancy(board: &Board) {
    // Note: UCI ignores unrecognized commands, and so we can print by sending junk commands
    // ♔♕♖♗♘♙♚♛♜♝♞♟︎🦆
    println!("╔═╦═╦═╦═╦═╦═╦═╦═╗");
    for row in 1..8 {
        let mut row_out : String = String::from("║");
        for column in 1..8 {
            row_out = format!("{}{}║", row_out, board.getSquare(8 * row + column).get_ASCII());
        }
        println!("{}", row_out);
        if row != 8 {println!("╠═╬═╬═╬═╬═╬═╬═╬═╣")}
    }
    println!("╚═╩═╩═╩═╩═╩═╩═╩═╝")    
}

fn genFEN(board: &Board) {

    
}

fn genPGN() {

}