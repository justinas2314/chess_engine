enum Square {
    E,
    P{white: bool},
    N{white: bool},
    B{white: bool},
    R{white: bool},
    Q{white: bool},
    K{white: bool}
}


struct Board {
    board: [[Square; 8]; 8],
    // white kingside, white queenside, black kingside, black queenside
    castling: [bool; 4],
    last_move_two_squares_pawn_move: bool,
    // from (a, b) to (c, d)
    last_move: [usize; 4]
}