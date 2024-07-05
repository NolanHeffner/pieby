
use crate::{board::{board::Board, types::PieceType}, util::magic_gen::popcnt};

pub fn evaluation(position: &Board) -> i16 {
    // Basic material count HCE to minimize troubleshooting during early stages
    let mut eval : i16 = 0;
    for idx in 0..PieceType::COUNT {
        let pieces = position.pieces[idx];
        let count : i16 = popcnt(*(pieces & position.colors[0])) as i16 - popcnt(*(pieces & position.colors[1])) as i16;
        eval += PieceType::new(idx).value() * count;
    }
    eval
}