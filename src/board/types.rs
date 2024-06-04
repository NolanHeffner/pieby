mod Types {
    enum PieceType {
        KING,
        QUEEN,
        ROOK,
        BISHOP,
        KNIGHT,
        PAWN
    }
    
    enum Color {
        WHITE,
        BLACK
    }
    
    impl PieceType {
        fn value(&self) -> f64 {
            match *self {
                KING => 1000,
                QUEEN => 9,
                ROOK => 5,
                BISHOP => 3.2,
                KNIGHT => 3,
                PAWN => 1.1,
            }
        }
    }
}

