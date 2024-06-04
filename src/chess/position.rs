use crate::board::Types;

struct PlayerInfo {
    color: Types::Color,
    castling_rights: bool,
    time_remaining: f64,
    increment: f64,
}

struct Position {
    turn: Types::Color,
    castling: [bool; 2],

}