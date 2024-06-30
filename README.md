## Pieby Chess Engine

Pieby is a WIP A/B duck chess engine that intends to eventually implement a WDL eval based on ideas from the Lc0 team.

| Options     | Description | Allowed range |
|    :---:    |    :----:   |     :---:     |
|    Hash     |     N/A     |      N/A      |

## Current To-Do List

- [ ] Finish magic number generation for bitboards
- [X] Finish programming Zobrist hashing
- [X] Perform bitboard tests
- [X] Integrate bitboards into position representation
- [ ] Implement minimax / negamax search
- [X] Program FEN parsing
- [ ] Program PGN parsing
- [ ] Add Hash to options
- [ ] Add basic HCE evaluation so that thing functions on basic level
- [X] Implement attacks.rs for pawns, king, and queen
- [ ] Finish pseudo-legal and legal movegen

## Notable Features

- Custom Zobrish hashing keys
- Black magic bitboards for sliding attack generation (WIP)
 - Custom black magic numbers (WIP)

## Thanks and Acknowledgements

- The Stockfish Team, for their ubiquitous influence; when in doubt, do as Stockfish does.
- Viridithas, Princhess, and Rustic for reference on how to implement ideas cleanly in Rust. Matt and I had no prior knowledge of Rust starting this project, so existing strong Rust engines provided formative practice in attempting implementations and seeing just how much better it could be executed.