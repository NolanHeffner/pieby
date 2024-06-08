mod uci;

fn printBoardFancy(&board: &Board) {
    // Note: UCI ignores unrecognized commands, and so we can print by sending junk commands
    for row in 1..8 {
        for column in 1..8 {
            row = format!("║{}║{}║{}║{}║{}║{}║{}║{}║", );
        }
    }
    
    uci::sendCommand();
}

fn genFEN() {

    
}

fn genPGN() {

}

/* 
╔══════════════════════════════════════════════════════════╗
║    Name               Elo Error   Wins Loss Draw   Total ║
╠═══════════════════╬═══════════════════════════════════════╣
║  1. Mexx                +1   30    240  238   22     500 ║
║  2. Sanctaphraxx        -1   30    238  240   22     500 ║
╚══════════════════════════════════════════════════════════╝ 
*/