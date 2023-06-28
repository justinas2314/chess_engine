use crate::utils::{Board, Move, Square};


pub static PRECOMPUTED_KNIGHT_MOVES: [[[Option<(usize, usize)>; 8]; 8]; 8] =
    [[[Some((2, 1)), Some((1, 2)), None, None, None, None, None, None],
        [Some((2, 0)), Some((2, 2)), Some((1, 3)), None, None, None, None, None],
        [Some((2, 1)), Some((2, 3)), Some((1, 0)), Some((1, 4)), None, None, None, None],
        [Some((2, 2)), Some((2, 4)), Some((1, 1)), Some((1, 5)), None, None, None, None],
        [Some((2, 3)), Some((2, 5)), Some((1, 2)), Some((1, 6)), None, None, None, None],
        [Some((2, 4)), Some((2, 6)), Some((1, 3)), Some((1, 7)), None, None, None, None],
        [Some((2, 5)), Some((2, 7)), Some((1, 4)), None, None, None, None, None],
        [Some((2, 6)), Some((1, 5)), None, None, None, None, None, None]],
        [[Some((3, 1)), Some((0, 2)), Some((2, 2)), None, None, None, None, None],
            [Some((3, 0)), Some((3, 2)), Some((0, 3)), Some((2, 3)), None, None, None, None],
            [Some((3, 1)), Some((3, 3)), Some((0, 0)), Some((0, 4)), Some((2, 0)), Some((2, 4)), None, None],
            [Some((3, 2)), Some((3, 4)), Some((0, 1)), Some((0, 5)), Some((2, 1)), Some((2, 5)), None, None],
            [Some((3, 3)), Some((3, 5)), Some((0, 2)), Some((0, 6)), Some((2, 2)), Some((2, 6)), None, None],
            [Some((3, 4)), Some((3, 6)), Some((0, 3)), Some((0, 7)), Some((2, 3)), Some((2, 7)), None, None],
            [Some((3, 5)), Some((3, 7)), Some((0, 4)), Some((2, 4)), None, None, None, None],
            [Some((3, 6)), Some((0, 5)), Some((2, 5)), None, None, None, None, None]],
        [[Some((0, 1)), Some((4, 1)), Some((1, 2)), Some((3, 2)), None, None, None, None],
            [Some((0, 0)), Some((0, 2)), Some((4, 0)), Some((4, 2)), Some((1, 3)), Some((3, 3)), None, None],
            [Some((0, 1)), Some((0, 3)), Some((4, 1)), Some((4, 3)), Some((1, 0)), Some((1, 4)), Some((3, 0)), Some((3, 4))],
            [Some((0, 2)), Some((0, 4)), Some((4, 2)), Some((4, 4)), Some((1, 1)), Some((1, 5)), Some((3, 1)), Some((3, 5))],
            [Some((0, 3)), Some((0, 5)), Some((4, 3)), Some((4, 5)), Some((1, 2)), Some((1, 6)), Some((3, 2)), Some((3, 6))],
            [Some((0, 4)), Some((0, 6)), Some((4, 4)), Some((4, 6)), Some((1, 3)), Some((1, 7)), Some((3, 3)), Some((3, 7))],
            [Some((0, 5)), Some((0, 7)), Some((4, 5)), Some((4, 7)), Some((1, 4)), Some((3, 4)), None, None],
            [Some((0, 6)), Some((4, 6)), Some((1, 5)), Some((3, 5)), None, None, None, None]],
        [[Some((1, 1)), Some((5, 1)), Some((2, 2)), Some((4, 2)), None, None, None, None],
            [Some((1, 0)), Some((1, 2)), Some((5, 0)), Some((5, 2)), Some((2, 3)), Some((4, 3)), None, None],
            [Some((1, 1)), Some((1, 3)), Some((5, 1)), Some((5, 3)), Some((2, 0)), Some((2, 4)), Some((4, 0)), Some((4, 4))],
            [Some((1, 2)), Some((1, 4)), Some((5, 2)), Some((5, 4)), Some((2, 1)), Some((2, 5)), Some((4, 1)), Some((4, 5))],
            [Some((1, 3)), Some((1, 5)), Some((5, 3)), Some((5, 5)), Some((2, 2)), Some((2, 6)), Some((4, 2)), Some((4, 6))],
            [Some((1, 4)), Some((1, 6)), Some((5, 4)), Some((5, 6)), Some((2, 3)), Some((2, 7)), Some((4, 3)), Some((4, 7))],
            [Some((1, 5)), Some((1, 7)), Some((5, 5)), Some((5, 7)), Some((2, 4)), Some((4, 4)), None, None],
            [Some((1, 6)), Some((5, 6)), Some((2, 5)), Some((4, 5)), None, None, None, None]],
        [[Some((2, 1)), Some((6, 1)), Some((3, 2)), Some((5, 2)), None, None, None, None],
            [Some((2, 0)), Some((2, 2)), Some((6, 0)), Some((6, 2)), Some((3, 3)), Some((5, 3)), None, None],
            [Some((2, 1)), Some((2, 3)), Some((6, 1)), Some((6, 3)), Some((3, 0)), Some((3, 4)), Some((5, 0)), Some((5, 4))],
            [Some((2, 2)), Some((2, 4)), Some((6, 2)), Some((6, 4)), Some((3, 1)), Some((3, 5)), Some((5, 1)), Some((5, 5))],
            [Some((2, 3)), Some((2, 5)), Some((6, 3)), Some((6, 5)), Some((3, 2)), Some((3, 6)), Some((5, 2)), Some((5, 6))],
            [Some((2, 4)), Some((2, 6)), Some((6, 4)), Some((6, 6)), Some((3, 3)), Some((3, 7)), Some((5, 3)), Some((5, 7))],
            [Some((2, 5)), Some((2, 7)), Some((6, 5)), Some((6, 7)), Some((3, 4)), Some((5, 4)), None, None],
            [Some((2, 6)), Some((6, 6)), Some((3, 5)), Some((5, 5)), None, None, None, None]],
        [[Some((3, 1)), Some((7, 1)), Some((4, 2)), Some((6, 2)), None, None, None, None],
            [Some((3, 0)), Some((3, 2)), Some((7, 0)), Some((7, 2)), Some((4, 3)), Some((6, 3)), None, None],
            [Some((3, 1)), Some((3, 3)), Some((7, 1)), Some((7, 3)), Some((4, 0)), Some((4, 4)), Some((6, 0)), Some((6, 4))],
            [Some((3, 2)), Some((3, 4)), Some((7, 2)), Some((7, 4)), Some((4, 1)), Some((4, 5)), Some((6, 1)), Some((6, 5))],
            [Some((3, 3)), Some((3, 5)), Some((7, 3)), Some((7, 5)), Some((4, 2)), Some((4, 6)), Some((6, 2)), Some((6, 6))],
            [Some((3, 4)), Some((3, 6)), Some((7, 4)), Some((7, 6)), Some((4, 3)), Some((4, 7)), Some((6, 3)), Some((6, 7))],
            [Some((3, 5)), Some((3, 7)), Some((7, 5)), Some((7, 7)), Some((4, 4)), Some((6, 4)), None, None],
            [Some((3, 6)), Some((7, 6)), Some((4, 5)), Some((6, 5)), None, None, None, None]],
        [[Some((4, 1)), Some((5, 2)), Some((7, 2)), None, None, None, None, None],
            [Some((4, 0)), Some((4, 2)), Some((5, 3)), Some((7, 3)), None, None, None, None],
            [Some((4, 1)), Some((4, 3)), Some((5, 0)), Some((5, 4)), Some((7, 0)), Some((7, 4)), None, None],
            [Some((4, 2)), Some((4, 4)), Some((5, 1)), Some((5, 5)), Some((7, 1)), Some((7, 5)), None, None],
            [Some((4, 3)), Some((4, 5)), Some((5, 2)), Some((5, 6)), Some((7, 2)), Some((7, 6)), None, None],
            [Some((4, 4)), Some((4, 6)), Some((5, 3)), Some((5, 7)), Some((7, 3)), Some((7, 7)), None, None],
            [Some((4, 5)), Some((4, 7)), Some((5, 4)), Some((7, 4)), None, None, None, None],
            [Some((4, 6)), Some((5, 5)), Some((7, 5)), None, None, None, None, None]],
        [[Some((5, 1)), Some((6, 2)), None, None, None, None, None, None],
            [Some((5, 0)), Some((5, 2)), Some((6, 3)), None, None, None, None, None],
            [Some((5, 1)), Some((5, 3)), Some((6, 0)), Some((6, 4)), None, None, None, None],
            [Some((5, 2)), Some((5, 4)), Some((6, 1)), Some((6, 5)), None, None, None, None],
            [Some((5, 3)), Some((5, 5)), Some((6, 2)), Some((6, 6)), None, None, None, None],
            [Some((5, 4)), Some((5, 6)), Some((6, 3)), Some((6, 7)), None, None, None, None],
            [Some((5, 5)), Some((5, 7)), Some((6, 4)), None, None, None, None, None],
            [Some((5, 6)), Some((6, 5)), None, None, None, None, None, None]]];


pub static PRECOMPUTED_BISHOP_MOVES: [[[[Option<(usize, usize)>; 7]; 4]; 8]; 8] =
    [[[[Some((1, 1)), Some((2, 2)), Some((3, 3)), Some((4, 4)), Some((5, 5)), Some((6, 6)), Some((7, 7))],
        [None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None]],
        [[Some((1, 2)), Some((2, 3)), Some((3, 4)), Some((4, 5)), Some((5, 6)), Some((6, 7)), None],
            [Some((1, 0)), None, None, None, None, None, None],
            [None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None]],
        [[Some((1, 3)), Some((2, 4)), Some((3, 5)), Some((4, 6)), Some((5, 7)), None, None],
            [Some((1, 1)), Some((2, 0)), None, None, None, None, None],
            [None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None]],
        [[Some((1, 4)), Some((2, 5)), Some((3, 6)), Some((4, 7)), None, None, None],
            [Some((1, 2)), Some((2, 1)), Some((3, 0)), None, None, None, None],
            [None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None]],
        [[Some((1, 5)), Some((2, 6)), Some((3, 7)), None, None, None, None],
            [Some((1, 3)), Some((2, 2)), Some((3, 1)), Some((4, 0)), None, None, None],
            [None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None]],
        [[Some((1, 6)), Some((2, 7)), None, None, None, None, None],
            [Some((1, 4)), Some((2, 3)), Some((3, 2)), Some((4, 1)), Some((5, 0)), None, None],
            [None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None]],
        [[Some((1, 7)), None, None, None, None, None, None],
            [Some((1, 5)), Some((2, 4)), Some((3, 3)), Some((4, 2)), Some((5, 1)), Some((6, 0)), None],
            [None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None]],
        [[None, None, None, None, None, None, None],
            [Some((1, 6)), Some((2, 5)), Some((3, 4)), Some((4, 3)), Some((5, 2)), Some((6, 1)), Some((7, 0))],
            [None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None]]],
        [[[Some((2, 1)), Some((3, 2)), Some((4, 3)), Some((5, 4)), Some((6, 5)), Some((7, 6)), None],
            [None, None, None, None, None, None, None],
            [Some((0, 1)), None, None, None, None, None, None],
            [None, None, None, None, None, None, None]],
            [[Some((2, 2)), Some((3, 3)), Some((4, 4)), Some((5, 5)), Some((6, 6)), Some((7, 7)), None],
                [Some((2, 0)), None, None, None, None, None, None],
                [Some((0, 2)), None, None, None, None, None, None],
                [Some((0, 0)), None, None, None, None, None, None]],
            [[Some((2, 3)), Some((3, 4)), Some((4, 5)), Some((5, 6)), Some((6, 7)), None, None],
                [Some((2, 1)), Some((3, 0)), None, None, None, None, None],
                [Some((0, 3)), None, None, None, None, None, None],
                [Some((0, 1)), None, None, None, None, None, None]],
            [[Some((2, 4)), Some((3, 5)), Some((4, 6)), Some((5, 7)), None, None, None],
                [Some((2, 2)), Some((3, 1)), Some((4, 0)), None, None, None, None],
                [Some((0, 4)), None, None, None, None, None, None],
                [Some((0, 2)), None, None, None, None, None, None]],
            [[Some((2, 5)), Some((3, 6)), Some((4, 7)), None, None, None, None],
                [Some((2, 3)), Some((3, 2)), Some((4, 1)), Some((5, 0)), None, None, None],
                [Some((0, 5)), None, None, None, None, None, None],
                [Some((0, 3)), None, None, None, None, None, None]],
            [[Some((2, 6)), Some((3, 7)), None, None, None, None, None],
                [Some((2, 4)), Some((3, 3)), Some((4, 2)), Some((5, 1)), Some((6, 0)), None, None],
                [Some((0, 6)), None, None, None, None, None, None],
                [Some((0, 4)), None, None, None, None, None, None]],
            [[Some((2, 7)), None, None, None, None, None, None],
                [Some((2, 5)), Some((3, 4)), Some((4, 3)), Some((5, 2)), Some((6, 1)), Some((7, 0)), None],
                [Some((0, 7)), None, None, None, None, None, None],
                [Some((0, 5)), None, None, None, None, None, None]],
            [[None, None, None, None, None, None, None],
                [Some((2, 6)), Some((3, 5)), Some((4, 4)), Some((5, 3)), Some((6, 2)), Some((7, 1)), None],
                [None, None, None, None, None, None, None],
                [Some((0, 6)), None, None, None, None, None, None]]],
        [[[Some((3, 1)), Some((4, 2)), Some((5, 3)), Some((6, 4)), Some((7, 5)), None, None],
            [None, None, None, None, None, None, None],
            [Some((1, 1)), Some((0, 2)), None, None, None, None, None],
            [None, None, None, None, None, None, None]],
            [[Some((3, 2)), Some((4, 3)), Some((5, 4)), Some((6, 5)), Some((7, 6)), None, None],
                [Some((3, 0)), None, None, None, None, None, None],
                [Some((1, 2)), Some((0, 3)), None, None, None, None, None],
                [Some((1, 0)), None, None, None, None, None, None]],
            [[Some((3, 3)), Some((4, 4)), Some((5, 5)), Some((6, 6)), Some((7, 7)), None, None],
                [Some((3, 1)), Some((4, 0)), None, None, None, None, None],
                [Some((1, 3)), Some((0, 4)), None, None, None, None, None],
                [Some((1, 1)), Some((0, 0)), None, None, None, None, None]],
            [[Some((3, 4)), Some((4, 5)), Some((5, 6)), Some((6, 7)), None, None, None],
                [Some((3, 2)), Some((4, 1)), Some((5, 0)), None, None, None, None],
                [Some((1, 4)), Some((0, 5)), None, None, None, None, None],
                [Some((1, 2)), Some((0, 1)), None, None, None, None, None]],
            [[Some((3, 5)), Some((4, 6)), Some((5, 7)), None, None, None, None],
                [Some((3, 3)), Some((4, 2)), Some((5, 1)), Some((6, 0)), None, None, None],
                [Some((1, 5)), Some((0, 6)), None, None, None, None, None],
                [Some((1, 3)), Some((0, 2)), None, None, None, None, None]],
            [[Some((3, 6)), Some((4, 7)), None, None, None, None, None],
                [Some((3, 4)), Some((4, 3)), Some((5, 2)), Some((6, 1)), Some((7, 0)), None, None],
                [Some((1, 6)), Some((0, 7)), None, None, None, None, None],
                [Some((1, 4)), Some((0, 3)), None, None, None, None, None]],
            [[Some((3, 7)), None, None, None, None, None, None],
                [Some((3, 5)), Some((4, 4)), Some((5, 3)), Some((6, 2)), Some((7, 1)), None, None],
                [Some((1, 7)), None, None, None, None, None, None],
                [Some((1, 5)), Some((0, 4)), None, None, None, None, None]],
            [[None, None, None, None, None, None, None],
                [Some((3, 6)), Some((4, 5)), Some((5, 4)), Some((6, 3)), Some((7, 2)), None, None],
                [None, None, None, None, None, None, None],
                [Some((1, 6)), Some((0, 5)), None, None, None, None, None]]],
        [[[Some((4, 1)), Some((5, 2)), Some((6, 3)), Some((7, 4)), None, None, None],
            [None, None, None, None, None, None, None],
            [Some((2, 1)), Some((1, 2)), Some((0, 3)), None, None, None, None],
            [None, None, None, None, None, None, None]],
            [[Some((4, 2)), Some((5, 3)), Some((6, 4)), Some((7, 5)), None, None, None],
                [Some((4, 0)), None, None, None, None, None, None],
                [Some((2, 2)), Some((1, 3)), Some((0, 4)), None, None, None, None],
                [Some((2, 0)), None, None, None, None, None, None]],
            [[Some((4, 3)), Some((5, 4)), Some((6, 5)), Some((7, 6)), None, None, None],
                [Some((4, 1)), Some((5, 0)), None, None, None, None, None],
                [Some((2, 3)), Some((1, 4)), Some((0, 5)), None, None, None, None],
                [Some((2, 1)), Some((1, 0)), None, None, None, None, None]],
            [[Some((4, 4)), Some((5, 5)), Some((6, 6)), Some((7, 7)), None, None, None],
                [Some((4, 2)), Some((5, 1)), Some((6, 0)), None, None, None, None],
                [Some((2, 4)), Some((1, 5)), Some((0, 6)), None, None, None, None],
                [Some((2, 2)), Some((1, 1)), Some((0, 0)), None, None, None, None]],
            [[Some((4, 5)), Some((5, 6)), Some((6, 7)), None, None, None, None],
                [Some((4, 3)), Some((5, 2)), Some((6, 1)), Some((7, 0)), None, None, None],
                [Some((2, 5)), Some((1, 6)), Some((0, 7)), None, None, None, None],
                [Some((2, 3)), Some((1, 2)), Some((0, 1)), None, None, None, None]],
            [[Some((4, 6)), Some((5, 7)), None, None, None, None, None],
                [Some((4, 4)), Some((5, 3)), Some((6, 2)), Some((7, 1)), None, None, None],
                [Some((2, 6)), Some((1, 7)), None, None, None, None, None],
                [Some((2, 4)), Some((1, 3)), Some((0, 2)), None, None, None, None]],
            [[Some((4, 7)), None, None, None, None, None, None],
                [Some((4, 5)), Some((5, 4)), Some((6, 3)), Some((7, 2)), None, None, None],
                [Some((2, 7)), None, None, None, None, None, None],
                [Some((2, 5)), Some((1, 4)), Some((0, 3)), None, None, None, None]],
            [[None, None, None, None, None, None, None],
                [Some((4, 6)), Some((5, 5)), Some((6, 4)), Some((7, 3)), None, None, None],
                [None, None, None, None, None, None, None],
                [Some((2, 6)), Some((1, 5)), Some((0, 4)), None, None, None, None]]],
        [[[Some((5, 1)), Some((6, 2)), Some((7, 3)), None, None, None, None],
            [None, None, None, None, None, None, None],
            [Some((3, 1)), Some((2, 2)), Some((1, 3)), Some((0, 4)), None, None, None],
            [None, None, None, None, None, None, None]],
            [[Some((5, 2)), Some((6, 3)), Some((7, 4)), None, None, None, None],
                [Some((5, 0)), None, None, None, None, None, None],
                [Some((3, 2)), Some((2, 3)), Some((1, 4)), Some((0, 5)), None, None, None],
                [Some((3, 0)), None, None, None, None, None, None]],
            [[Some((5, 3)), Some((6, 4)), Some((7, 5)), None, None, None, None],
                [Some((5, 1)), Some((6, 0)), None, None, None, None, None],
                [Some((3, 3)), Some((2, 4)), Some((1, 5)), Some((0, 6)), None, None, None],
                [Some((3, 1)), Some((2, 0)), None, None, None, None, None]],
            [[Some((5, 4)), Some((6, 5)), Some((7, 6)), None, None, None, None],
                [Some((5, 2)), Some((6, 1)), Some((7, 0)), None, None, None, None],
                [Some((3, 4)), Some((2, 5)), Some((1, 6)), Some((0, 7)), None, None, None],
                [Some((3, 2)), Some((2, 1)), Some((1, 0)), None, None, None, None]],
            [[Some((5, 5)), Some((6, 6)), Some((7, 7)), None, None, None, None],
                [Some((5, 3)), Some((6, 2)), Some((7, 1)), None, None, None, None],
                [Some((3, 5)), Some((2, 6)), Some((1, 7)), None, None, None, None],
                [Some((3, 3)), Some((2, 2)), Some((1, 1)), Some((0, 0)), None, None, None]],
            [[Some((5, 6)), Some((6, 7)), None, None, None, None, None],
                [Some((5, 4)), Some((6, 3)), Some((7, 2)), None, None, None, None],
                [Some((3, 6)), Some((2, 7)), None, None, None, None, None],
                [Some((3, 4)), Some((2, 3)), Some((1, 2)), Some((0, 1)), None, None, None]],
            [[Some((5, 7)), None, None, None, None, None, None],
                [Some((5, 5)), Some((6, 4)), Some((7, 3)), None, None, None, None],
                [Some((3, 7)), None, None, None, None, None, None],
                [Some((3, 5)), Some((2, 4)), Some((1, 3)), Some((0, 2)), None, None, None]],
            [[None, None, None, None, None, None, None],
                [Some((5, 6)), Some((6, 5)), Some((7, 4)), None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((3, 6)), Some((2, 5)), Some((1, 4)), Some((0, 3)), None, None, None]]],
        [[[Some((6, 1)), Some((7, 2)), None, None, None, None, None],
            [None, None, None, None, None, None, None],
            [Some((4, 1)), Some((3, 2)), Some((2, 3)), Some((1, 4)), Some((0, 5)), None, None],
            [None, None, None, None, None, None, None]],
            [[Some((6, 2)), Some((7, 3)), None, None, None, None, None],
                [Some((6, 0)), None, None, None, None, None, None],
                [Some((4, 2)), Some((3, 3)), Some((2, 4)), Some((1, 5)), Some((0, 6)), None, None],
                [Some((4, 0)), None, None, None, None, None, None]],
            [[Some((6, 3)), Some((7, 4)), None, None, None, None, None],
                [Some((6, 1)), Some((7, 0)), None, None, None, None, None],
                [Some((4, 3)), Some((3, 4)), Some((2, 5)), Some((1, 6)), Some((0, 7)), None, None],
                [Some((4, 1)), Some((3, 0)), None, None, None, None, None]],
            [[Some((6, 4)), Some((7, 5)), None, None, None, None, None],
                [Some((6, 2)), Some((7, 1)), None, None, None, None, None],
                [Some((4, 4)), Some((3, 5)), Some((2, 6)), Some((1, 7)), None, None, None],
                [Some((4, 2)), Some((3, 1)), Some((2, 0)), None, None, None, None]],
            [[Some((6, 5)), Some((7, 6)), None, None, None, None, None],
                [Some((6, 3)), Some((7, 2)), None, None, None, None, None],
                [Some((4, 5)), Some((3, 6)), Some((2, 7)), None, None, None, None],
                [Some((4, 3)), Some((3, 2)), Some((2, 1)), Some((1, 0)), None, None, None]],
            [[Some((6, 6)), Some((7, 7)), None, None, None, None, None],
                [Some((6, 4)), Some((7, 3)), None, None, None, None, None],
                [Some((4, 6)), Some((3, 7)), None, None, None, None, None],
                [Some((4, 4)), Some((3, 3)), Some((2, 2)), Some((1, 1)), Some((0, 0)), None, None]],
            [[Some((6, 7)), None, None, None, None, None, None],
                [Some((6, 5)), Some((7, 4)), None, None, None, None, None],
                [Some((4, 7)), None, None, None, None, None, None],
                [Some((4, 5)), Some((3, 4)), Some((2, 3)), Some((1, 2)), Some((0, 1)), None, None]],
            [[None, None, None, None, None, None, None],
                [Some((6, 6)), Some((7, 5)), None, None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((4, 6)), Some((3, 5)), Some((2, 4)), Some((1, 3)), Some((0, 2)), None, None]]],
        [[[Some((7, 1)), None, None, None, None, None, None],
            [None, None, None, None, None, None, None],
            [Some((5, 1)), Some((4, 2)), Some((3, 3)), Some((2, 4)), Some((1, 5)), Some((0, 6)), None],
            [None, None, None, None, None, None, None]],
            [[Some((7, 2)), None, None, None, None, None, None],
                [Some((7, 0)), None, None, None, None, None, None],
                [Some((5, 2)), Some((4, 3)), Some((3, 4)), Some((2, 5)), Some((1, 6)), Some((0, 7)), None],
                [Some((5, 0)), None, None, None, None, None, None]],
            [[Some((7, 3)), None, None, None, None, None, None],
                [Some((7, 1)), None, None, None, None, None, None],
                [Some((5, 3)), Some((4, 4)), Some((3, 5)), Some((2, 6)), Some((1, 7)), None, None],
                [Some((5, 1)), Some((4, 0)), None, None, None, None, None]],
            [[Some((7, 4)), None, None, None, None, None, None],
                [Some((7, 2)), None, None, None, None, None, None],
                [Some((5, 4)), Some((4, 5)), Some((3, 6)), Some((2, 7)), None, None, None],
                [Some((5, 2)), Some((4, 1)), Some((3, 0)), None, None, None, None]],
            [[Some((7, 5)), None, None, None, None, None, None],
                [Some((7, 3)), None, None, None, None, None, None],
                [Some((5, 5)), Some((4, 6)), Some((3, 7)), None, None, None, None],
                [Some((5, 3)), Some((4, 2)), Some((3, 1)), Some((2, 0)), None, None, None]],
            [[Some((7, 6)), None, None, None, None, None, None],
                [Some((7, 4)), None, None, None, None, None, None],
                [Some((5, 6)), Some((4, 7)), None, None, None, None, None],
                [Some((5, 4)), Some((4, 3)), Some((3, 2)), Some((2, 1)), Some((1, 0)), None, None]],
            [[Some((7, 7)), None, None, None, None, None, None],
                [Some((7, 5)), None, None, None, None, None, None],
                [Some((5, 7)), None, None, None, None, None, None],
                [Some((5, 5)), Some((4, 4)), Some((3, 3)), Some((2, 2)), Some((1, 1)), Some((0, 0)), None]],
            [[None, None, None, None, None, None, None],
                [Some((7, 6)), None, None, None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((5, 6)), Some((4, 5)), Some((3, 4)), Some((2, 3)), Some((1, 2)), Some((0, 1)), None]]],
        [[[None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None],
            [Some((6, 1)), Some((5, 2)), Some((4, 3)), Some((3, 4)), Some((2, 5)), Some((1, 6)), Some((0, 7))],
            [None, None, None, None, None, None, None]],
            [[None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((6, 2)), Some((5, 3)), Some((4, 4)), Some((3, 5)), Some((2, 6)), Some((1, 7)), None],
                [Some((6, 0)), None, None, None, None, None, None]],
            [[None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((6, 3)), Some((5, 4)), Some((4, 5)), Some((3, 6)), Some((2, 7)), None, None],
                [Some((6, 1)), Some((5, 0)), None, None, None, None, None]],
            [[None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((6, 4)), Some((5, 5)), Some((4, 6)), Some((3, 7)), None, None, None],
                [Some((6, 2)), Some((5, 1)), Some((4, 0)), None, None, None, None]],
            [[None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((6, 5)), Some((5, 6)), Some((4, 7)), None, None, None, None],
                [Some((6, 3)), Some((5, 2)), Some((4, 1)), Some((3, 0)), None, None, None]],
            [[None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((6, 6)), Some((5, 7)), None, None, None, None, None],
                [Some((6, 4)), Some((5, 3)), Some((4, 2)), Some((3, 1)), Some((2, 0)), None, None]],
            [[None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((6, 7)), None, None, None, None, None, None],
                [Some((6, 5)), Some((5, 4)), Some((4, 3)), Some((3, 2)), Some((2, 1)), Some((1, 0)), None]],
            [[None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((6, 6)), Some((5, 5)), Some((4, 4)), Some((3, 3)), Some((2, 2)), Some((1, 1)), Some((0, 0))]]]];


pub static PRECOMPUTED_ROOK_MOVES: [[[[Option<(usize, usize)>; 7]; 4]; 8]; 8] =
    [[[[Some((1, 0)), Some((2, 0)), Some((3, 0)), Some((4, 0)), Some((5, 0)), Some((6, 0)), Some((7, 0))],
        [None, None, None, None, None, None, None],
        [Some((0, 1)), Some((0, 2)), Some((0, 3)), Some((0, 4)), Some((0, 5)), Some((0, 6)), Some((0, 7))],
        [None, None, None, None, None, None, None]],
        [[Some((1, 1)), Some((2, 1)), Some((3, 1)), Some((4, 1)), Some((5, 1)), Some((6, 1)), Some((7, 1))],
            [None, None, None, None, None, None, None],
            [Some((0, 2)), Some((0, 3)), Some((0, 4)), Some((0, 5)), Some((0, 6)), Some((0, 7)), None],
            [Some((0, 0)), None, None, None, None, None, None]],
        [[Some((1, 2)), Some((2, 2)), Some((3, 2)), Some((4, 2)), Some((5, 2)), Some((6, 2)), Some((7, 2))],
            [None, None, None, None, None, None, None],
            [Some((0, 3)), Some((0, 4)), Some((0, 5)), Some((0, 6)), Some((0, 7)), None, None],
            [Some((0, 1)), Some((0, 0)), None, None, None, None, None]],
        [[Some((1, 3)), Some((2, 3)), Some((3, 3)), Some((4, 3)), Some((5, 3)), Some((6, 3)), Some((7, 3))],
            [None, None, None, None, None, None, None],
            [Some((0, 4)), Some((0, 5)), Some((0, 6)), Some((0, 7)), None, None, None],
            [Some((0, 2)), Some((0, 1)), Some((0, 0)), None, None, None, None]],
        [[Some((1, 4)), Some((2, 4)), Some((3, 4)), Some((4, 4)), Some((5, 4)), Some((6, 4)), Some((7, 4))],
            [None, None, None, None, None, None, None],
            [Some((0, 5)), Some((0, 6)), Some((0, 7)), None, None, None, None],
            [Some((0, 3)), Some((0, 2)), Some((0, 1)), Some((0, 0)), None, None, None]],
        [[Some((1, 5)), Some((2, 5)), Some((3, 5)), Some((4, 5)), Some((5, 5)), Some((6, 5)), Some((7, 5))],
            [None, None, None, None, None, None, None],
            [Some((0, 6)), Some((0, 7)), None, None, None, None, None],
            [Some((0, 4)), Some((0, 3)), Some((0, 2)), Some((0, 1)), Some((0, 0)), None, None]],
        [[Some((1, 6)), Some((2, 6)), Some((3, 6)), Some((4, 6)), Some((5, 6)), Some((6, 6)), Some((7, 6))],
            [None, None, None, None, None, None, None],
            [Some((0, 7)), None, None, None, None, None, None],
            [Some((0, 5)), Some((0, 4)), Some((0, 3)), Some((0, 2)), Some((0, 1)), Some((0, 0)), None]],
        [[Some((1, 7)), Some((2, 7)), Some((3, 7)), Some((4, 7)), Some((5, 7)), Some((6, 7)), Some((7, 7))],
            [None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None],
            [Some((0, 6)), Some((0, 5)), Some((0, 4)), Some((0, 3)), Some((0, 2)), Some((0, 1)), Some((0, 0))]]],
        [[[Some((2, 0)), Some((3, 0)), Some((4, 0)), Some((5, 0)), Some((6, 0)), Some((7, 0)), None],
            [Some((0, 0)), None, None, None, None, None, None],
            [Some((1, 1)), Some((1, 2)), Some((1, 3)), Some((1, 4)), Some((1, 5)), Some((1, 6)), Some((1, 7))],
            [None, None, None, None, None, None, None]],
            [[Some((2, 1)), Some((3, 1)), Some((4, 1)), Some((5, 1)), Some((6, 1)), Some((7, 1)), None],
                [Some((0, 1)), None, None, None, None, None, None],
                [Some((1, 2)), Some((1, 3)), Some((1, 4)), Some((1, 5)), Some((1, 6)), Some((1, 7)), None],
                [Some((1, 0)), None, None, None, None, None, None]],
            [[Some((2, 2)), Some((3, 2)), Some((4, 2)), Some((5, 2)), Some((6, 2)), Some((7, 2)), None],
                [Some((0, 2)), None, None, None, None, None, None],
                [Some((1, 3)), Some((1, 4)), Some((1, 5)), Some((1, 6)), Some((1, 7)), None, None],
                [Some((1, 1)), Some((1, 0)), None, None, None, None, None]],
            [[Some((2, 3)), Some((3, 3)), Some((4, 3)), Some((5, 3)), Some((6, 3)), Some((7, 3)), None],
                [Some((0, 3)), None, None, None, None, None, None],
                [Some((1, 4)), Some((1, 5)), Some((1, 6)), Some((1, 7)), None, None, None],
                [Some((1, 2)), Some((1, 1)), Some((1, 0)), None, None, None, None]],
            [[Some((2, 4)), Some((3, 4)), Some((4, 4)), Some((5, 4)), Some((6, 4)), Some((7, 4)), None],
                [Some((0, 4)), None, None, None, None, None, None],
                [Some((1, 5)), Some((1, 6)), Some((1, 7)), None, None, None, None],
                [Some((1, 3)), Some((1, 2)), Some((1, 1)), Some((1, 0)), None, None, None]],
            [[Some((2, 5)), Some((3, 5)), Some((4, 5)), Some((5, 5)), Some((6, 5)), Some((7, 5)), None],
                [Some((0, 5)), None, None, None, None, None, None],
                [Some((1, 6)), Some((1, 7)), None, None, None, None, None],
                [Some((1, 4)), Some((1, 3)), Some((1, 2)), Some((1, 1)), Some((1, 0)), None, None]],
            [[Some((2, 6)), Some((3, 6)), Some((4, 6)), Some((5, 6)), Some((6, 6)), Some((7, 6)), None],
                [Some((0, 6)), None, None, None, None, None, None],
                [Some((1, 7)), None, None, None, None, None, None],
                [Some((1, 5)), Some((1, 4)), Some((1, 3)), Some((1, 2)), Some((1, 1)), Some((1, 0)), None]],
            [[Some((2, 7)), Some((3, 7)), Some((4, 7)), Some((5, 7)), Some((6, 7)), Some((7, 7)), None],
                [Some((0, 7)), None, None, None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((1, 6)), Some((1, 5)), Some((1, 4)), Some((1, 3)), Some((1, 2)), Some((1, 1)), Some((1, 0))]]],
        [[[Some((3, 0)), Some((4, 0)), Some((5, 0)), Some((6, 0)), Some((7, 0)), None, None],
            [Some((1, 0)), Some((0, 0)), None, None, None, None, None],
            [Some((2, 1)), Some((2, 2)), Some((2, 3)), Some((2, 4)), Some((2, 5)), Some((2, 6)), Some((2, 7))],
            [None, None, None, None, None, None, None]],
            [[Some((3, 1)), Some((4, 1)), Some((5, 1)), Some((6, 1)), Some((7, 1)), None, None],
                [Some((1, 1)), Some((0, 1)), None, None, None, None, None],
                [Some((2, 2)), Some((2, 3)), Some((2, 4)), Some((2, 5)), Some((2, 6)), Some((2, 7)), None],
                [Some((2, 0)), None, None, None, None, None, None]],
            [[Some((3, 2)), Some((4, 2)), Some((5, 2)), Some((6, 2)), Some((7, 2)), None, None],
                [Some((1, 2)), Some((0, 2)), None, None, None, None, None],
                [Some((2, 3)), Some((2, 4)), Some((2, 5)), Some((2, 6)), Some((2, 7)), None, None],
                [Some((2, 1)), Some((2, 0)), None, None, None, None, None]],
            [[Some((3, 3)), Some((4, 3)), Some((5, 3)), Some((6, 3)), Some((7, 3)), None, None],
                [Some((1, 3)), Some((0, 3)), None, None, None, None, None],
                [Some((2, 4)), Some((2, 5)), Some((2, 6)), Some((2, 7)), None, None, None],
                [Some((2, 2)), Some((2, 1)), Some((2, 0)), None, None, None, None]],
            [[Some((3, 4)), Some((4, 4)), Some((5, 4)), Some((6, 4)), Some((7, 4)), None, None],
                [Some((1, 4)), Some((0, 4)), None, None, None, None, None],
                [Some((2, 5)), Some((2, 6)), Some((2, 7)), None, None, None, None],
                [Some((2, 3)), Some((2, 2)), Some((2, 1)), Some((2, 0)), None, None, None]],
            [[Some((3, 5)), Some((4, 5)), Some((5, 5)), Some((6, 5)), Some((7, 5)), None, None],
                [Some((1, 5)), Some((0, 5)), None, None, None, None, None],
                [Some((2, 6)), Some((2, 7)), None, None, None, None, None],
                [Some((2, 4)), Some((2, 3)), Some((2, 2)), Some((2, 1)), Some((2, 0)), None, None]],
            [[Some((3, 6)), Some((4, 6)), Some((5, 6)), Some((6, 6)), Some((7, 6)), None, None],
                [Some((1, 6)), Some((0, 6)), None, None, None, None, None],
                [Some((2, 7)), None, None, None, None, None, None],
                [Some((2, 5)), Some((2, 4)), Some((2, 3)), Some((2, 2)), Some((2, 1)), Some((2, 0)), None]],
            [[Some((3, 7)), Some((4, 7)), Some((5, 7)), Some((6, 7)), Some((7, 7)), None, None],
                [Some((1, 7)), Some((0, 7)), None, None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((2, 6)), Some((2, 5)), Some((2, 4)), Some((2, 3)), Some((2, 2)), Some((2, 1)), Some((2, 0))]]],
        [[[Some((4, 0)), Some((5, 0)), Some((6, 0)), Some((7, 0)), None, None, None],
            [Some((2, 0)), Some((1, 0)), Some((0, 0)), None, None, None, None],
            [Some((3, 1)), Some((3, 2)), Some((3, 3)), Some((3, 4)), Some((3, 5)), Some((3, 6)), Some((3, 7))],
            [None, None, None, None, None, None, None]],
            [[Some((4, 1)), Some((5, 1)), Some((6, 1)), Some((7, 1)), None, None, None],
                [Some((2, 1)), Some((1, 1)), Some((0, 1)), None, None, None, None],
                [Some((3, 2)), Some((3, 3)), Some((3, 4)), Some((3, 5)), Some((3, 6)), Some((3, 7)), None],
                [Some((3, 0)), None, None, None, None, None, None]],
            [[Some((4, 2)), Some((5, 2)), Some((6, 2)), Some((7, 2)), None, None, None],
                [Some((2, 2)), Some((1, 2)), Some((0, 2)), None, None, None, None],
                [Some((3, 3)), Some((3, 4)), Some((3, 5)), Some((3, 6)), Some((3, 7)), None, None],
                [Some((3, 1)), Some((3, 0)), None, None, None, None, None]],
            [[Some((4, 3)), Some((5, 3)), Some((6, 3)), Some((7, 3)), None, None, None],
                [Some((2, 3)), Some((1, 3)), Some((0, 3)), None, None, None, None],
                [Some((3, 4)), Some((3, 5)), Some((3, 6)), Some((3, 7)), None, None, None],
                [Some((3, 2)), Some((3, 1)), Some((3, 0)), None, None, None, None]],
            [[Some((4, 4)), Some((5, 4)), Some((6, 4)), Some((7, 4)), None, None, None],
                [Some((2, 4)), Some((1, 4)), Some((0, 4)), None, None, None, None],
                [Some((3, 5)), Some((3, 6)), Some((3, 7)), None, None, None, None],
                [Some((3, 3)), Some((3, 2)), Some((3, 1)), Some((3, 0)), None, None, None]],
            [[Some((4, 5)), Some((5, 5)), Some((6, 5)), Some((7, 5)), None, None, None],
                [Some((2, 5)), Some((1, 5)), Some((0, 5)), None, None, None, None],
                [Some((3, 6)), Some((3, 7)), None, None, None, None, None],
                [Some((3, 4)), Some((3, 3)), Some((3, 2)), Some((3, 1)), Some((3, 0)), None, None]],
            [[Some((4, 6)), Some((5, 6)), Some((6, 6)), Some((7, 6)), None, None, None],
                [Some((2, 6)), Some((1, 6)), Some((0, 6)), None, None, None, None],
                [Some((3, 7)), None, None, None, None, None, None],
                [Some((3, 5)), Some((3, 4)), Some((3, 3)), Some((3, 2)), Some((3, 1)), Some((3, 0)), None]],
            [[Some((4, 7)), Some((5, 7)), Some((6, 7)), Some((7, 7)), None, None, None],
                [Some((2, 7)), Some((1, 7)), Some((0, 7)), None, None, None, None],
                [None, None, None, None, None, None, None],
                [Some((3, 6)), Some((3, 5)), Some((3, 4)), Some((3, 3)), Some((3, 2)), Some((3, 1)), Some((3, 0))]]],
        [[[Some((5, 0)), Some((6, 0)), Some((7, 0)), None, None, None, None],
            [Some((3, 0)), Some((2, 0)), Some((1, 0)), Some((0, 0)), None, None, None],
            [Some((4, 1)), Some((4, 2)), Some((4, 3)), Some((4, 4)), Some((4, 5)), Some((4, 6)), Some((4, 7))],
            [None, None, None, None, None, None, None]],
            [[Some((5, 1)), Some((6, 1)), Some((7, 1)), None, None, None, None],
                [Some((3, 1)), Some((2, 1)), Some((1, 1)), Some((0, 1)), None, None, None],
                [Some((4, 2)), Some((4, 3)), Some((4, 4)), Some((4, 5)), Some((4, 6)), Some((4, 7)), None],
                [Some((4, 0)), None, None, None, None, None, None]],
            [[Some((5, 2)), Some((6, 2)), Some((7, 2)), None, None, None, None],
                [Some((3, 2)), Some((2, 2)), Some((1, 2)), Some((0, 2)), None, None, None],
                [Some((4, 3)), Some((4, 4)), Some((4, 5)), Some((4, 6)), Some((4, 7)), None, None],
                [Some((4, 1)), Some((4, 0)), None, None, None, None, None]],
            [[Some((5, 3)), Some((6, 3)), Some((7, 3)), None, None, None, None],
                [Some((3, 3)), Some((2, 3)), Some((1, 3)), Some((0, 3)), None, None, None],
                [Some((4, 4)), Some((4, 5)), Some((4, 6)), Some((4, 7)), None, None, None],
                [Some((4, 2)), Some((4, 1)), Some((4, 0)), None, None, None, None]],
            [[Some((5, 4)), Some((6, 4)), Some((7, 4)), None, None, None, None],
                [Some((3, 4)), Some((2, 4)), Some((1, 4)), Some((0, 4)), None, None, None],
                [Some((4, 5)), Some((4, 6)), Some((4, 7)), None, None, None, None],
                [Some((4, 3)), Some((4, 2)), Some((4, 1)), Some((4, 0)), None, None, None]],
            [[Some((5, 5)), Some((6, 5)), Some((7, 5)), None, None, None, None],
                [Some((3, 5)), Some((2, 5)), Some((1, 5)), Some((0, 5)), None, None, None],
                [Some((4, 6)), Some((4, 7)), None, None, None, None, None],
                [Some((4, 4)), Some((4, 3)), Some((4, 2)), Some((4, 1)), Some((4, 0)), None, None]],
            [[Some((5, 6)), Some((6, 6)), Some((7, 6)), None, None, None, None],
                [Some((3, 6)), Some((2, 6)), Some((1, 6)), Some((0, 6)), None, None, None],
                [Some((4, 7)), None, None, None, None, None, None],
                [Some((4, 5)), Some((4, 4)), Some((4, 3)), Some((4, 2)), Some((4, 1)), Some((4, 0)), None]],
            [[Some((5, 7)), Some((6, 7)), Some((7, 7)), None, None, None, None],
                [Some((3, 7)), Some((2, 7)), Some((1, 7)), Some((0, 7)), None, None, None],
                [None, None, None, None, None, None, None],
                [Some((4, 6)), Some((4, 5)), Some((4, 4)), Some((4, 3)), Some((4, 2)), Some((4, 1)), Some((4, 0))]]],
        [[[Some((6, 0)), Some((7, 0)), None, None, None, None, None],
            [Some((4, 0)), Some((3, 0)), Some((2, 0)), Some((1, 0)), Some((0, 0)), None, None],
            [Some((5, 1)), Some((5, 2)), Some((5, 3)), Some((5, 4)), Some((5, 5)), Some((5, 6)), Some((5, 7))],
            [None, None, None, None, None, None, None]],
            [[Some((6, 1)), Some((7, 1)), None, None, None, None, None],
                [Some((4, 1)), Some((3, 1)), Some((2, 1)), Some((1, 1)), Some((0, 1)), None, None],
                [Some((5, 2)), Some((5, 3)), Some((5, 4)), Some((5, 5)), Some((5, 6)), Some((5, 7)), None],
                [Some((5, 0)), None, None, None, None, None, None]],
            [[Some((6, 2)), Some((7, 2)), None, None, None, None, None],
                [Some((4, 2)), Some((3, 2)), Some((2, 2)), Some((1, 2)), Some((0, 2)), None, None],
                [Some((5, 3)), Some((5, 4)), Some((5, 5)), Some((5, 6)), Some((5, 7)), None, None],
                [Some((5, 1)), Some((5, 0)), None, None, None, None, None]],
            [[Some((6, 3)), Some((7, 3)), None, None, None, None, None],
                [Some((4, 3)), Some((3, 3)), Some((2, 3)), Some((1, 3)), Some((0, 3)), None, None],
                [Some((5, 4)), Some((5, 5)), Some((5, 6)), Some((5, 7)), None, None, None],
                [Some((5, 2)), Some((5, 1)), Some((5, 0)), None, None, None, None]],
            [[Some((6, 4)), Some((7, 4)), None, None, None, None, None],
                [Some((4, 4)), Some((3, 4)), Some((2, 4)), Some((1, 4)), Some((0, 4)), None, None],
                [Some((5, 5)), Some((5, 6)), Some((5, 7)), None, None, None, None],
                [Some((5, 3)), Some((5, 2)), Some((5, 1)), Some((5, 0)), None, None, None]],
            [[Some((6, 5)), Some((7, 5)), None, None, None, None, None],
                [Some((4, 5)), Some((3, 5)), Some((2, 5)), Some((1, 5)), Some((0, 5)), None, None],
                [Some((5, 6)), Some((5, 7)), None, None, None, None, None],
                [Some((5, 4)), Some((5, 3)), Some((5, 2)), Some((5, 1)), Some((5, 0)), None, None]],
            [[Some((6, 6)), Some((7, 6)), None, None, None, None, None],
                [Some((4, 6)), Some((3, 6)), Some((2, 6)), Some((1, 6)), Some((0, 6)), None, None],
                [Some((5, 7)), None, None, None, None, None, None],
                [Some((5, 5)), Some((5, 4)), Some((5, 3)), Some((5, 2)), Some((5, 1)), Some((5, 0)), None]],
            [[Some((6, 7)), Some((7, 7)), None, None, None, None, None],
                [Some((4, 7)), Some((3, 7)), Some((2, 7)), Some((1, 7)), Some((0, 7)), None, None],
                [None, None, None, None, None, None, None],
                [Some((5, 6)), Some((5, 5)), Some((5, 4)), Some((5, 3)), Some((5, 2)), Some((5, 1)), Some((5, 0))]]],
        [[[Some((7, 0)), None, None, None, None, None, None],
            [Some((5, 0)), Some((4, 0)), Some((3, 0)), Some((2, 0)), Some((1, 0)), Some((0, 0)), None],
            [Some((6, 1)), Some((6, 2)), Some((6, 3)), Some((6, 4)), Some((6, 5)), Some((6, 6)), Some((6, 7))],
            [None, None, None, None, None, None, None]],
            [[Some((7, 1)), None, None, None, None, None, None],
                [Some((5, 1)), Some((4, 1)), Some((3, 1)), Some((2, 1)), Some((1, 1)), Some((0, 1)), None],
                [Some((6, 2)), Some((6, 3)), Some((6, 4)), Some((6, 5)), Some((6, 6)), Some((6, 7)), None],
                [Some((6, 0)), None, None, None, None, None, None]],
            [[Some((7, 2)), None, None, None, None, None, None],
                [Some((5, 2)), Some((4, 2)), Some((3, 2)), Some((2, 2)), Some((1, 2)), Some((0, 2)), None],
                [Some((6, 3)), Some((6, 4)), Some((6, 5)), Some((6, 6)), Some((6, 7)), None, None],
                [Some((6, 1)), Some((6, 0)), None, None, None, None, None]],
            [[Some((7, 3)), None, None, None, None, None, None],
                [Some((5, 3)), Some((4, 3)), Some((3, 3)), Some((2, 3)), Some((1, 3)), Some((0, 3)), None],
                [Some((6, 4)), Some((6, 5)), Some((6, 6)), Some((6, 7)), None, None, None],
                [Some((6, 2)), Some((6, 1)), Some((6, 0)), None, None, None, None]],
            [[Some((7, 4)), None, None, None, None, None, None],
                [Some((5, 4)), Some((4, 4)), Some((3, 4)), Some((2, 4)), Some((1, 4)), Some((0, 4)), None],
                [Some((6, 5)), Some((6, 6)), Some((6, 7)), None, None, None, None],
                [Some((6, 3)), Some((6, 2)), Some((6, 1)), Some((6, 0)), None, None, None]],
            [[Some((7, 5)), None, None, None, None, None, None],
                [Some((5, 5)), Some((4, 5)), Some((3, 5)), Some((2, 5)), Some((1, 5)), Some((0, 5)), None],
                [Some((6, 6)), Some((6, 7)), None, None, None, None, None],
                [Some((6, 4)), Some((6, 3)), Some((6, 2)), Some((6, 1)), Some((6, 0)), None, None]],
            [[Some((7, 6)), None, None, None, None, None, None],
                [Some((5, 6)), Some((4, 6)), Some((3, 6)), Some((2, 6)), Some((1, 6)), Some((0, 6)), None],
                [Some((6, 7)), None, None, None, None, None, None],
                [Some((6, 5)), Some((6, 4)), Some((6, 3)), Some((6, 2)), Some((6, 1)), Some((6, 0)), None]],
            [[Some((7, 7)), None, None, None, None, None, None],
                [Some((5, 7)), Some((4, 7)), Some((3, 7)), Some((2, 7)), Some((1, 7)), Some((0, 7)), None],
                [None, None, None, None, None, None, None],
                [Some((6, 6)), Some((6, 5)), Some((6, 4)), Some((6, 3)), Some((6, 2)), Some((6, 1)), Some((6, 0))]]],
        [[[None, None, None, None, None, None, None],
            [Some((6, 0)), Some((5, 0)), Some((4, 0)), Some((3, 0)), Some((2, 0)), Some((1, 0)), Some((0, 0))],
            [Some((7, 1)), Some((7, 2)), Some((7, 3)), Some((7, 4)), Some((7, 5)), Some((7, 6)), Some((7, 7))],
            [None, None, None, None, None, None, None]],
            [[None, None, None, None, None, None, None],
                [Some((6, 1)), Some((5, 1)), Some((4, 1)), Some((3, 1)), Some((2, 1)), Some((1, 1)), Some((0, 1))],
                [Some((7, 2)), Some((7, 3)), Some((7, 4)), Some((7, 5)), Some((7, 6)), Some((7, 7)), None],
                [Some((7, 0)), None, None, None, None, None, None]],
            [[None, None, None, None, None, None, None],
                [Some((6, 2)), Some((5, 2)), Some((4, 2)), Some((3, 2)), Some((2, 2)), Some((1, 2)), Some((0, 2))],
                [Some((7, 3)), Some((7, 4)), Some((7, 5)), Some((7, 6)), Some((7, 7)), None, None],
                [Some((7, 1)), Some((7, 0)), None, None, None, None, None]],
            [[None, None, None, None, None, None, None],
                [Some((6, 3)), Some((5, 3)), Some((4, 3)), Some((3, 3)), Some((2, 3)), Some((1, 3)), Some((0, 3))],
                [Some((7, 4)), Some((7, 5)), Some((7, 6)), Some((7, 7)), None, None, None],
                [Some((7, 2)), Some((7, 1)), Some((7, 0)), None, None, None, None]],
            [[None, None, None, None, None, None, None],
                [Some((6, 4)), Some((5, 4)), Some((4, 4)), Some((3, 4)), Some((2, 4)), Some((1, 4)), Some((0, 4))],
                [Some((7, 5)), Some((7, 6)), Some((7, 7)), None, None, None, None],
                [Some((7, 3)), Some((7, 2)), Some((7, 1)), Some((7, 0)), None, None, None]],
            [[None, None, None, None, None, None, None],
                [Some((6, 5)), Some((5, 5)), Some((4, 5)), Some((3, 5)), Some((2, 5)), Some((1, 5)), Some((0, 5))],
                [Some((7, 6)), Some((7, 7)), None, None, None, None, None],
                [Some((7, 4)), Some((7, 3)), Some((7, 2)), Some((7, 1)), Some((7, 0)), None, None]],
            [[None, None, None, None, None, None, None],
                [Some((6, 6)), Some((5, 6)), Some((4, 6)), Some((3, 6)), Some((2, 6)), Some((1, 6)), Some((0, 6))],
                [Some((7, 7)), None, None, None, None, None, None],
                [Some((7, 5)), Some((7, 4)), Some((7, 3)), Some((7, 2)), Some((7, 1)), Some((7, 0)), None]],
            [[None, None, None, None, None, None, None],
                [Some((6, 7)), Some((5, 7)), Some((4, 7)), Some((3, 7)), Some((2, 7)), Some((1, 7)), Some((0, 7))],
                [None, None, None, None, None, None, None],
                [Some((7, 6)), Some((7, 5)), Some((7, 4)), Some((7, 3)), Some((7, 2)), Some((7, 1)), Some((7, 0))]]]];

pub static PRECOMPUTED_BISHOP_CHECKS: [[[[[Option<(usize, usize)>; 2]; 8]; 8]; 8]; 8] = [[[[[Some((0, 0)), Some((0, 0))], [None, None], [Some((1, 1)), None], [None, None], [Some((2, 2)), None], [None, None], [Some((3, 3)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 2)), None], [None, None], [Some((3, 3)), None], [None, None], [Some((4, 4)), None]], [[Some((1, 1)), None], [None, None], [None, None], [None, None], [Some((3, 3)), None], [None, None], [Some((4, 4)), None], [None, None]], [[None, None], [Some((2, 2)), None], [None, None], [None, None], [None, None], [Some((4, 4)), None], [None, None], [Some((5, 5)), None]], [[Some((2, 2)), None], [None, None], [Some((3, 3)), None], [None, None], [None, None], [None, None], [Some((5, 5)), None], [None, None]], [[None, None], [Some((3, 3)), None], [None, None], [Some((4, 4)), None], [None, None], [None, None], [None, None], [Some((6, 6)), None]], [[Some((3, 3)), None], [None, None], [Some((4, 4)), None], [None, None], [Some((5, 5)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 4)), None], [None, None], [Some((5, 5)), None], [None, None], [Some((6, 6)), None], [None, None], [None, None]]], [[[None, None], [Some((0, 1)), Some((0, 1))], [None, None], [Some((1, 2)), None], [None, None], [Some((2, 3)), None], [None, None], [Some((3, 4)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((2, 3)), None], [None, None], [Some((3, 4)), None], [None, None]], [[None, None], [Some((1, 2)), Some((1, 0))], [None, None], [None, None], [None, None], [Some((3, 4)), None], [None, None], [Some((4, 5)), None]], [[Some((1, 2)), None], [None, None], [Some((2, 3)), Some((1, 0))], [None, None], [None, None], [None, None], [Some((4, 5)), None], [None, None]], [[None, None], [Some((2, 3)), None], [None, None], [Some((3, 4)), Some((1, 0))], [None, None], [None, None], [None, None], [Some((5, 6)), None]], [[Some((2, 3)), None], [None, None], [Some((3, 4)), None], [None, None], [Some((4, 5)), Some((1, 0))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 4)), None], [None, None], [Some((4, 5)), None], [None, None], [Some((5, 6)), Some((1, 0))], [None, None], [None, None]], [[Some((3, 4)), None], [None, None], [Some((4, 5)), None], [None, None], [Some((5, 6)), None], [None, None], [Some((6, 7)), Some((1, 0))], [None, None]]], [[[Some((1, 1)), None], [None, None], [Some((0, 2)), Some((0, 2))], [None, None], [Some((1, 3)), None], [None, None], [Some((2, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((2, 4)), None], [None, None], [Some((3, 5)), None]], [[None, None], [None, None], [Some((1, 3)), Some((1, 1))], [None, None], [None, None], [None, None], [Some((3, 5)), None], [None, None]], [[None, None], [Some((1, 3)), Some((2, 0))], [None, None], [Some((2, 4)), Some((1, 1))], [None, None], [None, None], [None, None], [Some((4, 6)), None]], [[Some((1, 3)), None], [None, None], [Some((2, 4)), Some((2, 0))], [None, None], [Some((3, 5)), Some((1, 1))], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 4)), None], [None, None], [Some((3, 5)), Some((2, 0))], [None, None], [Some((4, 6)), Some((1, 1))], [None, None], [None, None]], [[Some((2, 4)), None], [None, None], [Some((3, 5)), None], [None, None], [Some((4, 6)), Some((2, 0))], [None, None], [Some((5, 7)), Some((1, 1))], [None, None]], [[None, None], [Some((3, 5)), None], [None, None], [Some((4, 6)), None], [None, None], [Some((5, 7)), Some((2, 0))], [None, None], [Some((1, 1)), None]]], [[[None, None], [Some((1, 2)), None], [None, None], [Some((0, 3)), Some((0, 3))], [None, None], [Some((1, 4)), None], [None, None], [Some((2, 5)), None]], [[Some((2, 1)), None], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((2, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((1, 4)), Some((1, 2))], [None, None], [None, None], [None, None], [Some((3, 6)), None]], [[None, None], [None, None], [Some((1, 4)), Some((2, 1))], [None, None], [Some((2, 5)), Some((1, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((1, 4)), Some((3, 0))], [None, None], [Some((2, 5)), Some((2, 1))], [None, None], [Some((3, 6)), Some((1, 2))], [None, None], [None, None]], [[Some((1, 4)), None], [None, None], [Some((2, 5)), Some((3, 0))], [None, None], [Some((3, 6)), Some((2, 1))], [None, None], [Some((4, 7)), Some((1, 2))], [None, None]], [[None, None], [Some((2, 5)), None], [None, None], [Some((3, 6)), Some((3, 0))], [None, None], [Some((4, 7)), Some((2, 1))], [None, None], [Some((1, 2)), None]], [[Some((2, 5)), None], [None, None], [Some((3, 6)), None], [None, None], [Some((4, 7)), Some((3, 0))], [None, None], [Some((2, 1)), None], [None, None]]], [[[Some((2, 2)), None], [None, None], [Some((1, 3)), None], [None, None], [Some((0, 4)), Some((0, 4))], [None, None], [Some((1, 5)), None], [None, None]], [[None, None], [Some((2, 2)), None], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((2, 6)), None]], [[Some((3, 1)), None], [None, None], [None, None], [None, None], [Some((1, 5)), Some((1, 3))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((1, 5)), Some((2, 2))], [None, None], [Some((2, 6)), Some((1, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((1, 5)), Some((3, 1))], [None, None], [Some((2, 6)), Some((2, 2))], [None, None], [Some((3, 7)), Some((1, 3))], [None, None]], [[None, None], [Some((1, 5)), Some((4, 0))], [None, None], [Some((2, 6)), Some((3, 1))], [None, None], [Some((3, 7)), Some((2, 2))], [None, None], [Some((1, 3)), None]], [[Some((1, 5)), None], [None, None], [Some((2, 6)), Some((4, 0))], [None, None], [Some((3, 7)), Some((3, 1))], [None, None], [Some((2, 2)), None], [None, None]], [[None, None], [Some((2, 6)), None], [None, None], [Some((3, 7)), Some((4, 0))], [None, None], [Some((3, 1)), None], [None, None], [Some((2, 2)), None]]], [[[None, None], [Some((2, 3)), None], [None, None], [Some((1, 4)), None], [None, None], [Some((0, 5)), Some((0, 5))], [None, None], [Some((1, 6)), None]], [[Some((3, 2)), None], [None, None], [Some((2, 3)), None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 2)), None], [None, None], [None, None], [None, None], [Some((1, 6)), Some((1, 4))], [None, None], [None, None]], [[Some((4, 1)), None], [None, None], [None, None], [None, None], [Some((1, 6)), Some((2, 3))], [None, None], [Some((2, 7)), Some((1, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((1, 6)), Some((3, 2))], [None, None], [Some((2, 7)), Some((2, 3))], [None, None], [Some((1, 4)), None]], [[None, None], [None, None], [Some((1, 6)), Some((4, 1))], [None, None], [Some((2, 7)), Some((3, 2))], [None, None], [Some((2, 3)), None], [None, None]], [[None, None], [Some((1, 6)), Some((5, 0))], [None, None], [Some((2, 7)), Some((4, 1))], [None, None], [Some((3, 2)), None], [None, None], [Some((2, 3)), None]], [[Some((1, 6)), None], [None, None], [Some((2, 7)), Some((5, 0))], [None, None], [Some((4, 1)), None], [None, None], [Some((3, 2)), None], [None, None]]], [[[Some((3, 3)), None], [None, None], [Some((2, 4)), None], [None, None], [Some((1, 5)), None], [None, None], [Some((0, 6)), Some((0, 6))], [None, None]], [[None, None], [Some((3, 3)), None], [None, None], [Some((2, 4)), None], [None, None], [None, None], [None, None], [None, None]], [[Some((4, 2)), None], [None, None], [Some((3, 3)), None], [None, None], [None, None], [None, None], [Some((1, 7)), Some((1, 5))], [None, None]], [[None, None], [Some((4, 2)), None], [None, None], [None, None], [None, None], [Some((1, 7)), Some((2, 4))], [None, None], [Some((1, 5)), None]], [[Some((5, 1)), None], [None, None], [None, None], [None, None], [Some((1, 7)), Some((3, 3))], [None, None], [Some((2, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((1, 7)), Some((4, 2))], [None, None], [Some((3, 3)), None], [None, None], [Some((2, 4)), None]], [[None, None], [None, None], [Some((1, 7)), Some((5, 1))], [None, None], [Some((4, 2)), None], [None, None], [Some((3, 3)), None], [None, None]], [[None, None], [Some((1, 7)), Some((6, 0))], [None, None], [Some((5, 1)), None], [None, None], [Some((4, 2)), None], [None, None], [Some((3, 3)), None]]], [[[None, None], [Some((3, 4)), None], [None, None], [Some((2, 5)), None], [None, None], [Some((1, 6)), None], [None, None], [Some((0, 7)), Some((0, 7))]], [[Some((4, 3)), None], [None, None], [Some((3, 4)), None], [None, None], [Some((2, 5)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 3)), None], [None, None], [Some((3, 4)), None], [None, None], [None, None], [None, None], [Some((1, 6)), None]], [[Some((5, 2)), None], [None, None], [Some((4, 3)), None], [None, None], [None, None], [None, None], [Some((2, 5)), None], [None, None]], [[None, None], [Some((5, 2)), None], [None, None], [None, None], [None, None], [Some((3, 4)), None], [None, None], [Some((2, 5)), None]], [[Some((6, 1)), None], [None, None], [None, None], [None, None], [Some((4, 3)), None], [None, None], [Some((3, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 2)), None], [None, None], [Some((4, 3)), None], [None, None], [Some((3, 4)), None]], [[None, None], [None, None], [Some((6, 1)), None], [None, None], [Some((5, 2)), None], [None, None], [Some((4, 3)), None], [None, None]]]], [[[[None, None], [None, None], [None, None], [Some((2, 1)), None], [None, None], [Some((3, 2)), None], [None, None], [Some((4, 3)), None]], [[Some((1, 0)), Some((1, 0))], [None, None], [Some((2, 1)), Some((0, 1))], [None, None], [Some((3, 2)), None], [None, None], [Some((4, 3)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 2)), Some((0, 1))], [None, None], [Some((4, 3)), None], [None, None], [Some((5, 4)), None]], [[Some((2, 1)), None], [None, None], [None, None], [None, None], [Some((4, 3)), Some((0, 1))], [None, None], [Some((5, 4)), None], [None, None]], [[None, None], [Some((3, 2)), None], [None, None], [None, None], [None, None], [Some((5, 4)), Some((0, 1))], [None, None], [Some((6, 5)), None]], [[Some((3, 2)), None], [None, None], [Some((4, 3)), None], [None, None], [None, None], [None, None], [Some((6, 5)), Some((0, 1))], [None, None]], [[None, None], [Some((4, 3)), None], [None, None], [Some((5, 4)), None], [None, None], [None, None], [None, None], [Some((7, 6)), Some((0, 1))]], [[Some((4, 3)), None], [None, None], [Some((5, 4)), None], [None, None], [Some((6, 5)), None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [Some((2, 2)), None], [None, None], [Some((3, 3)), None], [None, None]], [[None, None], [Some((1, 1)), Some((1, 1))], [None, None], [Some((2, 2)), Some((0, 2))], [None, None], [Some((3, 3)), None], [None, None], [Some((4, 4)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((3, 3)), Some((0, 2))], [None, None], [Some((4, 4)), None], [None, None]], [[None, None], [Some((2, 2)), Some((2, 0))], [None, None], [None, None], [None, None], [Some((4, 4)), Some((0, 2))], [None, None], [Some((5, 5)), None]], [[Some((2, 2)), None], [None, None], [Some((3, 3)), Some((2, 0))], [None, None], [None, None], [None, None], [Some((5, 5)), Some((0, 2))], [None, None]], [[None, None], [Some((3, 3)), None], [None, None], [Some((4, 4)), Some((2, 0))], [None, None], [None, None], [None, None], [Some((6, 6)), Some((0, 2))]], [[Some((3, 3)), None], [None, None], [Some((4, 4)), None], [None, None], [Some((5, 5)), Some((2, 0))], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 4)), None], [None, None], [Some((5, 5)), None], [None, None], [Some((6, 6)), Some((2, 0))], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [Some((2, 3)), None], [None, None], [Some((3, 4)), None]], [[Some((0, 1)), Some((2, 1))], [None, None], [Some((1, 2)), Some((1, 2))], [None, None], [Some((2, 3)), Some((0, 3))], [None, None], [Some((3, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((3, 4)), Some((0, 3))], [None, None], [Some((4, 5)), None]], [[None, None], [None, None], [Some((2, 3)), Some((2, 1))], [None, None], [None, None], [None, None], [Some((4, 5)), Some((0, 3))], [None, None]], [[None, None], [Some((2, 3)), Some((3, 0))], [None, None], [Some((3, 4)), Some((2, 1))], [None, None], [None, None], [None, None], [Some((5, 6)), Some((0, 3))]], [[Some((2, 3)), None], [None, None], [Some((3, 4)), Some((3, 0))], [None, None], [Some((4, 5)), Some((2, 1))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 4)), None], [None, None], [Some((4, 5)), Some((3, 0))], [None, None], [Some((5, 6)), Some((2, 1))], [None, None], [None, None]], [[Some((3, 4)), None], [None, None], [Some((4, 5)), None], [None, None], [Some((5, 6)), Some((3, 0))], [None, None], [Some((6, 7)), Some((2, 1))], [None, None]]], [[[Some((2, 2)), None], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((2, 4)), None], [None, None]], [[None, None], [Some((0, 2)), Some((2, 2))], [None, None], [Some((1, 3)), Some((1, 3))], [None, None], [Some((2, 4)), Some((0, 4))], [None, None], [Some((3, 5)), None]], [[Some((0, 2)), Some((3, 1))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((3, 5)), Some((0, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 4)), Some((2, 2))], [None, None], [None, None], [None, None], [Some((4, 6)), Some((0, 4))]], [[None, None], [None, None], [Some((2, 4)), Some((3, 1))], [None, None], [Some((3, 5)), Some((2, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 4)), Some((4, 0))], [None, None], [Some((3, 5)), Some((3, 1))], [None, None], [Some((4, 6)), Some((2, 2))], [None, None], [None, None]], [[Some((2, 4)), None], [None, None], [Some((3, 5)), Some((4, 0))], [None, None], [Some((4, 6)), Some((3, 1))], [None, None], [Some((5, 7)), Some((2, 2))], [None, None]], [[None, None], [Some((3, 5)), None], [None, None], [Some((4, 6)), Some((4, 0))], [None, None], [Some((5, 7)), Some((3, 1))], [None, None], [Some((2, 2)), None]]], [[[None, None], [Some((2, 3)), None], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((2, 5)), None]], [[Some((3, 2)), None], [None, None], [Some((0, 3)), Some((2, 3))], [None, None], [Some((1, 4)), Some((1, 4))], [None, None], [Some((2, 5)), Some((0, 5))], [None, None]], [[None, None], [Some((0, 3)), Some((3, 2))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((3, 6)), Some((0, 5))]], [[Some((0, 3)), Some((4, 1))], [None, None], [None, None], [None, None], [Some((2, 5)), Some((2, 3))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 5)), Some((3, 2))], [None, None], [Some((3, 6)), Some((2, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((2, 5)), Some((4, 1))], [None, None], [Some((3, 6)), Some((3, 2))], [None, None], [Some((4, 7)), Some((2, 3))], [None, None]], [[None, None], [Some((2, 5)), Some((5, 0))], [None, None], [Some((3, 6)), Some((4, 1))], [None, None], [Some((4, 7)), Some((3, 2))], [None, None], [Some((2, 3)), None]], [[Some((2, 5)), None], [None, None], [Some((3, 6)), Some((5, 0))], [None, None], [Some((4, 7)), Some((4, 1))], [None, None], [Some((3, 2)), None], [None, None]]], [[[Some((3, 3)), None], [None, None], [Some((2, 4)), None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 3)), None], [None, None], [Some((0, 4)), Some((2, 4))], [None, None], [Some((1, 5)), Some((1, 5))], [None, None], [Some((2, 6)), Some((0, 6))]], [[Some((4, 2)), None], [None, None], [Some((0, 4)), Some((3, 3))], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((0, 4)), Some((4, 2))], [None, None], [None, None], [None, None], [Some((2, 6)), Some((2, 4))], [None, None], [None, None]], [[Some((0, 4)), Some((5, 1))], [None, None], [None, None], [None, None], [Some((2, 6)), Some((3, 3))], [None, None], [Some((3, 7)), Some((2, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 6)), Some((4, 2))], [None, None], [Some((3, 7)), Some((3, 3))], [None, None], [Some((2, 4)), None]], [[None, None], [None, None], [Some((2, 6)), Some((5, 1))], [None, None], [Some((3, 7)), Some((4, 2))], [None, None], [Some((3, 3)), None], [None, None]], [[None, None], [Some((2, 6)), Some((6, 0))], [None, None], [Some((3, 7)), Some((5, 1))], [None, None], [Some((4, 2)), None], [None, None], [Some((3, 3)), None]]], [[[None, None], [Some((3, 4)), None], [None, None], [Some((2, 5)), None], [None, None], [None, None], [None, None], [None, None]], [[Some((4, 3)), None], [None, None], [Some((3, 4)), None], [None, None], [Some((0, 5)), Some((2, 5))], [None, None], [Some((1, 6)), Some((1, 6))], [None, None]], [[None, None], [Some((4, 3)), None], [None, None], [Some((0, 5)), Some((3, 4))], [None, None], [None, None], [None, None], [None, None]], [[Some((5, 2)), None], [None, None], [Some((0, 5)), Some((4, 3))], [None, None], [None, None], [None, None], [Some((2, 7)), Some((2, 5))], [None, None]], [[None, None], [Some((0, 5)), Some((5, 2))], [None, None], [None, None], [None, None], [Some((2, 7)), Some((3, 4))], [None, None], [Some((2, 5)), None]], [[Some((0, 5)), Some((6, 1))], [None, None], [None, None], [None, None], [Some((2, 7)), Some((4, 3))], [None, None], [Some((3, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 7)), Some((5, 2))], [None, None], [Some((4, 3)), None], [None, None], [Some((3, 4)), None]], [[None, None], [None, None], [Some((2, 7)), Some((6, 1))], [None, None], [Some((5, 2)), None], [None, None], [Some((4, 3)), None], [None, None]]], [[[Some((4, 4)), None], [None, None], [Some((3, 5)), None], [None, None], [Some((2, 6)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 4)), None], [None, None], [Some((3, 5)), None], [None, None], [Some((0, 6)), Some((2, 6))], [None, None], [Some((1, 7)), Some((1, 7))]], [[Some((5, 3)), None], [None, None], [Some((4, 4)), None], [None, None], [Some((0, 6)), Some((3, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 3)), None], [None, None], [Some((0, 6)), Some((4, 4))], [None, None], [None, None], [None, None], [Some((2, 6)), None]], [[Some((6, 2)), None], [None, None], [Some((0, 6)), Some((5, 3))], [None, None], [None, None], [None, None], [Some((3, 5)), None], [None, None]], [[None, None], [Some((0, 6)), Some((6, 2))], [None, None], [None, None], [None, None], [Some((4, 4)), None], [None, None], [Some((3, 5)), None]], [[Some((0, 6)), Some((7, 1))], [None, None], [None, None], [None, None], [Some((5, 3)), None], [None, None], [Some((4, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((6, 2)), None], [None, None], [Some((5, 3)), None], [None, None], [Some((4, 4)), None]]]], [[[[Some((1, 1)), None], [None, None], [None, None], [None, None], [Some((3, 1)), None], [None, None], [Some((4, 2)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 1)), Some((0, 2))], [None, None], [Some((4, 2)), None], [None, None], [Some((5, 3)), None]], [[Some((2, 0)), Some((2, 0))], [None, None], [Some((3, 1)), Some((1, 1))], [None, None], [Some((4, 2)), Some((0, 2))], [None, None], [Some((5, 3)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 2)), Some((1, 1))], [None, None], [Some((5, 3)), Some((0, 2))], [None, None], [Some((6, 4)), None]], [[Some((3, 1)), None], [None, None], [None, None], [None, None], [Some((5, 3)), Some((1, 1))], [None, None], [Some((6, 4)), Some((0, 2))], [None, None]], [[None, None], [Some((4, 2)), None], [None, None], [None, None], [None, None], [Some((6, 4)), Some((1, 1))], [None, None], [Some((7, 5)), Some((0, 2))]], [[Some((4, 2)), None], [None, None], [Some((5, 3)), None], [None, None], [None, None], [None, None], [Some((7, 5)), Some((1, 1))], [None, None]], [[None, None], [Some((5, 3)), None], [None, None], [Some((6, 4)), None], [None, None], [None, None], [None, None], [Some((1, 1)), None]]], [[[None, None], [Some((1, 0)), Some((1, 2))], [None, None], [None, None], [None, None], [Some((3, 2)), None], [None, None], [Some((4, 3)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((3, 2)), Some((0, 3))], [None, None], [Some((4, 3)), None], [None, None]], [[None, None], [Some((2, 1)), Some((2, 1))], [None, None], [Some((3, 2)), Some((1, 2))], [None, None], [Some((4, 3)), Some((0, 3))], [None, None], [Some((5, 4)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((4, 3)), Some((1, 2))], [None, None], [Some((5, 4)), Some((0, 3))], [None, None]], [[None, None], [Some((3, 2)), Some((3, 0))], [None, None], [None, None], [None, None], [Some((5, 4)), Some((1, 2))], [None, None], [Some((6, 5)), Some((0, 3))]], [[Some((3, 2)), None], [None, None], [Some((4, 3)), Some((3, 0))], [None, None], [None, None], [None, None], [Some((6, 5)), Some((1, 2))], [None, None]], [[None, None], [Some((4, 3)), None], [None, None], [Some((5, 4)), Some((3, 0))], [None, None], [None, None], [None, None], [Some((7, 6)), Some((1, 2))]], [[Some((4, 3)), None], [None, None], [Some((5, 4)), None], [None, None], [Some((6, 5)), Some((3, 0))], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [Some((1, 1)), Some((1, 3))], [None, None], [None, None], [None, None], [Some((3, 3)), None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((3, 3)), Some((0, 4))], [None, None], [Some((4, 4)), None]], [[Some((1, 1)), Some((3, 1))], [None, None], [Some((2, 2)), Some((2, 2))], [None, None], [Some((3, 3)), Some((1, 3))], [None, None], [Some((4, 4)), Some((0, 4))], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((4, 4)), Some((1, 3))], [None, None], [Some((5, 5)), Some((0, 4))]], [[None, None], [None, None], [Some((3, 3)), Some((3, 1))], [None, None], [None, None], [None, None], [Some((5, 5)), Some((1, 3))], [None, None]], [[None, None], [Some((3, 3)), Some((4, 0))], [None, None], [Some((4, 4)), Some((3, 1))], [None, None], [None, None], [None, None], [Some((6, 6)), Some((1, 3))]], [[Some((3, 3)), None], [None, None], [Some((4, 4)), Some((4, 0))], [None, None], [Some((5, 5)), Some((3, 1))], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 4)), None], [None, None], [Some((5, 5)), Some((4, 0))], [None, None], [Some((6, 6)), Some((3, 1))], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [Some((1, 2)), Some((1, 4))], [None, None], [None, None], [None, None], [Some((3, 4)), None]], [[Some((0, 1)), Some((3, 2))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((3, 4)), Some((0, 5))], [None, None]], [[None, None], [Some((1, 2)), Some((3, 2))], [None, None], [Some((2, 3)), Some((2, 3))], [None, None], [Some((3, 4)), Some((1, 4))], [None, None], [Some((4, 5)), Some((0, 5))]], [[Some((1, 2)), Some((4, 1))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((4, 5)), Some((1, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 4)), Some((3, 2))], [None, None], [None, None], [None, None], [Some((5, 6)), Some((1, 4))]], [[None, None], [None, None], [Some((3, 4)), Some((4, 1))], [None, None], [Some((4, 5)), Some((3, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 4)), Some((5, 0))], [None, None], [Some((4, 5)), Some((4, 1))], [None, None], [Some((5, 6)), Some((3, 2))], [None, None], [None, None]], [[Some((3, 4)), None], [None, None], [Some((4, 5)), Some((5, 0))], [None, None], [Some((5, 6)), Some((4, 1))], [None, None], [Some((6, 7)), Some((3, 2))], [None, None]]], [[[Some((3, 3)), None], [None, None], [None, None], [None, None], [Some((1, 3)), Some((1, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((0, 2)), Some((3, 3))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((3, 5)), Some((0, 6))]], [[Some((0, 2)), Some((4, 2))], [None, None], [Some((1, 3)), Some((3, 3))], [None, None], [Some((2, 4)), Some((2, 4))], [None, None], [Some((3, 5)), Some((1, 5))], [None, None]], [[None, None], [Some((1, 3)), Some((4, 2))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((4, 6)), Some((1, 5))]], [[Some((1, 3)), Some((5, 1))], [None, None], [None, None], [None, None], [Some((3, 5)), Some((3, 3))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 5)), Some((4, 2))], [None, None], [Some((4, 6)), Some((3, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((3, 5)), Some((5, 1))], [None, None], [Some((4, 6)), Some((4, 2))], [None, None], [Some((5, 7)), Some((3, 3))], [None, None]], [[None, None], [Some((3, 5)), Some((6, 0))], [None, None], [Some((4, 6)), Some((5, 1))], [None, None], [Some((5, 7)), Some((4, 2))], [None, None], [Some((3, 3)), None]]], [[[None, None], [Some((3, 4)), None], [None, None], [None, None], [None, None], [Some((1, 4)), Some((1, 6))], [None, None], [None, None]], [[Some((4, 3)), None], [None, None], [Some((0, 3)), Some((3, 4))], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((0, 3)), Some((4, 3))], [None, None], [Some((1, 4)), Some((3, 4))], [None, None], [Some((2, 5)), Some((2, 5))], [None, None], [Some((3, 6)), Some((1, 6))]], [[Some((0, 3)), Some((5, 2))], [None, None], [Some((1, 4)), Some((4, 3))], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((1, 4)), Some((5, 2))], [None, None], [None, None], [None, None], [Some((3, 6)), Some((3, 4))], [None, None], [None, None]], [[Some((1, 4)), Some((6, 1))], [None, None], [None, None], [None, None], [Some((3, 6)), Some((4, 3))], [None, None], [Some((4, 7)), Some((3, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 6)), Some((5, 2))], [None, None], [Some((4, 7)), Some((4, 3))], [None, None], [Some((3, 4)), None]], [[None, None], [None, None], [Some((3, 6)), Some((6, 1))], [None, None], [Some((4, 7)), Some((5, 2))], [None, None], [Some((4, 3)), None], [None, None]]], [[[Some((4, 4)), None], [None, None], [Some((3, 5)), None], [None, None], [None, None], [None, None], [Some((1, 5)), Some((1, 7))], [None, None]], [[None, None], [Some((4, 4)), None], [None, None], [Some((0, 4)), Some((3, 5))], [None, None], [None, None], [None, None], [None, None]], [[Some((5, 3)), None], [None, None], [Some((0, 4)), Some((4, 4))], [None, None], [Some((1, 5)), Some((3, 5))], [None, None], [Some((2, 6)), Some((2, 6))], [None, None]], [[None, None], [Some((0, 4)), Some((5, 3))], [None, None], [Some((1, 5)), Some((4, 4))], [None, None], [None, None], [None, None], [None, None]], [[Some((0, 4)), Some((6, 2))], [None, None], [Some((1, 5)), Some((5, 3))], [None, None], [None, None], [None, None], [Some((3, 7)), Some((3, 5))], [None, None]], [[None, None], [Some((1, 5)), Some((6, 2))], [None, None], [None, None], [None, None], [Some((3, 7)), Some((4, 4))], [None, None], [Some((3, 5)), None]], [[Some((1, 5)), Some((7, 1))], [None, None], [None, None], [None, None], [Some((3, 7)), Some((5, 3))], [None, None], [Some((4, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 7)), Some((6, 2))], [None, None], [Some((5, 3)), None], [None, None], [Some((4, 4)), None]]], [[[None, None], [Some((4, 5)), None], [None, None], [Some((3, 6)), None], [None, None], [None, None], [None, None], [Some((1, 6)), None]], [[Some((5, 4)), None], [None, None], [Some((4, 5)), None], [None, None], [Some((0, 5)), Some((3, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 4)), None], [None, None], [Some((0, 5)), Some((4, 5))], [None, None], [Some((1, 6)), Some((3, 6))], [None, None], [Some((2, 7)), Some((2, 7))]], [[Some((6, 3)), None], [None, None], [Some((0, 5)), Some((5, 4))], [None, None], [Some((1, 6)), Some((4, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((0, 5)), Some((6, 3))], [None, None], [Some((1, 6)), Some((5, 4))], [None, None], [None, None], [None, None], [Some((3, 6)), None]], [[Some((0, 5)), Some((7, 2))], [None, None], [Some((1, 6)), Some((6, 3))], [None, None], [None, None], [None, None], [Some((4, 5)), None], [None, None]], [[None, None], [Some((1, 6)), Some((7, 2))], [None, None], [None, None], [None, None], [Some((5, 4)), None], [None, None], [Some((4, 5)), None]], [[Some((1, 6)), None], [None, None], [None, None], [None, None], [Some((6, 3)), None], [None, None], [Some((5, 4)), None], [None, None]]]], [[[[None, None], [Some((1, 2)), None], [None, None], [None, None], [None, None], [Some((4, 1)), None], [None, None], [Some((5, 2)), None]], [[Some((2, 1)), None], [None, None], [None, None], [None, None], [Some((4, 1)), Some((0, 3))], [None, None], [Some((5, 2)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 1)), Some((1, 2))], [None, None], [Some((5, 2)), Some((0, 3))], [None, None], [Some((6, 3)), None]], [[Some((3, 0)), Some((3, 0))], [None, None], [Some((4, 1)), Some((2, 1))], [None, None], [Some((5, 2)), Some((1, 2))], [None, None], [Some((6, 3)), Some((0, 3))], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 2)), Some((2, 1))], [None, None], [Some((6, 3)), Some((1, 2))], [None, None], [Some((7, 4)), Some((0, 3))]], [[Some((4, 1)), None], [None, None], [None, None], [None, None], [Some((6, 3)), Some((2, 1))], [None, None], [Some((7, 4)), Some((1, 2))], [None, None]], [[None, None], [Some((5, 2)), None], [None, None], [None, None], [None, None], [Some((7, 4)), Some((2, 1))], [None, None], [Some((1, 2)), None]], [[Some((5, 2)), None], [None, None], [Some((6, 3)), None], [None, None], [None, None], [None, None], [Some((2, 1)), None], [None, None]]], [[[Some((2, 2)), None], [None, None], [Some((2, 0)), Some((1, 3))], [None, None], [None, None], [None, None], [Some((4, 2)), None], [None, None]], [[None, None], [Some((2, 0)), Some((2, 2))], [None, None], [None, None], [None, None], [Some((4, 2)), Some((0, 4))], [None, None], [Some((5, 3)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((4, 2)), Some((1, 3))], [None, None], [Some((5, 3)), Some((0, 4))], [None, None]], [[None, None], [Some((3, 1)), Some((3, 1))], [None, None], [Some((4, 2)), Some((2, 2))], [None, None], [Some((5, 3)), Some((1, 3))], [None, None], [Some((6, 4)), Some((0, 4))]], [[None, None], [None, None], [None, None], [None, None], [Some((5, 3)), Some((2, 2))], [None, None], [Some((6, 4)), Some((1, 3))], [None, None]], [[None, None], [Some((4, 2)), Some((4, 0))], [None, None], [None, None], [None, None], [Some((6, 4)), Some((2, 2))], [None, None], [Some((7, 5)), Some((1, 3))]], [[Some((4, 2)), None], [None, None], [Some((5, 3)), Some((4, 0))], [None, None], [None, None], [None, None], [Some((7, 5)), Some((2, 2))], [None, None]], [[None, None], [Some((5, 3)), None], [None, None], [Some((6, 4)), Some((4, 0))], [None, None], [None, None], [None, None], [Some((2, 2)), None]]], [[[None, None], [Some((1, 0)), Some((2, 3))], [None, None], [Some((2, 1)), Some((1, 4))], [None, None], [None, None], [None, None], [Some((4, 3)), None]], [[None, None], [None, None], [Some((2, 1)), Some((2, 3))], [None, None], [None, None], [None, None], [Some((4, 3)), Some((0, 5))], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((4, 3)), Some((1, 4))], [None, None], [Some((5, 4)), Some((0, 5))]], [[Some((2, 1)), Some((4, 1))], [None, None], [Some((3, 2)), Some((3, 2))], [None, None], [Some((4, 3)), Some((2, 3))], [None, None], [Some((5, 4)), Some((1, 4))], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 4)), Some((2, 3))], [None, None], [Some((6, 5)), Some((1, 4))]], [[None, None], [None, None], [Some((4, 3)), Some((4, 1))], [None, None], [None, None], [None, None], [Some((6, 5)), Some((2, 3))], [None, None]], [[None, None], [Some((4, 3)), Some((5, 0))], [None, None], [Some((5, 4)), Some((4, 1))], [None, None], [None, None], [None, None], [Some((7, 6)), Some((2, 3))]], [[Some((4, 3)), None], [None, None], [Some((5, 4)), Some((5, 0))], [None, None], [Some((6, 5)), Some((4, 1))], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [Some((1, 1)), Some((2, 4))], [None, None], [Some((2, 2)), Some((1, 5))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 2)), Some((2, 4))], [None, None], [None, None], [None, None], [Some((4, 4)), Some((0, 6))]], [[Some((1, 1)), Some((4, 2))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((4, 4)), Some((1, 5))], [None, None]], [[None, None], [Some((2, 2)), Some((4, 2))], [None, None], [Some((3, 3)), Some((3, 3))], [None, None], [Some((4, 4)), Some((2, 4))], [None, None], [Some((5, 5)), Some((1, 5))]], [[Some((2, 2)), Some((5, 1))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 5)), Some((2, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 4)), Some((4, 2))], [None, None], [None, None], [None, None], [Some((6, 6)), Some((2, 4))]], [[None, None], [None, None], [Some((4, 4)), Some((5, 1))], [None, None], [Some((5, 5)), Some((4, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 4)), Some((6, 0))], [None, None], [Some((5, 5)), Some((5, 1))], [None, None], [Some((6, 6)), Some((4, 2))], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [Some((1, 2)), Some((2, 5))], [None, None], [Some((2, 3)), Some((1, 6))], [None, None], [None, None]], [[Some((0, 1)), Some((4, 3))], [None, None], [None, None], [None, None], [Some((2, 3)), Some((2, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((1, 2)), Some((4, 3))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((4, 5)), Some((1, 6))]], [[Some((1, 2)), Some((5, 2))], [None, None], [Some((2, 3)), Some((4, 3))], [None, None], [Some((3, 4)), Some((3, 4))], [None, None], [Some((4, 5)), Some((2, 5))], [None, None]], [[None, None], [Some((2, 3)), Some((5, 2))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 6)), Some((2, 5))]], [[Some((2, 3)), Some((6, 1))], [None, None], [None, None], [None, None], [Some((4, 5)), Some((4, 3))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 5)), Some((5, 2))], [None, None], [Some((5, 6)), Some((4, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((4, 5)), Some((6, 1))], [None, None], [Some((5, 6)), Some((5, 2))], [None, None], [Some((6, 7)), Some((4, 3))], [None, None]]], [[[Some((4, 4)), None], [None, None], [None, None], [None, None], [Some((1, 3)), Some((2, 6))], [None, None], [Some((2, 4)), Some((1, 7))], [None, None]], [[None, None], [Some((0, 2)), Some((4, 4))], [None, None], [None, None], [None, None], [Some((2, 4)), Some((2, 6))], [None, None], [None, None]], [[Some((0, 2)), Some((5, 3))], [None, None], [Some((1, 3)), Some((4, 4))], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((1, 3)), Some((5, 3))], [None, None], [Some((2, 4)), Some((4, 4))], [None, None], [Some((3, 5)), Some((3, 5))], [None, None], [Some((4, 6)), Some((2, 6))]], [[Some((1, 3)), Some((6, 2))], [None, None], [Some((2, 4)), Some((5, 3))], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 4)), Some((6, 2))], [None, None], [None, None], [None, None], [Some((4, 6)), Some((4, 4))], [None, None], [None, None]], [[Some((2, 4)), Some((7, 1))], [None, None], [None, None], [None, None], [Some((4, 6)), Some((5, 3))], [None, None], [Some((5, 7)), Some((4, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 6)), Some((6, 2))], [None, None], [Some((5, 7)), Some((5, 3))], [None, None], [Some((4, 4)), None]]], [[[None, None], [Some((4, 5)), None], [None, None], [None, None], [None, None], [Some((1, 4)), Some((2, 7))], [None, None], [Some((2, 5)), None]], [[Some((5, 4)), None], [None, None], [Some((0, 3)), Some((4, 5))], [None, None], [None, None], [None, None], [Some((2, 5)), Some((2, 7))], [None, None]], [[None, None], [Some((0, 3)), Some((5, 4))], [None, None], [Some((1, 4)), Some((4, 5))], [None, None], [None, None], [None, None], [None, None]], [[Some((0, 3)), Some((6, 3))], [None, None], [Some((1, 4)), Some((5, 4))], [None, None], [Some((2, 5)), Some((4, 5))], [None, None], [Some((3, 6)), Some((3, 6))], [None, None]], [[None, None], [Some((1, 4)), Some((6, 3))], [None, None], [Some((2, 5)), Some((5, 4))], [None, None], [None, None], [None, None], [None, None]], [[Some((1, 4)), Some((7, 2))], [None, None], [Some((2, 5)), Some((6, 3))], [None, None], [None, None], [None, None], [Some((4, 7)), Some((4, 5))], [None, None]], [[None, None], [Some((2, 5)), Some((7, 2))], [None, None], [None, None], [None, None], [Some((4, 7)), Some((5, 4))], [None, None], [Some((4, 5)), None]], [[Some((2, 5)), None], [None, None], [None, None], [None, None], [Some((4, 7)), Some((6, 3))], [None, None], [Some((5, 4)), None], [None, None]]], [[[Some((5, 5)), None], [None, None], [Some((4, 6)), None], [None, None], [None, None], [None, None], [Some((1, 5)), None], [None, None]], [[None, None], [Some((5, 5)), None], [None, None], [Some((0, 4)), Some((4, 6))], [None, None], [None, None], [None, None], [Some((2, 6)), None]], [[Some((6, 4)), None], [None, None], [Some((0, 4)), Some((5, 5))], [None, None], [Some((1, 5)), Some((4, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((0, 4)), Some((6, 4))], [None, None], [Some((1, 5)), Some((5, 5))], [None, None], [Some((2, 6)), Some((4, 6))], [None, None], [Some((3, 7)), Some((3, 7))]], [[Some((0, 4)), Some((7, 3))], [None, None], [Some((1, 5)), Some((6, 4))], [None, None], [Some((2, 6)), Some((5, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((1, 5)), Some((7, 3))], [None, None], [Some((2, 6)), Some((6, 4))], [None, None], [None, None], [None, None], [Some((4, 6)), None]], [[Some((1, 5)), None], [None, None], [Some((2, 6)), Some((7, 3))], [None, None], [None, None], [None, None], [Some((5, 5)), None], [None, None]], [[None, None], [Some((2, 6)), None], [None, None], [None, None], [None, None], [Some((6, 4)), None], [None, None], [Some((5, 5)), None]]]], [[[[Some((2, 2)), None], [None, None], [Some((1, 3)), None], [None, None], [None, None], [None, None], [Some((5, 1)), None], [None, None]], [[None, None], [Some((2, 2)), None], [None, None], [None, None], [None, None], [Some((5, 1)), Some((0, 4))], [None, None], [Some((6, 2)), None]], [[Some((3, 1)), None], [None, None], [None, None], [None, None], [Some((5, 1)), Some((1, 3))], [None, None], [Some((6, 2)), Some((0, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 1)), Some((2, 2))], [None, None], [Some((6, 2)), Some((1, 3))], [None, None], [Some((7, 3)), Some((0, 4))]], [[Some((4, 0)), Some((4, 0))], [None, None], [Some((5, 1)), Some((3, 1))], [None, None], [Some((6, 2)), Some((2, 2))], [None, None], [Some((7, 3)), Some((1, 3))], [None, None]], [[None, None], [None, None], [None, None], [Some((6, 2)), Some((3, 1))], [None, None], [Some((7, 3)), Some((2, 2))], [None, None], [Some((1, 3)), None]], [[Some((5, 1)), None], [None, None], [None, None], [None, None], [Some((7, 3)), Some((3, 1))], [None, None], [Some((2, 2)), None], [None, None]], [[None, None], [Some((6, 2)), None], [None, None], [None, None], [None, None], [Some((3, 1)), None], [None, None], [Some((2, 2)), None]]], [[[None, None], [Some((2, 3)), None], [None, None], [Some((3, 0)), Some((1, 4))], [None, None], [None, None], [None, None], [Some((5, 2)), None]], [[Some((3, 2)), None], [None, None], [Some((3, 0)), Some((2, 3))], [None, None], [None, None], [None, None], [Some((5, 2)), Some((0, 5))], [None, None]], [[None, None], [Some((3, 0)), Some((3, 2))], [None, None], [None, None], [None, None], [Some((5, 2)), Some((1, 4))], [None, None], [Some((6, 3)), Some((0, 5))]], [[None, None], [None, None], [None, None], [None, None], [Some((5, 2)), Some((2, 3))], [None, None], [Some((6, 3)), Some((1, 4))], [None, None]], [[None, None], [Some((4, 1)), Some((4, 1))], [None, None], [Some((5, 2)), Some((3, 2))], [None, None], [Some((6, 3)), Some((2, 3))], [None, None], [Some((7, 4)), Some((1, 4))]], [[None, None], [None, None], [None, None], [None, None], [Some((6, 3)), Some((3, 2))], [None, None], [Some((7, 4)), Some((2, 3))], [None, None]], [[None, None], [Some((5, 2)), Some((5, 0))], [None, None], [None, None], [None, None], [Some((7, 4)), Some((3, 2))], [None, None], [Some((2, 3)), None]], [[Some((5, 2)), None], [None, None], [Some((6, 3)), Some((5, 0))], [None, None], [None, None], [None, None], [Some((3, 2)), None], [None, None]]], [[[Some((3, 3)), None], [None, None], [Some((2, 0)), Some((2, 4))], [None, None], [Some((3, 1)), Some((1, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 0)), Some((3, 3))], [None, None], [Some((3, 1)), Some((2, 4))], [None, None], [None, None], [None, None], [Some((5, 3)), Some((0, 6))]], [[None, None], [None, None], [Some((3, 1)), Some((3, 3))], [None, None], [None, None], [None, None], [Some((5, 3)), Some((1, 5))], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 3)), Some((2, 4))], [None, None], [Some((6, 4)), Some((1, 5))]], [[Some((3, 1)), Some((5, 1))], [None, None], [Some((4, 2)), Some((4, 2))], [None, None], [Some((5, 3)), Some((3, 3))], [None, None], [Some((6, 4)), Some((2, 4))], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((6, 4)), Some((3, 3))], [None, None], [Some((7, 5)), Some((2, 4))]], [[None, None], [None, None], [Some((5, 3)), Some((5, 1))], [None, None], [None, None], [None, None], [Some((7, 5)), Some((3, 3))], [None, None]], [[None, None], [Some((5, 3)), Some((6, 0))], [None, None], [Some((6, 4)), Some((5, 1))], [None, None], [None, None], [None, None], [Some((3, 3)), None]]], [[[None, None], [Some((1, 0)), Some((3, 4))], [None, None], [Some((2, 1)), Some((2, 5))], [None, None], [Some((3, 2)), Some((1, 6))], [None, None], [None, None]], [[None, None], [None, None], [Some((2, 1)), Some((3, 4))], [None, None], [Some((3, 2)), Some((2, 5))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 2)), Some((3, 4))], [None, None], [None, None], [None, None], [Some((5, 4)), Some((1, 6))]], [[Some((2, 1)), Some((5, 2))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 4)), Some((2, 5))], [None, None]], [[None, None], [Some((3, 2)), Some((5, 2))], [None, None], [Some((4, 3)), Some((4, 3))], [None, None], [Some((5, 4)), Some((3, 4))], [None, None], [Some((6, 5)), Some((2, 5))]], [[Some((3, 2)), Some((6, 1))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((6, 5)), Some((3, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 4)), Some((5, 2))], [None, None], [None, None], [None, None], [Some((7, 6)), Some((3, 4))]], [[None, None], [None, None], [Some((5, 4)), Some((6, 1))], [None, None], [Some((6, 5)), Some((5, 2))], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [Some((1, 1)), Some((3, 5))], [None, None], [Some((2, 2)), Some((2, 6))], [None, None], [Some((3, 3)), Some((1, 7))], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 2)), Some((3, 5))], [None, None], [Some((3, 3)), Some((2, 6))], [None, None], [None, None]], [[Some((1, 1)), Some((5, 3))], [None, None], [None, None], [None, None], [Some((3, 3)), Some((3, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 2)), Some((5, 3))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 5)), Some((2, 6))]], [[Some((2, 2)), Some((6, 2))], [None, None], [Some((3, 3)), Some((5, 3))], [None, None], [Some((4, 4)), Some((4, 4))], [None, None], [Some((5, 5)), Some((3, 5))], [None, None]], [[None, None], [Some((3, 3)), Some((6, 2))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((6, 6)), Some((3, 5))]], [[Some((3, 3)), Some((7, 1))], [None, None], [None, None], [None, None], [Some((5, 5)), Some((5, 3))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 5)), Some((6, 2))], [None, None], [Some((6, 6)), Some((5, 3))], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [Some((1, 2)), Some((3, 6))], [None, None], [Some((2, 3)), Some((2, 7))], [None, None], [Some((3, 4)), None]], [[Some((0, 1)), Some((5, 4))], [None, None], [None, None], [None, None], [Some((2, 3)), Some((3, 6))], [None, None], [Some((3, 4)), Some((2, 7))], [None, None]], [[None, None], [Some((1, 2)), Some((5, 4))], [None, None], [None, None], [None, None], [Some((3, 4)), Some((3, 6))], [None, None], [None, None]], [[Some((1, 2)), Some((6, 3))], [None, None], [Some((2, 3)), Some((5, 4))], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 3)), Some((6, 3))], [None, None], [Some((3, 4)), Some((5, 4))], [None, None], [Some((4, 5)), Some((4, 5))], [None, None], [Some((5, 6)), Some((3, 6))]], [[Some((2, 3)), Some((7, 2))], [None, None], [Some((3, 4)), Some((6, 3))], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 4)), Some((7, 2))], [None, None], [None, None], [None, None], [Some((5, 6)), Some((5, 4))], [None, None], [None, None]], [[Some((3, 4)), None], [None, None], [None, None], [None, None], [Some((5, 6)), Some((6, 3))], [None, None], [Some((6, 7)), Some((5, 4))], [None, None]]], [[[Some((5, 5)), None], [None, None], [None, None], [None, None], [Some((1, 3)), Some((3, 7))], [None, None], [Some((2, 4)), None], [None, None]], [[None, None], [Some((0, 2)), Some((5, 5))], [None, None], [None, None], [None, None], [Some((2, 4)), Some((3, 7))], [None, None], [Some((3, 5)), None]], [[Some((0, 2)), Some((6, 4))], [None, None], [Some((1, 3)), Some((5, 5))], [None, None], [None, None], [None, None], [Some((3, 5)), Some((3, 7))], [None, None]], [[None, None], [Some((1, 3)), Some((6, 4))], [None, None], [Some((2, 4)), Some((5, 5))], [None, None], [None, None], [None, None], [None, None]], [[Some((1, 3)), Some((7, 3))], [None, None], [Some((2, 4)), Some((6, 4))], [None, None], [Some((3, 5)), Some((5, 5))], [None, None], [Some((4, 6)), Some((4, 6))], [None, None]], [[None, None], [Some((2, 4)), Some((7, 3))], [None, None], [Some((3, 5)), Some((6, 4))], [None, None], [None, None], [None, None], [None, None]], [[Some((2, 4)), None], [None, None], [Some((3, 5)), Some((7, 3))], [None, None], [None, None], [None, None], [Some((5, 7)), Some((5, 5))], [None, None]], [[None, None], [Some((3, 5)), None], [None, None], [None, None], [None, None], [Some((5, 7)), Some((6, 4))], [None, None], [Some((5, 5)), None]]], [[[None, None], [Some((5, 6)), None], [None, None], [None, None], [None, None], [Some((1, 4)), None], [None, None], [Some((2, 5)), None]], [[Some((6, 5)), None], [None, None], [Some((0, 3)), Some((5, 6))], [None, None], [None, None], [None, None], [Some((2, 5)), None], [None, None]], [[None, None], [Some((0, 3)), Some((6, 5))], [None, None], [Some((1, 4)), Some((5, 6))], [None, None], [None, None], [None, None], [Some((3, 6)), None]], [[Some((0, 3)), Some((7, 4))], [None, None], [Some((1, 4)), Some((6, 5))], [None, None], [Some((2, 5)), Some((5, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((1, 4)), Some((7, 4))], [None, None], [Some((2, 5)), Some((6, 5))], [None, None], [Some((3, 6)), Some((5, 6))], [None, None], [Some((4, 7)), Some((4, 7))]], [[Some((1, 4)), None], [None, None], [Some((2, 5)), Some((7, 4))], [None, None], [Some((3, 6)), Some((6, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 5)), None], [None, None], [Some((3, 6)), Some((7, 4))], [None, None], [None, None], [None, None], [Some((5, 6)), None]], [[Some((2, 5)), None], [None, None], [Some((3, 6)), None], [None, None], [None, None], [None, None], [Some((6, 5)), None], [None, None]]]], [[[[None, None], [Some((2, 3)), None], [None, None], [Some((1, 4)), None], [None, None], [None, None], [None, None], [Some((6, 1)), None]], [[Some((3, 2)), None], [None, None], [Some((2, 3)), None], [None, None], [None, None], [None, None], [Some((6, 1)), Some((0, 5))], [None, None]], [[None, None], [Some((3, 2)), None], [None, None], [None, None], [None, None], [Some((6, 1)), Some((1, 4))], [None, None], [Some((7, 2)), Some((0, 5))]], [[Some((4, 1)), None], [None, None], [None, None], [None, None], [Some((6, 1)), Some((2, 3))], [None, None], [Some((7, 2)), Some((1, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((6, 1)), Some((3, 2))], [None, None], [Some((7, 2)), Some((2, 3))], [None, None], [Some((1, 4)), None]], [[Some((5, 0)), Some((5, 0))], [None, None], [Some((6, 1)), Some((4, 1))], [None, None], [Some((7, 2)), Some((3, 2))], [None, None], [Some((2, 3)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((7, 2)), Some((4, 1))], [None, None], [Some((3, 2)), None], [None, None], [Some((2, 3)), None]], [[Some((6, 1)), None], [None, None], [None, None], [None, None], [Some((4, 1)), None], [None, None], [Some((3, 2)), None], [None, None]]], [[[Some((3, 3)), None], [None, None], [Some((2, 4)), None], [None, None], [Some((4, 0)), Some((1, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 3)), None], [None, None], [Some((4, 0)), Some((2, 4))], [None, None], [None, None], [None, None], [Some((6, 2)), Some((0, 6))]], [[Some((4, 2)), None], [None, None], [Some((4, 0)), Some((3, 3))], [None, None], [None, None], [None, None], [Some((6, 2)), Some((1, 5))], [None, None]], [[None, None], [Some((4, 0)), Some((4, 2))], [None, None], [None, None], [None, None], [Some((6, 2)), Some((2, 4))], [None, None], [Some((7, 3)), Some((1, 5))]], [[None, None], [None, None], [None, None], [None, None], [Some((6, 2)), Some((3, 3))], [None, None], [Some((7, 3)), Some((2, 4))], [None, None]], [[None, None], [Some((5, 1)), Some((5, 1))], [None, None], [Some((6, 2)), Some((4, 2))], [None, None], [Some((7, 3)), Some((3, 3))], [None, None], [Some((2, 4)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((7, 3)), Some((4, 2))], [None, None], [Some((3, 3)), None], [None, None]], [[None, None], [Some((6, 2)), Some((6, 0))], [None, None], [None, None], [None, None], [Some((4, 2)), None], [None, None], [Some((3, 3)), None]]], [[[None, None], [Some((3, 4)), None], [None, None], [Some((3, 0)), Some((2, 5))], [None, None], [Some((4, 1)), Some((1, 6))], [None, None], [None, None]], [[Some((4, 3)), None], [None, None], [Some((3, 0)), Some((3, 4))], [None, None], [Some((4, 1)), Some((2, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 0)), Some((4, 3))], [None, None], [Some((4, 1)), Some((3, 4))], [None, None], [None, None], [None, None], [Some((6, 3)), Some((1, 6))]], [[None, None], [None, None], [Some((4, 1)), Some((4, 3))], [None, None], [None, None], [None, None], [Some((6, 3)), Some((2, 5))], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((6, 3)), Some((3, 4))], [None, None], [Some((7, 4)), Some((2, 5))]], [[Some((4, 1)), Some((6, 1))], [None, None], [Some((5, 2)), Some((5, 2))], [None, None], [Some((6, 3)), Some((4, 3))], [None, None], [Some((7, 4)), Some((3, 4))], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((7, 4)), Some((4, 3))], [None, None], [Some((3, 4)), None]], [[None, None], [None, None], [Some((6, 3)), Some((6, 1))], [None, None], [None, None], [None, None], [Some((4, 3)), None], [None, None]]], [[[Some((4, 4)), None], [None, None], [Some((2, 0)), Some((3, 5))], [None, None], [Some((3, 1)), Some((2, 6))], [None, None], [Some((4, 2)), Some((1, 7))], [None, None]], [[None, None], [Some((2, 0)), Some((4, 4))], [None, None], [Some((3, 1)), Some((3, 5))], [None, None], [Some((4, 2)), Some((2, 6))], [None, None], [None, None]], [[None, None], [None, None], [Some((3, 1)), Some((4, 4))], [None, None], [Some((4, 2)), Some((3, 5))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 2)), Some((4, 4))], [None, None], [None, None], [None, None], [Some((6, 4)), Some((2, 6))]], [[Some((3, 1)), Some((6, 2))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((6, 4)), Some((3, 5))], [None, None]], [[None, None], [Some((4, 2)), Some((6, 2))], [None, None], [Some((5, 3)), Some((5, 3))], [None, None], [Some((6, 4)), Some((4, 4))], [None, None], [Some((7, 5)), Some((3, 5))]], [[Some((4, 2)), Some((7, 1))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((7, 5)), Some((4, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((6, 4)), Some((6, 2))], [None, None], [None, None], [None, None], [Some((4, 4)), None]]], [[[None, None], [Some((1, 0)), Some((4, 5))], [None, None], [Some((2, 1)), Some((3, 6))], [None, None], [Some((3, 2)), Some((2, 7))], [None, None], [Some((4, 3)), None]], [[None, None], [None, None], [Some((2, 1)), Some((4, 5))], [None, None], [Some((3, 2)), Some((3, 6))], [None, None], [Some((4, 3)), Some((2, 7))], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 2)), Some((4, 5))], [None, None], [Some((4, 3)), Some((3, 6))], [None, None], [None, None]], [[Some((2, 1)), Some((6, 3))], [None, None], [None, None], [None, None], [Some((4, 3)), Some((4, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 2)), Some((6, 3))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((6, 5)), Some((3, 6))]], [[Some((3, 2)), Some((7, 2))], [None, None], [Some((4, 3)), Some((6, 3))], [None, None], [Some((5, 4)), Some((5, 4))], [None, None], [Some((6, 5)), Some((4, 5))], [None, None]], [[None, None], [Some((4, 3)), Some((7, 2))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((7, 6)), Some((4, 5))]], [[Some((4, 3)), None], [None, None], [None, None], [None, None], [Some((6, 5)), Some((6, 3))], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [Some((1, 1)), Some((4, 6))], [None, None], [Some((2, 2)), Some((3, 7))], [None, None], [Some((3, 3)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 2)), Some((4, 6))], [None, None], [Some((3, 3)), Some((3, 7))], [None, None], [Some((4, 4)), None]], [[Some((1, 1)), Some((6, 4))], [None, None], [None, None], [None, None], [Some((3, 3)), Some((4, 6))], [None, None], [Some((4, 4)), Some((3, 7))], [None, None]], [[None, None], [Some((2, 2)), Some((6, 4))], [None, None], [None, None], [None, None], [Some((4, 4)), Some((4, 6))], [None, None], [None, None]], [[Some((2, 2)), Some((7, 3))], [None, None], [Some((3, 3)), Some((6, 4))], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 3)), Some((7, 3))], [None, None], [Some((4, 4)), Some((6, 4))], [None, None], [Some((5, 5)), Some((5, 5))], [None, None], [Some((6, 6)), Some((4, 6))]], [[Some((3, 3)), None], [None, None], [Some((4, 4)), Some((7, 3))], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 4)), None], [None, None], [None, None], [None, None], [Some((6, 6)), Some((6, 4))], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [Some((1, 2)), Some((4, 7))], [None, None], [Some((2, 3)), None], [None, None], [Some((3, 4)), None]], [[Some((0, 1)), Some((6, 5))], [None, None], [None, None], [None, None], [Some((2, 3)), Some((4, 7))], [None, None], [Some((3, 4)), None], [None, None]], [[None, None], [Some((1, 2)), Some((6, 5))], [None, None], [None, None], [None, None], [Some((3, 4)), Some((4, 7))], [None, None], [Some((4, 5)), None]], [[Some((1, 2)), Some((7, 4))], [None, None], [Some((2, 3)), Some((6, 5))], [None, None], [None, None], [None, None], [Some((4, 5)), Some((4, 7))], [None, None]], [[None, None], [Some((2, 3)), Some((7, 4))], [None, None], [Some((3, 4)), Some((6, 5))], [None, None], [None, None], [None, None], [None, None]], [[Some((2, 3)), None], [None, None], [Some((3, 4)), Some((7, 4))], [None, None], [Some((4, 5)), Some((6, 5))], [None, None], [Some((5, 6)), Some((5, 6))], [None, None]], [[None, None], [Some((3, 4)), None], [None, None], [Some((4, 5)), Some((7, 4))], [None, None], [None, None], [None, None], [None, None]], [[Some((3, 4)), None], [None, None], [Some((4, 5)), None], [None, None], [None, None], [None, None], [Some((6, 7)), Some((6, 5))], [None, None]]], [[[Some((6, 6)), None], [None, None], [None, None], [None, None], [Some((1, 3)), None], [None, None], [Some((2, 4)), None], [None, None]], [[None, None], [Some((0, 2)), Some((6, 6))], [None, None], [None, None], [None, None], [Some((2, 4)), None], [None, None], [Some((3, 5)), None]], [[Some((0, 2)), Some((7, 5))], [None, None], [Some((1, 3)), Some((6, 6))], [None, None], [None, None], [None, None], [Some((3, 5)), None], [None, None]], [[None, None], [Some((1, 3)), Some((7, 5))], [None, None], [Some((2, 4)), Some((6, 6))], [None, None], [None, None], [None, None], [Some((4, 6)), None]], [[Some((1, 3)), None], [None, None], [Some((2, 4)), Some((7, 5))], [None, None], [Some((3, 5)), Some((6, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 4)), None], [None, None], [Some((3, 5)), Some((7, 5))], [None, None], [Some((4, 6)), Some((6, 6))], [None, None], [Some((5, 7)), Some((5, 7))]], [[Some((2, 4)), None], [None, None], [Some((3, 5)), None], [None, None], [Some((4, 6)), Some((7, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 5)), None], [None, None], [Some((4, 6)), None], [None, None], [None, None], [None, None], [Some((6, 6)), None]]]], [[[[Some((3, 3)), None], [None, None], [Some((2, 4)), None], [None, None], [Some((1, 5)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 3)), None], [None, None], [Some((2, 4)), None], [None, None], [None, None], [None, None], [Some((7, 1)), Some((0, 6))]], [[Some((4, 2)), None], [None, None], [Some((3, 3)), None], [None, None], [None, None], [None, None], [Some((7, 1)), Some((1, 5))], [None, None]], [[None, None], [Some((4, 2)), None], [None, None], [None, None], [None, None], [Some((7, 1)), Some((2, 4))], [None, None], [Some((1, 5)), None]], [[Some((5, 1)), None], [None, None], [None, None], [None, None], [Some((7, 1)), Some((3, 3))], [None, None], [Some((2, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((7, 1)), Some((4, 2))], [None, None], [Some((3, 3)), None], [None, None], [Some((2, 4)), None]], [[Some((6, 0)), Some((6, 0))], [None, None], [Some((7, 1)), Some((5, 1))], [None, None], [Some((4, 2)), None], [None, None], [Some((3, 3)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 1)), None], [None, None], [Some((4, 2)), None], [None, None], [Some((3, 3)), None]]], [[[None, None], [Some((3, 4)), None], [None, None], [Some((2, 5)), None], [None, None], [Some((5, 0)), Some((1, 6))], [None, None], [None, None]], [[Some((4, 3)), None], [None, None], [Some((3, 4)), None], [None, None], [Some((5, 0)), Some((2, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 3)), None], [None, None], [Some((5, 0)), Some((3, 4))], [None, None], [None, None], [None, None], [Some((7, 2)), Some((1, 6))]], [[Some((5, 2)), None], [None, None], [Some((5, 0)), Some((4, 3))], [None, None], [None, None], [None, None], [Some((7, 2)), Some((2, 5))], [None, None]], [[None, None], [Some((5, 0)), Some((5, 2))], [None, None], [None, None], [None, None], [Some((7, 2)), Some((3, 4))], [None, None], [Some((2, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((7, 2)), Some((4, 3))], [None, None], [Some((3, 4)), None], [None, None]], [[None, None], [Some((6, 1)), Some((6, 1))], [None, None], [Some((7, 2)), Some((5, 2))], [None, None], [Some((4, 3)), None], [None, None], [Some((3, 4)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((5, 2)), None], [None, None], [Some((4, 3)), None], [None, None]]], [[[Some((4, 4)), None], [None, None], [Some((3, 5)), None], [None, None], [Some((4, 0)), Some((2, 6))], [None, None], [Some((5, 1)), Some((1, 7))], [None, None]], [[None, None], [Some((4, 4)), None], [None, None], [Some((4, 0)), Some((3, 5))], [None, None], [Some((5, 1)), Some((2, 6))], [None, None], [None, None]], [[Some((5, 3)), None], [None, None], [Some((4, 0)), Some((4, 4))], [None, None], [Some((5, 1)), Some((3, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 0)), Some((5, 3))], [None, None], [Some((5, 1)), Some((4, 4))], [None, None], [None, None], [None, None], [Some((7, 3)), Some((2, 6))]], [[None, None], [None, None], [Some((5, 1)), Some((5, 3))], [None, None], [None, None], [None, None], [Some((7, 3)), Some((3, 5))], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((7, 3)), Some((4, 4))], [None, None], [Some((3, 5)), None]], [[Some((5, 1)), Some((7, 1))], [None, None], [Some((6, 2)), Some((6, 2))], [None, None], [Some((7, 3)), Some((5, 3))], [None, None], [Some((4, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 3)), None], [None, None], [Some((4, 4)), None]]], [[[None, None], [Some((4, 5)), None], [None, None], [Some((3, 0)), Some((3, 6))], [None, None], [Some((4, 1)), Some((2, 7))], [None, None], [Some((5, 2)), None]], [[Some((5, 4)), None], [None, None], [Some((3, 0)), Some((4, 5))], [None, None], [Some((4, 1)), Some((3, 6))], [None, None], [Some((5, 2)), Some((2, 7))], [None, None]], [[None, None], [Some((3, 0)), Some((5, 4))], [None, None], [Some((4, 1)), Some((4, 5))], [None, None], [Some((5, 2)), Some((3, 6))], [None, None], [None, None]], [[None, None], [None, None], [Some((4, 1)), Some((5, 4))], [None, None], [Some((5, 2)), Some((4, 5))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 2)), Some((5, 4))], [None, None], [None, None], [None, None], [Some((7, 4)), Some((3, 6))]], [[Some((4, 1)), Some((7, 2))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((7, 4)), Some((4, 5))], [None, None]], [[None, None], [Some((5, 2)), Some((7, 2))], [None, None], [Some((6, 3)), Some((6, 3))], [None, None], [Some((7, 4)), Some((5, 4))], [None, None], [Some((4, 5)), None]], [[Some((5, 2)), None], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 4)), None], [None, None]]], [[[Some((5, 5)), None], [None, None], [Some((2, 0)), Some((4, 6))], [None, None], [Some((3, 1)), Some((3, 7))], [None, None], [Some((4, 2)), None], [None, None]], [[None, None], [Some((2, 0)), Some((5, 5))], [None, None], [Some((3, 1)), Some((4, 6))], [None, None], [Some((4, 2)), Some((3, 7))], [None, None], [Some((5, 3)), None]], [[None, None], [None, None], [Some((3, 1)), Some((5, 5))], [None, None], [Some((4, 2)), Some((4, 6))], [None, None], [Some((5, 3)), Some((3, 7))], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 2)), Some((5, 5))], [None, None], [Some((5, 3)), Some((4, 6))], [None, None], [None, None]], [[Some((3, 1)), Some((7, 3))], [None, None], [None, None], [None, None], [Some((5, 3)), Some((5, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 2)), Some((7, 3))], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((7, 5)), Some((4, 6))]], [[Some((4, 2)), None], [None, None], [Some((5, 3)), Some((7, 3))], [None, None], [Some((6, 4)), Some((6, 4))], [None, None], [Some((7, 5)), Some((5, 5))], [None, None]], [[None, None], [Some((5, 3)), None], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 5)), None]]], [[[None, None], [Some((1, 0)), Some((5, 6))], [None, None], [Some((2, 1)), Some((4, 7))], [None, None], [Some((3, 2)), None], [None, None], [Some((4, 3)), None]], [[None, None], [None, None], [Some((2, 1)), Some((5, 6))], [None, None], [Some((3, 2)), Some((4, 7))], [None, None], [Some((4, 3)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 2)), Some((5, 6))], [None, None], [Some((4, 3)), Some((4, 7))], [None, None], [Some((5, 4)), None]], [[Some((2, 1)), Some((7, 4))], [None, None], [None, None], [None, None], [Some((4, 3)), Some((5, 6))], [None, None], [Some((5, 4)), Some((4, 7))], [None, None]], [[None, None], [Some((3, 2)), Some((7, 4))], [None, None], [None, None], [None, None], [Some((5, 4)), Some((5, 6))], [None, None], [None, None]], [[Some((3, 2)), None], [None, None], [Some((4, 3)), Some((7, 4))], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 3)), None], [None, None], [Some((5, 4)), Some((7, 4))], [None, None], [Some((6, 5)), Some((6, 5))], [None, None], [Some((7, 6)), Some((5, 6))]], [[Some((4, 3)), None], [None, None], [Some((5, 4)), None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [Some((1, 1)), Some((5, 7))], [None, None], [Some((2, 2)), None], [None, None], [Some((3, 3)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 2)), Some((5, 7))], [None, None], [Some((3, 3)), None], [None, None], [Some((4, 4)), None]], [[Some((1, 1)), Some((7, 5))], [None, None], [None, None], [None, None], [Some((3, 3)), Some((5, 7))], [None, None], [Some((4, 4)), None], [None, None]], [[None, None], [Some((2, 2)), Some((7, 5))], [None, None], [None, None], [None, None], [Some((4, 4)), Some((5, 7))], [None, None], [Some((5, 5)), None]], [[Some((2, 2)), None], [None, None], [Some((3, 3)), Some((7, 5))], [None, None], [None, None], [None, None], [Some((5, 5)), Some((5, 7))], [None, None]], [[None, None], [Some((3, 3)), None], [None, None], [Some((4, 4)), Some((7, 5))], [None, None], [None, None], [None, None], [None, None]], [[Some((3, 3)), None], [None, None], [Some((4, 4)), None], [None, None], [Some((5, 5)), Some((7, 5))], [None, None], [Some((6, 6)), Some((6, 6))], [None, None]], [[None, None], [Some((4, 4)), None], [None, None], [Some((5, 5)), None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [Some((1, 2)), None], [None, None], [Some((2, 3)), None], [None, None], [Some((3, 4)), None]], [[Some((0, 1)), Some((7, 6))], [None, None], [None, None], [None, None], [Some((2, 3)), None], [None, None], [Some((3, 4)), None], [None, None]], [[None, None], [Some((1, 2)), Some((7, 6))], [None, None], [None, None], [None, None], [Some((3, 4)), None], [None, None], [Some((4, 5)), None]], [[Some((1, 2)), None], [None, None], [Some((2, 3)), Some((7, 6))], [None, None], [None, None], [None, None], [Some((4, 5)), None], [None, None]], [[None, None], [Some((2, 3)), None], [None, None], [Some((3, 4)), Some((7, 6))], [None, None], [None, None], [None, None], [Some((5, 6)), None]], [[Some((2, 3)), None], [None, None], [Some((3, 4)), None], [None, None], [Some((4, 5)), Some((7, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 4)), None], [None, None], [Some((4, 5)), None], [None, None], [Some((5, 6)), Some((7, 6))], [None, None], [Some((6, 7)), Some((6, 7))]], [[Some((3, 4)), None], [None, None], [Some((4, 5)), None], [None, None], [Some((5, 6)), None], [None, None], [None, None], [None, None]]]], [[[[None, None], [Some((3, 4)), None], [None, None], [Some((2, 5)), None], [None, None], [Some((1, 6)), None], [None, None], [None, None]], [[Some((4, 3)), None], [None, None], [Some((3, 4)), None], [None, None], [Some((2, 5)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 3)), None], [None, None], [Some((3, 4)), None], [None, None], [None, None], [None, None], [Some((1, 6)), None]], [[Some((5, 2)), None], [None, None], [Some((4, 3)), None], [None, None], [None, None], [None, None], [Some((2, 5)), None], [None, None]], [[None, None], [Some((5, 2)), None], [None, None], [None, None], [None, None], [Some((3, 4)), None], [None, None], [Some((2, 5)), None]], [[Some((6, 1)), None], [None, None], [None, None], [None, None], [Some((4, 3)), None], [None, None], [Some((3, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 2)), None], [None, None], [Some((4, 3)), None], [None, None], [Some((3, 4)), None]], [[Some((7, 0)), Some((7, 0))], [None, None], [Some((6, 1)), None], [None, None], [Some((5, 2)), None], [None, None], [Some((4, 3)), None], [None, None]]], [[[Some((4, 4)), None], [None, None], [Some((3, 5)), None], [None, None], [Some((2, 6)), None], [None, None], [Some((6, 0)), Some((1, 7))], [None, None]], [[None, None], [Some((4, 4)), None], [None, None], [Some((3, 5)), None], [None, None], [Some((6, 0)), Some((2, 6))], [None, None], [None, None]], [[Some((5, 3)), None], [None, None], [Some((4, 4)), None], [None, None], [Some((6, 0)), Some((3, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 3)), None], [None, None], [Some((6, 0)), Some((4, 4))], [None, None], [None, None], [None, None], [Some((2, 6)), None]], [[Some((6, 2)), None], [None, None], [Some((6, 0)), Some((5, 3))], [None, None], [None, None], [None, None], [Some((3, 5)), None], [None, None]], [[None, None], [Some((6, 0)), Some((6, 2))], [None, None], [None, None], [None, None], [Some((4, 4)), None], [None, None], [Some((3, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((5, 3)), None], [None, None], [Some((4, 4)), None], [None, None]], [[None, None], [Some((7, 1)), Some((7, 1))], [None, None], [Some((6, 2)), None], [None, None], [Some((5, 3)), None], [None, None], [Some((4, 4)), None]]], [[[None, None], [Some((4, 5)), None], [None, None], [Some((3, 6)), None], [None, None], [Some((5, 0)), Some((2, 7))], [None, None], [Some((6, 1)), None]], [[Some((5, 4)), None], [None, None], [Some((4, 5)), None], [None, None], [Some((5, 0)), Some((3, 6))], [None, None], [Some((6, 1)), Some((2, 7))], [None, None]], [[None, None], [Some((5, 4)), None], [None, None], [Some((5, 0)), Some((4, 5))], [None, None], [Some((6, 1)), Some((3, 6))], [None, None], [None, None]], [[Some((6, 3)), None], [None, None], [Some((5, 0)), Some((5, 4))], [None, None], [Some((6, 1)), Some((4, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 0)), Some((6, 3))], [None, None], [Some((6, 1)), Some((5, 4))], [None, None], [None, None], [None, None], [Some((3, 6)), None]], [[None, None], [None, None], [Some((6, 1)), Some((6, 3))], [None, None], [None, None], [None, None], [Some((4, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 4)), None], [None, None], [Some((4, 5)), None]], [[Some((6, 1)), None], [None, None], [Some((7, 2)), Some((7, 2))], [None, None], [Some((6, 3)), None], [None, None], [Some((5, 4)), None], [None, None]]], [[[Some((5, 5)), None], [None, None], [Some((4, 6)), None], [None, None], [Some((4, 0)), Some((3, 7))], [None, None], [Some((5, 1)), None], [None, None]], [[None, None], [Some((5, 5)), None], [None, None], [Some((4, 0)), Some((4, 6))], [None, None], [Some((5, 1)), Some((3, 7))], [None, None], [Some((6, 2)), None]], [[Some((6, 4)), None], [None, None], [Some((4, 0)), Some((5, 5))], [None, None], [Some((5, 1)), Some((4, 6))], [None, None], [Some((6, 2)), Some((3, 7))], [None, None]], [[None, None], [Some((4, 0)), Some((6, 4))], [None, None], [Some((5, 1)), Some((5, 5))], [None, None], [Some((6, 2)), Some((4, 6))], [None, None], [None, None]], [[None, None], [None, None], [Some((5, 1)), Some((6, 4))], [None, None], [Some((6, 2)), Some((5, 5))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((6, 2)), Some((6, 4))], [None, None], [None, None], [None, None], [Some((4, 6)), None]], [[Some((5, 1)), None], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 5)), None], [None, None]], [[None, None], [Some((6, 2)), None], [None, None], [Some((7, 3)), Some((7, 3))], [None, None], [Some((6, 4)), None], [None, None], [Some((5, 5)), None]]], [[[None, None], [Some((5, 6)), None], [None, None], [Some((3, 0)), Some((4, 7))], [None, None], [Some((4, 1)), None], [None, None], [Some((5, 2)), None]], [[Some((6, 5)), None], [None, None], [Some((3, 0)), Some((5, 6))], [None, None], [Some((4, 1)), Some((4, 7))], [None, None], [Some((5, 2)), None], [None, None]], [[None, None], [Some((3, 0)), Some((6, 5))], [None, None], [Some((4, 1)), Some((5, 6))], [None, None], [Some((5, 2)), Some((4, 7))], [None, None], [Some((6, 3)), None]], [[None, None], [None, None], [Some((4, 1)), Some((6, 5))], [None, None], [Some((5, 2)), Some((5, 6))], [None, None], [Some((6, 3)), Some((4, 7))], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 2)), Some((6, 5))], [None, None], [Some((6, 3)), Some((5, 6))], [None, None], [None, None]], [[Some((4, 1)), None], [None, None], [None, None], [None, None], [Some((6, 3)), Some((6, 5))], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 2)), None], [None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 6)), None]], [[Some((5, 2)), None], [None, None], [Some((6, 3)), None], [None, None], [Some((7, 4)), Some((7, 4))], [None, None], [Some((6, 5)), None], [None, None]]], [[[Some((6, 6)), None], [None, None], [Some((2, 0)), Some((5, 7))], [None, None], [Some((3, 1)), None], [None, None], [Some((4, 2)), None], [None, None]], [[None, None], [Some((2, 0)), Some((6, 6))], [None, None], [Some((3, 1)), Some((5, 7))], [None, None], [Some((4, 2)), None], [None, None], [Some((5, 3)), None]], [[None, None], [None, None], [Some((3, 1)), Some((6, 6))], [None, None], [Some((4, 2)), Some((5, 7))], [None, None], [Some((5, 3)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 2)), Some((6, 6))], [None, None], [Some((5, 3)), Some((5, 7))], [None, None], [Some((6, 4)), None]], [[Some((3, 1)), None], [None, None], [None, None], [None, None], [Some((5, 3)), Some((6, 6))], [None, None], [Some((6, 4)), Some((5, 7))], [None, None]], [[None, None], [Some((4, 2)), None], [None, None], [None, None], [None, None], [Some((6, 4)), Some((6, 6))], [None, None], [None, None]], [[Some((4, 2)), None], [None, None], [Some((5, 3)), None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 3)), None], [None, None], [Some((6, 4)), None], [None, None], [Some((7, 5)), Some((7, 5))], [None, None], [Some((6, 6)), None]]], [[[None, None], [Some((1, 0)), Some((6, 7))], [None, None], [Some((2, 1)), None], [None, None], [Some((3, 2)), None], [None, None], [Some((4, 3)), None]], [[None, None], [None, None], [Some((2, 1)), Some((6, 7))], [None, None], [Some((3, 2)), None], [None, None], [Some((4, 3)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 2)), Some((6, 7))], [None, None], [Some((4, 3)), None], [None, None], [Some((5, 4)), None]], [[Some((2, 1)), None], [None, None], [None, None], [None, None], [Some((4, 3)), Some((6, 7))], [None, None], [Some((5, 4)), None], [None, None]], [[None, None], [Some((3, 2)), None], [None, None], [None, None], [None, None], [Some((5, 4)), Some((6, 7))], [None, None], [Some((6, 5)), None]], [[Some((3, 2)), None], [None, None], [Some((4, 3)), None], [None, None], [None, None], [None, None], [Some((6, 5)), Some((6, 7))], [None, None]], [[None, None], [Some((4, 3)), None], [None, None], [Some((5, 4)), None], [None, None], [None, None], [None, None], [None, None]], [[Some((4, 3)), None], [None, None], [Some((5, 4)), None], [None, None], [Some((6, 5)), None], [None, None], [Some((7, 6)), Some((7, 6))], [None, None]]], [[[None, None], [None, None], [Some((1, 1)), None], [None, None], [Some((2, 2)), None], [None, None], [Some((3, 3)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 2)), None], [None, None], [Some((3, 3)), None], [None, None], [Some((4, 4)), None]], [[Some((1, 1)), None], [None, None], [None, None], [None, None], [Some((3, 3)), None], [None, None], [Some((4, 4)), None], [None, None]], [[None, None], [Some((2, 2)), None], [None, None], [None, None], [None, None], [Some((4, 4)), None], [None, None], [Some((5, 5)), None]], [[Some((2, 2)), None], [None, None], [Some((3, 3)), None], [None, None], [None, None], [None, None], [Some((5, 5)), None], [None, None]], [[None, None], [Some((3, 3)), None], [None, None], [Some((4, 4)), None], [None, None], [None, None], [None, None], [Some((6, 6)), None]], [[Some((3, 3)), None], [None, None], [Some((4, 4)), None], [None, None], [Some((5, 5)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 4)), None], [None, None], [Some((5, 5)), None], [None, None], [Some((6, 6)), None], [None, None], [Some((7, 7)), Some((7, 7))]]]]];
pub static PRECOMPUTED_KNIGHT_CHECKS: [[[[[Option<(usize, usize)>; 2]; 8]; 8]; 8]; 8] = [[[[[None, None], [None, None], [Some((2, 1)), None], [None, None], [Some((1, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 1)), None], [None, None], [None, None], [None, None], [None, None]], [[Some((1, 2)), None], [None, None], [None, None], [None, None], [Some((1, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((1, 2)), None], [None, None], [Some((1, 2)), Some((2, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((2, 1)), None], [None, None], [Some((2, 1)), None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [Some((2, 2)), None], [None, None], [Some((1, 3)), None], [None, None], [None, None]], [[Some((2, 2)), None], [None, None], [Some((2, 0)), None], [None, None], [Some((2, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((1, 3)), None], [None, None], [None, None], [None, None], [Some((1, 3)), None], [None, None], [None, None]], [[Some((2, 2)), None], [None, None], [Some((1, 3)), Some((2, 0))], [None, None], [Some((1, 3)), Some((2, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 0)), Some((2, 2))], [None, None], [Some((2, 2)), None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[Some((2, 1)), None], [None, None], [None, None], [None, None], [Some((2, 3)), None], [None, None], [Some((1, 4)), None], [None, None]], [[None, None], [Some((2, 3)), None], [None, None], [Some((2, 1)), None], [None, None], [Some((2, 3)), None], [None, None], [None, None]], [[None, None], [None, None], [Some((1, 0)), Some((1, 4))], [None, None], [None, None], [None, None], [Some((1, 4)), None], [None, None]], [[None, None], [Some((1, 0)), Some((2, 3))], [None, None], [Some((1, 4)), Some((2, 1))], [None, None], [Some((1, 4)), Some((2, 3))], [None, None], [None, None]], [[Some((2, 1)), None], [None, None], [Some((2, 1)), Some((2, 3))], [None, None], [Some((2, 3)), None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [Some((2, 2)), None], [None, None], [None, None], [None, None], [Some((2, 4)), None], [None, None], [Some((1, 5)), None]], [[Some((2, 2)), None], [None, None], [Some((2, 4)), None], [None, None], [Some((2, 2)), None], [None, None], [Some((2, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((1, 1)), Some((1, 5))], [None, None], [None, None], [None, None], [Some((1, 5)), None]], [[Some((1, 1)), Some((2, 2))], [None, None], [Some((1, 1)), Some((2, 4))], [None, None], [Some((1, 5)), Some((2, 2))], [None, None], [Some((1, 5)), Some((2, 4))], [None, None]], [[None, None], [Some((2, 2)), None], [None, None], [Some((2, 2)), Some((2, 4))], [None, None], [Some((2, 4)), None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[Some((1, 2)), None], [None, None], [Some((2, 3)), None], [None, None], [None, None], [None, None], [Some((2, 5)), None], [None, None]], [[None, None], [Some((2, 3)), None], [None, None], [Some((2, 5)), None], [None, None], [Some((2, 3)), None], [None, None], [Some((2, 5)), None]], [[Some((1, 2)), None], [None, None], [None, None], [None, None], [Some((1, 2)), Some((1, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((1, 2)), Some((2, 3))], [None, None], [Some((1, 2)), Some((2, 5))], [None, None], [Some((1, 6)), Some((2, 3))], [None, None], [Some((1, 6)), Some((2, 5))]], [[None, None], [None, None], [Some((2, 3)), None], [None, None], [Some((2, 3)), Some((2, 5))], [None, None], [Some((2, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [Some((1, 3)), None], [None, None], [Some((2, 4)), None], [None, None], [None, None], [None, None], [Some((2, 6)), None]], [[None, None], [None, None], [Some((2, 4)), None], [None, None], [Some((2, 6)), None], [None, None], [Some((2, 4)), None], [None, None]], [[None, None], [Some((1, 3)), None], [None, None], [None, None], [None, None], [Some((1, 3)), Some((1, 7))], [None, None], [None, None]], [[None, None], [None, None], [Some((1, 3)), Some((2, 4))], [None, None], [Some((1, 3)), Some((2, 6))], [None, None], [Some((1, 7)), Some((2, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 4)), None], [None, None], [Some((2, 4)), Some((2, 6))], [None, None], [Some((2, 6)), None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [Some((1, 4)), None], [None, None], [Some((2, 5)), None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 5)), None], [None, None], [Some((2, 7)), None], [None, None], [Some((2, 5)), None]], [[None, None], [None, None], [Some((1, 4)), None], [None, None], [None, None], [None, None], [Some((1, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((1, 4)), Some((2, 5))], [None, None], [Some((1, 4)), Some((2, 7))], [None, None], [Some((2, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((2, 5)), None], [None, None], [Some((2, 5)), Some((2, 7))], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [Some((1, 5)), None], [None, None], [Some((2, 6)), None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [Some((2, 6)), None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((1, 5)), None], [None, None], [None, None], [None, None], [Some((1, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((1, 5)), Some((2, 6))], [None, None], [Some((1, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((2, 6)), None], [None, None], [Some((2, 6)), None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]]], [[[[None, None], [Some((2, 2)), None], [None, None], [Some((2, 2)), None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [Some((3, 1)), None], [None, None], [Some((0, 2)), Some((2, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((0, 2)), None], [None, None], [Some((0, 2)), Some((3, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((2, 2)), None], [None, None], [None, None], [None, None], [Some((2, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 2)), None], [None, None], [Some((2, 2)), Some((3, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((3, 1)), None], [None, None], [Some((3, 1)), None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [Some((2, 3)), None], [None, None], [Some((2, 3)), None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 2)), None], [None, None], [Some((0, 3)), Some((2, 3))], [None, None], [None, None]], [[Some((3, 2)), None], [None, None], [Some((0, 3)), Some((3, 0))], [None, None], [Some((0, 3)), Some((3, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 3)), None], [None, None], [None, None], [None, None], [Some((2, 3)), None], [None, None], [None, None]], [[Some((3, 2)), None], [None, None], [Some((2, 3)), Some((3, 0))], [None, None], [Some((2, 3)), Some((3, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 0)), Some((3, 2))], [None, None], [Some((3, 2)), None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [Some((2, 0)), None], [None, None], [Some((2, 4)), None], [None, None], [Some((2, 4)), None], [None, None], [None, None]], [[Some((3, 1)), None], [None, None], [None, None], [None, None], [Some((3, 3)), None], [None, None], [Some((0, 4)), Some((2, 4))], [None, None]], [[None, None], [Some((0, 0)), Some((3, 3))], [None, None], [Some((0, 4)), Some((3, 1))], [None, None], [Some((0, 4)), Some((3, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((2, 0)), Some((2, 4))], [None, None], [None, None], [None, None], [Some((2, 4)), None], [None, None]], [[None, None], [Some((2, 0)), Some((3, 3))], [None, None], [Some((2, 4)), Some((3, 1))], [None, None], [Some((2, 4)), Some((3, 3))], [None, None], [None, None]], [[Some((3, 1)), None], [None, None], [Some((3, 1)), Some((3, 3))], [None, None], [Some((3, 3)), None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[Some((2, 1)), None], [None, None], [Some((2, 1)), None], [None, None], [Some((2, 5)), None], [None, None], [Some((2, 5)), None], [None, None]], [[None, None], [Some((3, 2)), None], [None, None], [None, None], [None, None], [Some((3, 4)), None], [None, None], [Some((0, 5)), Some((2, 5))]], [[Some((0, 1)), Some((3, 2))], [None, None], [Some((0, 1)), Some((3, 4))], [None, None], [Some((0, 5)), Some((3, 2))], [None, None], [Some((0, 5)), Some((3, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 1)), Some((2, 5))], [None, None], [None, None], [None, None], [Some((2, 5)), None]], [[Some((2, 1)), Some((3, 2))], [None, None], [Some((2, 1)), Some((3, 4))], [None, None], [Some((2, 5)), Some((3, 2))], [None, None], [Some((2, 5)), Some((3, 4))], [None, None]], [[None, None], [Some((3, 2)), None], [None, None], [Some((3, 2)), Some((3, 4))], [None, None], [Some((3, 4)), None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [Some((2, 2)), None], [None, None], [Some((2, 2)), None], [None, None], [Some((2, 6)), None], [None, None], [Some((2, 6)), None]], [[Some((0, 2)), Some((2, 2))], [None, None], [Some((3, 3)), None], [None, None], [None, None], [None, None], [Some((3, 5)), None], [None, None]], [[None, None], [Some((0, 2)), Some((3, 3))], [None, None], [Some((0, 2)), Some((3, 5))], [None, None], [Some((0, 6)), Some((3, 3))], [None, None], [Some((0, 6)), Some((3, 5))]], [[Some((2, 2)), None], [None, None], [None, None], [None, None], [Some((2, 2)), Some((2, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 2)), Some((3, 3))], [None, None], [Some((2, 2)), Some((3, 5))], [None, None], [Some((2, 6)), Some((3, 3))], [None, None], [Some((2, 6)), Some((3, 5))]], [[None, None], [None, None], [Some((3, 3)), None], [None, None], [Some((3, 3)), Some((3, 5))], [None, None], [Some((3, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [Some((2, 3)), None], [None, None], [Some((2, 3)), None], [None, None], [Some((2, 7)), None], [None, None]], [[None, None], [Some((0, 3)), Some((2, 3))], [None, None], [Some((3, 4)), None], [None, None], [None, None], [None, None], [Some((3, 6)), None]], [[None, None], [None, None], [Some((0, 3)), Some((3, 4))], [None, None], [Some((0, 3)), Some((3, 6))], [None, None], [Some((0, 7)), Some((3, 4))], [None, None]], [[None, None], [Some((2, 3)), None], [None, None], [None, None], [None, None], [Some((2, 3)), Some((2, 7))], [None, None], [None, None]], [[None, None], [None, None], [Some((2, 3)), Some((3, 4))], [None, None], [Some((2, 3)), Some((3, 6))], [None, None], [Some((2, 7)), Some((3, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 4)), None], [None, None], [Some((3, 4)), Some((3, 6))], [None, None], [Some((3, 6)), None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [Some((2, 4)), None], [None, None], [Some((2, 4)), None], [None, None], [None, None]], [[None, None], [None, None], [Some((0, 4)), Some((2, 4))], [None, None], [Some((3, 5)), None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((0, 4)), Some((3, 5))], [None, None], [Some((0, 4)), Some((3, 7))], [None, None], [Some((3, 5)), None]], [[None, None], [None, None], [Some((2, 4)), None], [None, None], [None, None], [None, None], [Some((2, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 4)), Some((3, 5))], [None, None], [Some((2, 4)), Some((3, 7))], [None, None], [Some((3, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((3, 5)), None], [None, None], [Some((3, 5)), Some((3, 7))], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [Some((2, 5)), None], [None, None], [Some((2, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((0, 5)), Some((2, 5))], [None, None], [Some((3, 6)), None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [Some((0, 5)), Some((3, 6))], [None, None], [Some((0, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 5)), None], [None, None], [None, None], [None, None], [Some((2, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((2, 5)), Some((3, 6))], [None, None], [Some((2, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((3, 6)), None], [None, None], [Some((3, 6)), None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]]], [[[[Some((1, 2)), None], [None, None], [None, None], [None, None], [Some((1, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 2)), None], [None, None], [Some((3, 2)), Some((0, 1))], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [Some((0, 1)), Some((4, 1))], [None, None], [Some((1, 2)), Some((3, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((1, 2)), None], [None, None], [Some((1, 2)), Some((4, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((3, 2)), None], [None, None], [None, None], [None, None], [Some((3, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 2)), None], [None, None], [Some((3, 2)), Some((4, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((4, 1)), None], [None, None], [Some((4, 1)), None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [Some((1, 3)), None], [None, None], [None, None], [None, None], [Some((1, 3)), None], [None, None], [None, None]], [[Some((0, 2)), None], [None, None], [Some((3, 3)), Some((0, 0))], [None, None], [Some((3, 3)), Some((0, 2))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((0, 2)), Some((4, 2))], [None, None], [Some((1, 3)), Some((3, 3))], [None, None], [None, None]], [[Some((4, 2)), None], [None, None], [Some((1, 3)), Some((4, 0))], [None, None], [Some((1, 3)), Some((4, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 3)), None], [None, None], [None, None], [None, None], [Some((3, 3)), None], [None, None], [None, None]], [[Some((4, 2)), None], [None, None], [Some((3, 3)), Some((4, 0))], [None, None], [Some((3, 3)), Some((4, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 0)), Some((4, 2))], [None, None], [Some((4, 2)), None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [Some((1, 0)), Some((1, 4))], [None, None], [None, None], [None, None], [Some((1, 4)), None], [None, None]], [[None, None], [Some((3, 0)), Some((0, 3))], [None, None], [Some((3, 4)), Some((0, 1))], [None, None], [Some((3, 4)), Some((0, 3))], [None, None], [None, None]], [[Some((0, 1)), Some((4, 1))], [None, None], [None, None], [None, None], [Some((0, 3)), Some((4, 3))], [None, None], [Some((1, 4)), Some((3, 4))], [None, None]], [[None, None], [Some((1, 0)), Some((4, 3))], [None, None], [Some((1, 4)), Some((4, 1))], [None, None], [Some((1, 4)), Some((4, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((3, 0)), Some((3, 4))], [None, None], [None, None], [None, None], [Some((3, 4)), None], [None, None]], [[None, None], [Some((3, 0)), Some((4, 3))], [None, None], [Some((3, 4)), Some((4, 1))], [None, None], [Some((3, 4)), Some((4, 3))], [None, None], [None, None]], [[Some((4, 1)), None], [None, None], [Some((4, 1)), Some((4, 3))], [None, None], [Some((4, 3)), None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [Some((1, 1)), Some((1, 5))], [None, None], [None, None], [None, None], [Some((1, 5)), None]], [[Some((3, 1)), Some((0, 2))], [None, None], [Some((3, 1)), Some((0, 4))], [None, None], [Some((3, 5)), Some((0, 2))], [None, None], [Some((3, 5)), Some((0, 4))], [None, None]], [[None, None], [Some((0, 2)), Some((4, 2))], [None, None], [None, None], [None, None], [Some((0, 4)), Some((4, 4))], [None, None], [Some((1, 5)), Some((3, 5))]], [[Some((1, 1)), Some((4, 2))], [None, None], [Some((1, 1)), Some((4, 4))], [None, None], [Some((1, 5)), Some((4, 2))], [None, None], [Some((1, 5)), Some((4, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 1)), Some((3, 5))], [None, None], [None, None], [None, None], [Some((3, 5)), None]], [[Some((3, 1)), Some((4, 2))], [None, None], [Some((3, 1)), Some((4, 4))], [None, None], [Some((3, 5)), Some((4, 2))], [None, None], [Some((3, 5)), Some((4, 4))], [None, None]], [[None, None], [Some((4, 2)), None], [None, None], [Some((4, 2)), Some((4, 4))], [None, None], [Some((4, 4)), None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[Some((1, 2)), None], [None, None], [None, None], [None, None], [Some((1, 2)), Some((1, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 2)), Some((0, 3))], [None, None], [Some((3, 2)), Some((0, 5))], [None, None], [Some((3, 6)), Some((0, 3))], [None, None], [Some((3, 6)), Some((0, 5))]], [[Some((1, 2)), Some((3, 2))], [None, None], [Some((0, 3)), Some((4, 3))], [None, None], [None, None], [None, None], [Some((0, 5)), Some((4, 5))], [None, None]], [[None, None], [Some((1, 2)), Some((4, 3))], [None, None], [Some((1, 2)), Some((4, 5))], [None, None], [Some((1, 6)), Some((4, 3))], [None, None], [Some((1, 6)), Some((4, 5))]], [[Some((3, 2)), None], [None, None], [None, None], [None, None], [Some((3, 2)), Some((3, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 2)), Some((4, 3))], [None, None], [Some((3, 2)), Some((4, 5))], [None, None], [Some((3, 6)), Some((4, 3))], [None, None], [Some((3, 6)), Some((4, 5))]], [[None, None], [None, None], [Some((4, 3)), None], [None, None], [Some((4, 3)), Some((4, 5))], [None, None], [Some((4, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [Some((1, 3)), None], [None, None], [None, None], [None, None], [Some((1, 3)), Some((1, 7))], [None, None], [None, None]], [[None, None], [None, None], [Some((3, 3)), Some((0, 4))], [None, None], [Some((3, 3)), Some((0, 6))], [None, None], [Some((3, 7)), Some((0, 4))], [None, None]], [[None, None], [Some((1, 3)), Some((3, 3))], [None, None], [Some((0, 4)), Some((4, 4))], [None, None], [None, None], [None, None], [Some((0, 6)), Some((4, 6))]], [[None, None], [None, None], [Some((1, 3)), Some((4, 4))], [None, None], [Some((1, 3)), Some((4, 6))], [None, None], [Some((1, 7)), Some((4, 4))], [None, None]], [[None, None], [Some((3, 3)), None], [None, None], [None, None], [None, None], [Some((3, 3)), Some((3, 7))], [None, None], [None, None]], [[None, None], [None, None], [Some((3, 3)), Some((4, 4))], [None, None], [Some((3, 3)), Some((4, 6))], [None, None], [Some((3, 7)), Some((4, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 4)), None], [None, None], [Some((4, 4)), Some((4, 6))], [None, None], [Some((4, 6)), None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [Some((1, 4)), None], [None, None], [None, None], [None, None], [Some((1, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 4)), Some((0, 5))], [None, None], [Some((3, 4)), Some((0, 7))], [None, None], [Some((0, 5)), None]], [[None, None], [None, None], [Some((1, 4)), Some((3, 4))], [None, None], [Some((0, 5)), Some((4, 5))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((1, 4)), Some((4, 5))], [None, None], [Some((1, 4)), Some((4, 7))], [None, None], [Some((4, 5)), None]], [[None, None], [None, None], [Some((3, 4)), None], [None, None], [None, None], [None, None], [Some((3, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 4)), Some((4, 5))], [None, None], [Some((3, 4)), Some((4, 7))], [None, None], [Some((4, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((4, 5)), None], [None, None], [Some((4, 5)), Some((4, 7))], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [Some((1, 5)), None], [None, None], [None, None], [None, None], [Some((1, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((3, 5)), Some((0, 6))], [None, None], [Some((3, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((1, 5)), Some((3, 5))], [None, None], [Some((0, 6)), Some((4, 6))], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [Some((1, 5)), Some((4, 6))], [None, None], [Some((1, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 5)), None], [None, None], [None, None], [None, None], [Some((3, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((3, 5)), Some((4, 6))], [None, None], [Some((3, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((4, 6)), None], [None, None], [Some((4, 6)), None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]]]], [[[[None, None], [Some((2, 2)), None], [None, None], [Some((2, 2)), Some((1, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((2, 2)), None], [None, None], [None, None], [None, None], [Some((2, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 2)), None], [None, None], [Some((4, 2)), Some((1, 1))], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [Some((1, 1)), Some((5, 1))], [None, None], [Some((2, 2)), Some((4, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 2)), None], [None, None], [Some((2, 2)), Some((5, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((4, 2)), None], [None, None], [None, None], [None, None], [Some((4, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 2)), None], [None, None], [Some((4, 2)), Some((5, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((5, 1)), None], [None, None], [Some((5, 1)), None], [None, None], [None, None], [None, None], [None, None], [None, None]]], [[[Some((1, 2)), None], [None, None], [Some((2, 3)), Some((1, 0))], [None, None], [Some((2, 3)), Some((1, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((2, 3)), None], [None, None], [None, None], [None, None], [Some((2, 3)), None], [None, None], [None, None]], [[Some((1, 2)), None], [None, None], [Some((4, 3)), Some((1, 0))], [None, None], [Some((4, 3)), Some((1, 2))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((1, 2)), Some((5, 2))], [None, None], [Some((2, 3)), Some((4, 3))], [None, None], [None, None]], [[Some((5, 2)), None], [None, None], [Some((2, 3)), Some((5, 0))], [None, None], [Some((2, 3)), Some((5, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 3)), None], [None, None], [None, None], [None, None], [Some((4, 3)), None], [None, None], [None, None]], [[Some((5, 2)), None], [None, None], [Some((4, 3)), Some((5, 0))], [None, None], [Some((4, 3)), Some((5, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 0)), Some((5, 2))], [None, None], [Some((5, 2)), None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [Some((2, 0)), Some((1, 3))], [None, None], [Some((2, 4)), Some((1, 1))], [None, None], [Some((2, 4)), Some((1, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((2, 0)), Some((2, 4))], [None, None], [None, None], [None, None], [Some((2, 4)), None], [None, None]], [[None, None], [Some((4, 0)), Some((1, 3))], [None, None], [Some((4, 4)), Some((1, 1))], [None, None], [Some((4, 4)), Some((1, 3))], [None, None], [None, None]], [[Some((1, 1)), Some((5, 1))], [None, None], [None, None], [None, None], [Some((1, 3)), Some((5, 3))], [None, None], [Some((2, 4)), Some((4, 4))], [None, None]], [[None, None], [Some((2, 0)), Some((5, 3))], [None, None], [Some((2, 4)), Some((5, 1))], [None, None], [Some((2, 4)), Some((5, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((4, 0)), Some((4, 4))], [None, None], [None, None], [None, None], [Some((4, 4)), None], [None, None]], [[None, None], [Some((4, 0)), Some((5, 3))], [None, None], [Some((4, 4)), Some((5, 1))], [None, None], [Some((4, 4)), Some((5, 3))], [None, None], [None, None]], [[Some((5, 1)), None], [None, None], [Some((5, 1)), Some((5, 3))], [None, None], [Some((5, 3)), None], [None, None], [None, None], [None, None]]], [[[Some((2, 1)), Some((1, 2))], [None, None], [Some((2, 1)), Some((1, 4))], [None, None], [Some((2, 5)), Some((1, 2))], [None, None], [Some((2, 5)), Some((1, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 1)), Some((2, 5))], [None, None], [None, None], [None, None], [Some((2, 5)), None]], [[Some((4, 1)), Some((1, 2))], [None, None], [Some((4, 1)), Some((1, 4))], [None, None], [Some((4, 5)), Some((1, 2))], [None, None], [Some((4, 5)), Some((1, 4))], [None, None]], [[None, None], [Some((1, 2)), Some((5, 2))], [None, None], [None, None], [None, None], [Some((1, 4)), Some((5, 4))], [None, None], [Some((2, 5)), Some((4, 5))]], [[Some((2, 1)), Some((5, 2))], [None, None], [Some((2, 1)), Some((5, 4))], [None, None], [Some((2, 5)), Some((5, 2))], [None, None], [Some((2, 5)), Some((5, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 1)), Some((4, 5))], [None, None], [None, None], [None, None], [Some((4, 5)), None]], [[Some((4, 1)), Some((5, 2))], [None, None], [Some((4, 1)), Some((5, 4))], [None, None], [Some((4, 5)), Some((5, 2))], [None, None], [Some((4, 5)), Some((5, 4))], [None, None]], [[None, None], [Some((5, 2)), None], [None, None], [Some((5, 2)), Some((5, 4))], [None, None], [Some((5, 4)), None], [None, None], [None, None]]], [[[None, None], [Some((2, 2)), Some((1, 3))], [None, None], [Some((2, 2)), Some((1, 5))], [None, None], [Some((2, 6)), Some((1, 3))], [None, None], [Some((2, 6)), Some((1, 5))]], [[Some((2, 2)), None], [None, None], [None, None], [None, None], [Some((2, 2)), Some((2, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 2)), Some((1, 3))], [None, None], [Some((4, 2)), Some((1, 5))], [None, None], [Some((4, 6)), Some((1, 3))], [None, None], [Some((4, 6)), Some((1, 5))]], [[Some((2, 2)), Some((4, 2))], [None, None], [Some((1, 3)), Some((5, 3))], [None, None], [None, None], [None, None], [Some((1, 5)), Some((5, 5))], [None, None]], [[None, None], [Some((2, 2)), Some((5, 3))], [None, None], [Some((2, 2)), Some((5, 5))], [None, None], [Some((2, 6)), Some((5, 3))], [None, None], [Some((2, 6)), Some((5, 5))]], [[Some((4, 2)), None], [None, None], [None, None], [None, None], [Some((4, 2)), Some((4, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 2)), Some((5, 3))], [None, None], [Some((4, 2)), Some((5, 5))], [None, None], [Some((4, 6)), Some((5, 3))], [None, None], [Some((4, 6)), Some((5, 5))]], [[None, None], [None, None], [Some((5, 3)), None], [None, None], [Some((5, 3)), Some((5, 5))], [None, None], [Some((5, 5)), None], [None, None]]], [[[None, None], [None, None], [Some((2, 3)), Some((1, 4))], [None, None], [Some((2, 3)), Some((1, 6))], [None, None], [Some((2, 7)), Some((1, 4))], [None, None]], [[None, None], [Some((2, 3)), None], [None, None], [None, None], [None, None], [Some((2, 3)), Some((2, 7))], [None, None], [None, None]], [[None, None], [None, None], [Some((4, 3)), Some((1, 4))], [None, None], [Some((4, 3)), Some((1, 6))], [None, None], [Some((4, 7)), Some((1, 4))], [None, None]], [[None, None], [Some((2, 3)), Some((4, 3))], [None, None], [Some((1, 4)), Some((5, 4))], [None, None], [None, None], [None, None], [Some((1, 6)), Some((5, 6))]], [[None, None], [None, None], [Some((2, 3)), Some((5, 4))], [None, None], [Some((2, 3)), Some((5, 6))], [None, None], [Some((2, 7)), Some((5, 4))], [None, None]], [[None, None], [Some((4, 3)), None], [None, None], [None, None], [None, None], [Some((4, 3)), Some((4, 7))], [None, None], [None, None]], [[None, None], [None, None], [Some((4, 3)), Some((5, 4))], [None, None], [Some((4, 3)), Some((5, 6))], [None, None], [Some((4, 7)), Some((5, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 4)), None], [None, None], [Some((5, 4)), Some((5, 6))], [None, None], [Some((5, 6)), None]]], [[[None, None], [None, None], [None, None], [Some((2, 4)), Some((1, 5))], [None, None], [Some((2, 4)), Some((1, 7))], [None, None], [Some((1, 5)), None]], [[None, None], [None, None], [Some((2, 4)), None], [None, None], [None, None], [None, None], [Some((2, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 4)), Some((1, 5))], [None, None], [Some((4, 4)), Some((1, 7))], [None, None], [Some((1, 5)), None]], [[None, None], [None, None], [Some((2, 4)), Some((4, 4))], [None, None], [Some((1, 5)), Some((5, 5))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 4)), Some((5, 5))], [None, None], [Some((2, 4)), Some((5, 7))], [None, None], [Some((5, 5)), None]], [[None, None], [None, None], [Some((4, 4)), None], [None, None], [None, None], [None, None], [Some((4, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 4)), Some((5, 5))], [None, None], [Some((4, 4)), Some((5, 7))], [None, None], [Some((5, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((5, 5)), None], [None, None], [Some((5, 5)), Some((5, 7))], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [Some((2, 5)), Some((1, 6))], [None, None], [Some((2, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 5)), None], [None, None], [None, None], [None, None], [Some((2, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((4, 5)), Some((1, 6))], [None, None], [Some((4, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 5)), Some((4, 5))], [None, None], [Some((1, 6)), Some((5, 6))], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [Some((2, 5)), Some((5, 6))], [None, None], [Some((2, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 5)), None], [None, None], [None, None], [None, None], [Some((4, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((4, 5)), Some((5, 6))], [None, None], [Some((4, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 6)), None], [None, None], [Some((5, 6)), None]]]], [[[[Some((2, 1)), None], [None, None], [Some((2, 1)), None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 2)), None], [None, None], [Some((3, 2)), Some((2, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((3, 2)), None], [None, None], [None, None], [None, None], [Some((3, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 2)), None], [None, None], [Some((5, 2)), Some((2, 1))], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [Some((2, 1)), Some((6, 1))], [None, None], [Some((3, 2)), Some((5, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 2)), None], [None, None], [Some((3, 2)), Some((6, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((5, 2)), None], [None, None], [None, None], [None, None], [Some((5, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 2)), None], [None, None], [Some((5, 2)), Some((6, 1))], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [Some((2, 0)), Some((2, 2))], [None, None], [Some((2, 2)), None], [None, None], [None, None], [None, None], [None, None]], [[Some((2, 2)), None], [None, None], [Some((3, 3)), Some((2, 0))], [None, None], [Some((3, 3)), Some((2, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 3)), None], [None, None], [None, None], [None, None], [Some((3, 3)), None], [None, None], [None, None]], [[Some((2, 2)), None], [None, None], [Some((5, 3)), Some((2, 0))], [None, None], [Some((5, 3)), Some((2, 2))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((2, 2)), Some((6, 2))], [None, None], [Some((3, 3)), Some((5, 3))], [None, None], [None, None]], [[Some((6, 2)), None], [None, None], [Some((3, 3)), Some((6, 0))], [None, None], [Some((3, 3)), Some((6, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 3)), None], [None, None], [None, None], [None, None], [Some((5, 3)), None], [None, None], [None, None]], [[Some((6, 2)), None], [None, None], [Some((5, 3)), Some((6, 0))], [None, None], [Some((5, 3)), Some((6, 2))], [None, None], [None, None], [None, None]]], [[[Some((2, 1)), None], [None, None], [Some((2, 1)), Some((2, 3))], [None, None], [Some((2, 3)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 0)), Some((2, 3))], [None, None], [Some((3, 4)), Some((2, 1))], [None, None], [Some((3, 4)), Some((2, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((3, 0)), Some((3, 4))], [None, None], [None, None], [None, None], [Some((3, 4)), None], [None, None]], [[None, None], [Some((5, 0)), Some((2, 3))], [None, None], [Some((5, 4)), Some((2, 1))], [None, None], [Some((5, 4)), Some((2, 3))], [None, None], [None, None]], [[Some((2, 1)), Some((6, 1))], [None, None], [None, None], [None, None], [Some((2, 3)), Some((6, 3))], [None, None], [Some((3, 4)), Some((5, 4))], [None, None]], [[None, None], [Some((3, 0)), Some((6, 3))], [None, None], [Some((3, 4)), Some((6, 1))], [None, None], [Some((3, 4)), Some((6, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((5, 0)), Some((5, 4))], [None, None], [None, None], [None, None], [Some((5, 4)), None], [None, None]], [[None, None], [Some((5, 0)), Some((6, 3))], [None, None], [Some((5, 4)), Some((6, 1))], [None, None], [Some((5, 4)), Some((6, 3))], [None, None], [None, None]]], [[[None, None], [Some((2, 2)), None], [None, None], [Some((2, 2)), Some((2, 4))], [None, None], [Some((2, 4)), None], [None, None], [None, None]], [[Some((3, 1)), Some((2, 2))], [None, None], [Some((3, 1)), Some((2, 4))], [None, None], [Some((3, 5)), Some((2, 2))], [None, None], [Some((3, 5)), Some((2, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 1)), Some((3, 5))], [None, None], [None, None], [None, None], [Some((3, 5)), None]], [[Some((5, 1)), Some((2, 2))], [None, None], [Some((5, 1)), Some((2, 4))], [None, None], [Some((5, 5)), Some((2, 2))], [None, None], [Some((5, 5)), Some((2, 4))], [None, None]], [[None, None], [Some((2, 2)), Some((6, 2))], [None, None], [None, None], [None, None], [Some((2, 4)), Some((6, 4))], [None, None], [Some((3, 5)), Some((5, 5))]], [[Some((3, 1)), Some((6, 2))], [None, None], [Some((3, 1)), Some((6, 4))], [None, None], [Some((3, 5)), Some((6, 2))], [None, None], [Some((3, 5)), Some((6, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 1)), Some((5, 5))], [None, None], [None, None], [None, None], [Some((5, 5)), None]], [[Some((5, 1)), Some((6, 2))], [None, None], [Some((5, 1)), Some((6, 4))], [None, None], [Some((5, 5)), Some((6, 2))], [None, None], [Some((5, 5)), Some((6, 4))], [None, None]]], [[[None, None], [None, None], [Some((2, 3)), None], [None, None], [Some((2, 3)), Some((2, 5))], [None, None], [Some((2, 5)), None], [None, None]], [[None, None], [Some((3, 2)), Some((2, 3))], [None, None], [Some((3, 2)), Some((2, 5))], [None, None], [Some((3, 6)), Some((2, 3))], [None, None], [Some((3, 6)), Some((2, 5))]], [[Some((3, 2)), None], [None, None], [None, None], [None, None], [Some((3, 2)), Some((3, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 2)), Some((2, 3))], [None, None], [Some((5, 2)), Some((2, 5))], [None, None], [Some((5, 6)), Some((2, 3))], [None, None], [Some((5, 6)), Some((2, 5))]], [[Some((3, 2)), Some((5, 2))], [None, None], [Some((2, 3)), Some((6, 3))], [None, None], [None, None], [None, None], [Some((2, 5)), Some((6, 5))], [None, None]], [[None, None], [Some((3, 2)), Some((6, 3))], [None, None], [Some((3, 2)), Some((6, 5))], [None, None], [Some((3, 6)), Some((6, 3))], [None, None], [Some((3, 6)), Some((6, 5))]], [[Some((5, 2)), None], [None, None], [None, None], [None, None], [Some((5, 2)), Some((5, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 2)), Some((6, 3))], [None, None], [Some((5, 2)), Some((6, 5))], [None, None], [Some((5, 6)), Some((6, 3))], [None, None], [Some((5, 6)), Some((6, 5))]]], [[[None, None], [None, None], [None, None], [Some((2, 4)), None], [None, None], [Some((2, 4)), Some((2, 6))], [None, None], [Some((2, 6)), None]], [[None, None], [None, None], [Some((3, 3)), Some((2, 4))], [None, None], [Some((3, 3)), Some((2, 6))], [None, None], [Some((3, 7)), Some((2, 4))], [None, None]], [[None, None], [Some((3, 3)), None], [None, None], [None, None], [None, None], [Some((3, 3)), Some((3, 7))], [None, None], [None, None]], [[None, None], [None, None], [Some((5, 3)), Some((2, 4))], [None, None], [Some((5, 3)), Some((2, 6))], [None, None], [Some((5, 7)), Some((2, 4))], [None, None]], [[None, None], [Some((3, 3)), Some((5, 3))], [None, None], [Some((2, 4)), Some((6, 4))], [None, None], [None, None], [None, None], [Some((2, 6)), Some((6, 6))]], [[None, None], [None, None], [Some((3, 3)), Some((6, 4))], [None, None], [Some((3, 3)), Some((6, 6))], [None, None], [Some((3, 7)), Some((6, 4))], [None, None]], [[None, None], [Some((5, 3)), None], [None, None], [None, None], [None, None], [Some((5, 3)), Some((5, 7))], [None, None], [None, None]], [[None, None], [None, None], [Some((5, 3)), Some((6, 4))], [None, None], [Some((5, 3)), Some((6, 6))], [None, None], [Some((5, 7)), Some((6, 4))], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [Some((2, 5)), None], [None, None], [Some((2, 5)), Some((2, 7))], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 4)), Some((2, 5))], [None, None], [Some((3, 4)), Some((2, 7))], [None, None], [Some((2, 5)), None]], [[None, None], [None, None], [Some((3, 4)), None], [None, None], [None, None], [None, None], [Some((3, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 4)), Some((2, 5))], [None, None], [Some((5, 4)), Some((2, 7))], [None, None], [Some((2, 5)), None]], [[None, None], [None, None], [Some((3, 4)), Some((5, 4))], [None, None], [Some((2, 5)), Some((6, 5))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 4)), Some((6, 5))], [None, None], [Some((3, 4)), Some((6, 7))], [None, None], [Some((6, 5)), None]], [[None, None], [None, None], [Some((5, 4)), None], [None, None], [None, None], [None, None], [Some((5, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 4)), Some((6, 5))], [None, None], [Some((5, 4)), Some((6, 7))], [None, None], [Some((6, 5)), None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [Some((2, 6)), None], [None, None], [Some((2, 6)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((3, 5)), Some((2, 6))], [None, None], [Some((3, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 5)), None], [None, None], [None, None], [None, None], [Some((3, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((5, 5)), Some((2, 6))], [None, None], [Some((5, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 5)), Some((5, 5))], [None, None], [Some((2, 6)), Some((6, 6))], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [Some((3, 5)), Some((6, 6))], [None, None], [Some((3, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 5)), None], [None, None], [None, None], [None, None], [Some((5, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((5, 5)), Some((6, 6))], [None, None], [Some((5, 5)), None], [None, None]]]], [[[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[Some((3, 1)), None], [None, None], [Some((3, 1)), None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 2)), None], [None, None], [Some((4, 2)), Some((3, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((4, 2)), None], [None, None], [None, None], [None, None], [Some((4, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((6, 2)), None], [None, None], [Some((6, 2)), Some((3, 1))], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [Some((3, 1)), Some((7, 1))], [None, None], [Some((4, 2)), Some((6, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 2)), None], [None, None], [Some((4, 2)), Some((7, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((6, 2)), None], [None, None], [None, None], [None, None], [Some((6, 2)), None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 0)), Some((3, 2))], [None, None], [Some((3, 2)), None], [None, None], [None, None], [None, None], [None, None]], [[Some((3, 2)), None], [None, None], [Some((4, 3)), Some((3, 0))], [None, None], [Some((4, 3)), Some((3, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 3)), None], [None, None], [None, None], [None, None], [Some((4, 3)), None], [None, None], [None, None]], [[Some((3, 2)), None], [None, None], [Some((6, 3)), Some((3, 0))], [None, None], [Some((6, 3)), Some((3, 2))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 2)), Some((7, 2))], [None, None], [Some((4, 3)), Some((6, 3))], [None, None], [None, None]], [[Some((7, 2)), None], [None, None], [Some((4, 3)), Some((7, 0))], [None, None], [Some((4, 3)), Some((7, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((6, 3)), None], [None, None], [None, None], [None, None], [Some((6, 3)), None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[Some((3, 1)), None], [None, None], [Some((3, 1)), Some((3, 3))], [None, None], [Some((3, 3)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 0)), Some((3, 3))], [None, None], [Some((4, 4)), Some((3, 1))], [None, None], [Some((4, 4)), Some((3, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((4, 0)), Some((4, 4))], [None, None], [None, None], [None, None], [Some((4, 4)), None], [None, None]], [[None, None], [Some((6, 0)), Some((3, 3))], [None, None], [Some((6, 4)), Some((3, 1))], [None, None], [Some((6, 4)), Some((3, 3))], [None, None], [None, None]], [[Some((3, 1)), Some((7, 1))], [None, None], [None, None], [None, None], [Some((3, 3)), Some((7, 3))], [None, None], [Some((4, 4)), Some((6, 4))], [None, None]], [[None, None], [Some((4, 0)), Some((7, 3))], [None, None], [Some((4, 4)), Some((7, 1))], [None, None], [Some((4, 4)), Some((7, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((6, 0)), Some((6, 4))], [None, None], [None, None], [None, None], [Some((6, 4)), None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((3, 2)), None], [None, None], [Some((3, 2)), Some((3, 4))], [None, None], [Some((3, 4)), None], [None, None], [None, None]], [[Some((4, 1)), Some((3, 2))], [None, None], [Some((4, 1)), Some((3, 4))], [None, None], [Some((4, 5)), Some((3, 2))], [None, None], [Some((4, 5)), Some((3, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 1)), Some((4, 5))], [None, None], [None, None], [None, None], [Some((4, 5)), None]], [[Some((6, 1)), Some((3, 2))], [None, None], [Some((6, 1)), Some((3, 4))], [None, None], [Some((6, 5)), Some((3, 2))], [None, None], [Some((6, 5)), Some((3, 4))], [None, None]], [[None, None], [Some((3, 2)), Some((7, 2))], [None, None], [None, None], [None, None], [Some((3, 4)), Some((7, 4))], [None, None], [Some((4, 5)), Some((6, 5))]], [[Some((4, 1)), Some((7, 2))], [None, None], [Some((4, 1)), Some((7, 4))], [None, None], [Some((4, 5)), Some((7, 2))], [None, None], [Some((4, 5)), Some((7, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((6, 1)), Some((6, 5))], [None, None], [None, None], [None, None], [Some((6, 5)), None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [Some((3, 3)), None], [None, None], [Some((3, 3)), Some((3, 5))], [None, None], [Some((3, 5)), None], [None, None]], [[None, None], [Some((4, 2)), Some((3, 3))], [None, None], [Some((4, 2)), Some((3, 5))], [None, None], [Some((4, 6)), Some((3, 3))], [None, None], [Some((4, 6)), Some((3, 5))]], [[Some((4, 2)), None], [None, None], [None, None], [None, None], [Some((4, 2)), Some((4, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((6, 2)), Some((3, 3))], [None, None], [Some((6, 2)), Some((3, 5))], [None, None], [Some((6, 6)), Some((3, 3))], [None, None], [Some((6, 6)), Some((3, 5))]], [[Some((4, 2)), Some((6, 2))], [None, None], [Some((3, 3)), Some((7, 3))], [None, None], [None, None], [None, None], [Some((3, 5)), Some((7, 5))], [None, None]], [[None, None], [Some((4, 2)), Some((7, 3))], [None, None], [Some((4, 2)), Some((7, 5))], [None, None], [Some((4, 6)), Some((7, 3))], [None, None], [Some((4, 6)), Some((7, 5))]], [[Some((6, 2)), None], [None, None], [None, None], [None, None], [Some((6, 2)), Some((6, 6))], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((3, 4)), None], [None, None], [Some((3, 4)), Some((3, 6))], [None, None], [Some((3, 6)), None]], [[None, None], [None, None], [Some((4, 3)), Some((3, 4))], [None, None], [Some((4, 3)), Some((3, 6))], [None, None], [Some((4, 7)), Some((3, 4))], [None, None]], [[None, None], [Some((4, 3)), None], [None, None], [None, None], [None, None], [Some((4, 3)), Some((4, 7))], [None, None], [None, None]], [[None, None], [None, None], [Some((6, 3)), Some((3, 4))], [None, None], [Some((6, 3)), Some((3, 6))], [None, None], [Some((6, 7)), Some((3, 4))], [None, None]], [[None, None], [Some((4, 3)), Some((6, 3))], [None, None], [Some((3, 4)), Some((7, 4))], [None, None], [None, None], [None, None], [Some((3, 6)), Some((7, 6))]], [[None, None], [None, None], [Some((4, 3)), Some((7, 4))], [None, None], [Some((4, 3)), Some((7, 6))], [None, None], [Some((4, 7)), Some((7, 4))], [None, None]], [[None, None], [Some((6, 3)), None], [None, None], [None, None], [None, None], [Some((6, 3)), Some((6, 7))], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [Some((3, 5)), None], [None, None], [Some((3, 5)), Some((3, 7))], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 4)), Some((3, 5))], [None, None], [Some((4, 4)), Some((3, 7))], [None, None], [Some((3, 5)), None]], [[None, None], [None, None], [Some((4, 4)), None], [None, None], [None, None], [None, None], [Some((4, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((6, 4)), Some((3, 5))], [None, None], [Some((6, 4)), Some((3, 7))], [None, None], [Some((3, 5)), None]], [[None, None], [None, None], [Some((4, 4)), Some((6, 4))], [None, None], [Some((3, 5)), Some((7, 5))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 4)), Some((7, 5))], [None, None], [Some((4, 4)), Some((7, 7))], [None, None], [Some((7, 5)), None]], [[None, None], [None, None], [Some((6, 4)), None], [None, None], [None, None], [None, None], [Some((6, 4)), None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((3, 6)), None], [None, None], [Some((3, 6)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((4, 5)), Some((3, 6))], [None, None], [Some((4, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 5)), None], [None, None], [None, None], [None, None], [Some((4, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((6, 5)), Some((3, 6))], [None, None], [Some((6, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 5)), Some((6, 5))], [None, None], [Some((3, 6)), Some((7, 6))], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [Some((4, 5)), Some((7, 6))], [None, None], [Some((4, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((6, 5)), None], [None, None], [None, None], [None, None], [Some((6, 5)), None]]]], [[[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[Some((4, 1)), None], [None, None], [Some((4, 1)), None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 2)), None], [None, None], [Some((5, 2)), Some((4, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((5, 2)), None], [None, None], [None, None], [None, None], [Some((5, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((7, 2)), None], [None, None], [Some((7, 2)), Some((4, 1))], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [Some((4, 1)), None], [None, None], [Some((5, 2)), Some((7, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 2)), None], [None, None], [Some((5, 2)), None], [None, None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 0)), Some((4, 2))], [None, None], [Some((4, 2)), None], [None, None], [None, None], [None, None], [None, None]], [[Some((4, 2)), None], [None, None], [Some((5, 3)), Some((4, 0))], [None, None], [Some((5, 3)), Some((4, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 3)), None], [None, None], [None, None], [None, None], [Some((5, 3)), None], [None, None], [None, None]], [[Some((4, 2)), None], [None, None], [Some((7, 3)), Some((4, 0))], [None, None], [Some((7, 3)), Some((4, 2))], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 2)), None], [None, None], [Some((5, 3)), Some((7, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((5, 3)), None], [None, None], [Some((5, 3)), None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[Some((4, 1)), None], [None, None], [Some((4, 1)), Some((4, 3))], [None, None], [Some((4, 3)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 0)), Some((4, 3))], [None, None], [Some((5, 4)), Some((4, 1))], [None, None], [Some((5, 4)), Some((4, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((5, 0)), Some((5, 4))], [None, None], [None, None], [None, None], [Some((5, 4)), None], [None, None]], [[None, None], [Some((7, 0)), Some((4, 3))], [None, None], [Some((7, 4)), Some((4, 1))], [None, None], [Some((7, 4)), Some((4, 3))], [None, None], [None, None]], [[Some((4, 1)), None], [None, None], [None, None], [None, None], [Some((4, 3)), None], [None, None], [Some((5, 4)), Some((7, 4))], [None, None]], [[None, None], [Some((5, 0)), None], [None, None], [Some((5, 4)), None], [None, None], [Some((5, 4)), None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((4, 2)), None], [None, None], [Some((4, 2)), Some((4, 4))], [None, None], [Some((4, 4)), None], [None, None], [None, None]], [[Some((5, 1)), Some((4, 2))], [None, None], [Some((5, 1)), Some((4, 4))], [None, None], [Some((5, 5)), Some((4, 2))], [None, None], [Some((5, 5)), Some((4, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 1)), Some((5, 5))], [None, None], [None, None], [None, None], [Some((5, 5)), None]], [[Some((7, 1)), Some((4, 2))], [None, None], [Some((7, 1)), Some((4, 4))], [None, None], [Some((7, 5)), Some((4, 2))], [None, None], [Some((7, 5)), Some((4, 4))], [None, None]], [[None, None], [Some((4, 2)), None], [None, None], [None, None], [None, None], [Some((4, 4)), None], [None, None], [Some((5, 5)), Some((7, 5))]], [[Some((5, 1)), None], [None, None], [Some((5, 1)), None], [None, None], [Some((5, 5)), None], [None, None], [Some((5, 5)), None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [Some((4, 3)), None], [None, None], [Some((4, 3)), Some((4, 5))], [None, None], [Some((4, 5)), None], [None, None]], [[None, None], [Some((5, 2)), Some((4, 3))], [None, None], [Some((5, 2)), Some((4, 5))], [None, None], [Some((5, 6)), Some((4, 3))], [None, None], [Some((5, 6)), Some((4, 5))]], [[Some((5, 2)), None], [None, None], [None, None], [None, None], [Some((5, 2)), Some((5, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((7, 2)), Some((4, 3))], [None, None], [Some((7, 2)), Some((4, 5))], [None, None], [Some((7, 6)), Some((4, 3))], [None, None], [Some((7, 6)), Some((4, 5))]], [[Some((5, 2)), Some((7, 2))], [None, None], [Some((4, 3)), None], [None, None], [None, None], [None, None], [Some((4, 5)), None], [None, None]], [[None, None], [Some((5, 2)), None], [None, None], [Some((5, 2)), None], [None, None], [Some((5, 6)), None], [None, None], [Some((5, 6)), None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((4, 4)), None], [None, None], [Some((4, 4)), Some((4, 6))], [None, None], [Some((4, 6)), None]], [[None, None], [None, None], [Some((5, 3)), Some((4, 4))], [None, None], [Some((5, 3)), Some((4, 6))], [None, None], [Some((5, 7)), Some((4, 4))], [None, None]], [[None, None], [Some((5, 3)), None], [None, None], [None, None], [None, None], [Some((5, 3)), Some((5, 7))], [None, None], [None, None]], [[None, None], [None, None], [Some((7, 3)), Some((4, 4))], [None, None], [Some((7, 3)), Some((4, 6))], [None, None], [Some((7, 7)), Some((4, 4))], [None, None]], [[None, None], [Some((5, 3)), Some((7, 3))], [None, None], [Some((4, 4)), None], [None, None], [None, None], [None, None], [Some((4, 6)), None]], [[None, None], [None, None], [Some((5, 3)), None], [None, None], [Some((5, 3)), None], [None, None], [Some((5, 7)), None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [Some((4, 5)), None], [None, None], [Some((4, 5)), Some((4, 7))], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 4)), Some((4, 5))], [None, None], [Some((5, 4)), Some((4, 7))], [None, None], [Some((4, 5)), None]], [[None, None], [None, None], [Some((5, 4)), None], [None, None], [None, None], [None, None], [Some((5, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((7, 4)), Some((4, 5))], [None, None], [Some((7, 4)), Some((4, 7))], [None, None], [Some((4, 5)), None]], [[None, None], [None, None], [Some((5, 4)), Some((7, 4))], [None, None], [Some((4, 5)), None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 4)), None], [None, None], [Some((5, 4)), None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((4, 6)), None], [None, None], [Some((4, 6)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((5, 5)), Some((4, 6))], [None, None], [Some((5, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 5)), None], [None, None], [None, None], [None, None], [Some((5, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((7, 5)), Some((4, 6))], [None, None], [Some((7, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 5)), Some((7, 5))], [None, None], [Some((4, 6)), None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [Some((5, 5)), None], [None, None], [Some((5, 5)), None], [None, None]]]], [[[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[Some((5, 1)), None], [None, None], [Some((5, 1)), None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((6, 2)), None], [None, None], [Some((6, 2)), Some((5, 1))], [None, None], [None, None], [None, None], [None, None]], [[Some((6, 2)), None], [None, None], [None, None], [None, None], [Some((6, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 1)), None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [Some((5, 1)), None], [None, None], [Some((6, 2)), None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 0)), Some((5, 2))], [None, None], [Some((5, 2)), None], [None, None], [None, None], [None, None], [None, None]], [[Some((5, 2)), None], [None, None], [Some((6, 3)), Some((5, 0))], [None, None], [Some((6, 3)), Some((5, 2))], [None, None], [None, None], [None, None]], [[None, None], [Some((6, 3)), None], [None, None], [None, None], [None, None], [Some((6, 3)), None], [None, None], [None, None]], [[Some((5, 2)), None], [None, None], [Some((5, 0)), None], [None, None], [Some((5, 2)), None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 2)), None], [None, None], [Some((6, 3)), None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[Some((5, 1)), None], [None, None], [Some((5, 1)), Some((5, 3))], [None, None], [Some((5, 3)), None], [None, None], [None, None], [None, None]], [[None, None], [Some((6, 0)), Some((5, 3))], [None, None], [Some((6, 4)), Some((5, 1))], [None, None], [Some((6, 4)), Some((5, 3))], [None, None], [None, None]], [[None, None], [None, None], [Some((6, 0)), Some((6, 4))], [None, None], [None, None], [None, None], [Some((6, 4)), None], [None, None]], [[None, None], [Some((5, 3)), None], [None, None], [Some((5, 1)), None], [None, None], [Some((5, 3)), None], [None, None], [None, None]], [[Some((5, 1)), None], [None, None], [None, None], [None, None], [Some((5, 3)), None], [None, None], [Some((6, 4)), None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 2)), None], [None, None], [Some((5, 2)), Some((5, 4))], [None, None], [Some((5, 4)), None], [None, None], [None, None]], [[Some((6, 1)), Some((5, 2))], [None, None], [Some((6, 1)), Some((5, 4))], [None, None], [Some((6, 5)), Some((5, 2))], [None, None], [Some((6, 5)), Some((5, 4))], [None, None]], [[None, None], [None, None], [None, None], [Some((6, 1)), Some((6, 5))], [None, None], [None, None], [None, None], [Some((6, 5)), None]], [[Some((5, 2)), None], [None, None], [Some((5, 4)), None], [None, None], [Some((5, 2)), None], [None, None], [Some((5, 4)), None], [None, None]], [[None, None], [Some((5, 2)), None], [None, None], [None, None], [None, None], [Some((5, 4)), None], [None, None], [Some((6, 5)), None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [Some((5, 3)), None], [None, None], [Some((5, 3)), Some((5, 5))], [None, None], [Some((5, 5)), None], [None, None]], [[None, None], [Some((6, 2)), Some((5, 3))], [None, None], [Some((6, 2)), Some((5, 5))], [None, None], [Some((6, 6)), Some((5, 3))], [None, None], [Some((6, 6)), Some((5, 5))]], [[Some((6, 2)), None], [None, None], [None, None], [None, None], [Some((6, 2)), Some((6, 6))], [None, None], [None, None], [None, None]], [[None, None], [Some((5, 3)), None], [None, None], [Some((5, 5)), None], [None, None], [Some((5, 3)), None], [None, None], [Some((5, 5)), None]], [[Some((6, 2)), None], [None, None], [Some((5, 3)), None], [None, None], [None, None], [None, None], [Some((5, 5)), None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 4)), None], [None, None], [Some((5, 4)), Some((5, 6))], [None, None], [Some((5, 6)), None]], [[None, None], [None, None], [Some((6, 3)), Some((5, 4))], [None, None], [Some((6, 3)), Some((5, 6))], [None, None], [Some((6, 7)), Some((5, 4))], [None, None]], [[None, None], [Some((6, 3)), None], [None, None], [None, None], [None, None], [Some((6, 3)), Some((6, 7))], [None, None], [None, None]], [[None, None], [None, None], [Some((5, 4)), None], [None, None], [Some((5, 6)), None], [None, None], [Some((5, 4)), None], [None, None]], [[None, None], [Some((6, 3)), None], [None, None], [Some((5, 4)), None], [None, None], [None, None], [None, None], [Some((5, 6)), None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [Some((5, 5)), None], [None, None], [Some((5, 5)), Some((5, 7))], [None, None]], [[None, None], [None, None], [None, None], [Some((6, 4)), Some((5, 5))], [None, None], [Some((6, 4)), Some((5, 7))], [None, None], [Some((5, 5)), None]], [[None, None], [None, None], [Some((6, 4)), None], [None, None], [None, None], [None, None], [Some((6, 4)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((5, 5)), None], [None, None], [Some((5, 7)), None], [None, None], [Some((5, 5)), None]], [[None, None], [None, None], [Some((6, 4)), None], [None, None], [Some((5, 5)), None], [None, None], [None, None], [None, None]]], [[[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [None, None], [None, None], [Some((5, 6)), None], [None, None], [Some((5, 6)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((6, 5)), Some((5, 6))], [None, None], [Some((6, 5)), None], [None, None]], [[None, None], [None, None], [None, None], [Some((6, 5)), None], [None, None], [None, None], [None, None], [Some((6, 5)), None]], [[None, None], [None, None], [None, None], [None, None], [Some((5, 6)), None], [None, None], [None, None], [None, None]], [[None, None], [None, None], [None, None], [Some((6, 5)), None], [None, None], [Some((5, 6)), None], [None, None], [None, None]]]]];

pub fn generate_knight_moves(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    for i in PRECOMPUTED_KNIGHT_MOVES[x][y].iter() {
        if let Some((a, b)) = i {
            if board.board[*a][*b].can_move_to(board.white_to_move) {
                vector.push(Move::KnightMove(x, y, *a, *b, board.board[*a][*b]));
            }
        } else {
            return;
        }
    }
}

pub fn generate_knight_captures(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    for i in PRECOMPUTED_KNIGHT_MOVES[x][y].iter() {
        if let Some((a, b)) = i {
            if board.board[*a][*b].can_capture(board.white_to_move) {
                vector.push(Move::KnightMove(x, y, *a, *b, board.board[*a][*b]));
            }
        } else {
            return;
        }
    }
}

pub fn generate_knight_checks(board: &Board, king_x: usize, king_y: usize, x: usize, y: usize, vector: &mut Vec<Move>) {
    for i in PRECOMPUTED_KNIGHT_CHECKS[x][y][king_x][king_y].iter() {
        if let Some((a, b)) = *i {
            if board.board[a][b].is_empty() {
                vector.push(Move::KnightMove(x, y, a, b, board.board[a][b]));
            }
        } else {
            break;
        }
    }
}

pub fn generate_bishop_moves(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    for i in PRECOMPUTED_BISHOP_MOVES[x][y].iter() {
        for j in i {
            if let Some((cx, cy)) = j {
                match board.board[*cx][*cy].can_move_to_and_through(board.white_to_move) {
                    (true, true) => vector.push(Move::BishopMove(x, y, *cx, *cy, Square::E)),
                    (true, false) => {vector.push(Move::BishopMove(x, y, *cx, *cy, board.board[*cx][*cy])); break},
                    _ => break
                }
            } else {
                break
            }
        }
    }
}

pub fn generate_bishop_captures(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    for i in PRECOMPUTED_BISHOP_MOVES[x][y].iter() {
        for j in i {
            if let Some((cx, cy)) = j {
                match board.board[*cx][*cy].can_move_to_and_through(board.white_to_move) {
                    (true, true) => continue,
                    (true, false) => {vector.push(Move::BishopMove(x, y, *cx, *cy, board.board[*cx][*cy])); break},
                    _ => break
                }
            } else {
                break
            }
        }
    }
}

pub fn generate_bishop_checks(board: &Board, king_x: usize, king_y: usize, x: usize, y: usize, vector: &mut Vec<Move>) {
    // checks which are also captures will not be counted
    let mut diagonal;
    'outer: for i in PRECOMPUTED_BISHOP_CHECKS[x][y][king_x][king_y].iter() {
        if let Some((a, b)) = *i {
            diagonal = match (a > x, b > y) {
                (true, true) => 0,
                (true, false) => 1,
                (false, true) => 2,
                (false, false) => 3
            };
            for j in PRECOMPUTED_BISHOP_MOVES[x][y][diagonal].iter() {
                if let Some((cx, cy)) = *j {
                    if !board.board[cx][cy].is_empty() {
                        continue 'outer;
                    }
                    if cx == a {
                        break;
                    }
                }
            }
            diagonal = match (a > king_x, b > king_y) {
                (true, true) => 0,
                (true, false) => 1,
                (false, true) => 2,
                (false, false) => 3
            };
            for j in PRECOMPUTED_BISHOP_MOVES[king_x][king_y][diagonal].iter() {
                if let Some((cx, cy)) = *j {
                    if cx == a {
                        break;
                    }
                    if !board.board[cx][cy].is_empty() {
                        continue 'outer;
                    }
                }
            }
            vector.push(Move::BishopMove(x, y, a, b, Square::E));
        } else {
            break;
        }
    }
}


pub fn generate_rook_moves(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    for i in PRECOMPUTED_ROOK_MOVES[x][y].iter() {
        for j in i {
            if let Some((cx, cy)) = j {
                match board.board[*cx][*cy].can_move_to_and_through(board.white_to_move) {
                    (true, true) => vector.push(Move::RookMove(x, y, *cx, *cy, Square::E)),
                    (true, false) => {vector.push(Move::RookMove(x, y, *cx, *cy, board.board[*cx][*cy])); break},
                    _ => break
                }
            } else {
                break
            }
        }
    }
}

pub fn generate_rook_captures(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    for i in PRECOMPUTED_ROOK_MOVES[x][y].iter() {
        for j in i {
            if let Some((cx, cy)) = j {
                match board.board[*cx][*cy].can_move_to_and_through(board.white_to_move) {
                    (true, true) => continue,
                    (true, false) => {vector.push(Move::RookMove(x, y, *cx, *cy, board.board[*cx][*cy])); break},
                    _ => break
                }
            } else {
                break
            }
        }
    }
}

pub fn generate_rook_checks(board: &Board, king_x: usize, king_y: usize, x: usize, y: usize, vector: &mut Vec<Move>) {
    if king_x == x || king_y == y {
        return;
    }
    let mut diagonal = if king_x > x {0} else {1};
    'outer: for i in PRECOMPUTED_ROOK_MOVES[x][y][diagonal].iter() {
        if let Some((cx, cy)) = *i {
            if !board.board[cx][cy].is_empty() {
                break;
            }
            if cx == king_x {
                diagonal = if king_y > y {2} else {3};
                for j in PRECOMPUTED_ROOK_MOVES[cx][cy][diagonal].iter() {
                    if let Some((cxx, cyy)) = *j {
                        if cyy == king_y {
                            vector.push(Move::RookMove(x, y, king_x, y, Square::E));
                            break 'outer;
                        }
                        if !board.board[cxx][cyy].is_empty() {
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    diagonal = if king_y > y {2} else {3};
    'outer: for i in PRECOMPUTED_ROOK_MOVES[x][y][diagonal].iter() {
        if let Some((cx, cy)) = *i {
            if !board.board[cx][cy].is_empty() {
                break;
            }
            if cy == king_y {
                diagonal = if king_x > x {0} else {1};
                for j in PRECOMPUTED_ROOK_MOVES[cx][cy][diagonal].iter() {
                    if let Some((cxx, cyy)) = *j {
                        if cyy == king_y {
                            vector.push(Move::RookMove(x, y, x, king_y, Square::E));
                            break 'outer;
                        }
                        if !board.board[cxx][cyy].is_empty() {
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
}


pub fn generate_queen_moves(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    for i in PRECOMPUTED_BISHOP_MOVES[x][y].iter() {
        for j in i {
            if let Some((cx, cy)) = j {
                match board.board[*cx][*cy].can_move_to_and_through(board.white_to_move) {
                    (true, true) => vector.push(Move::QueenMove(x, y, *cx, *cy, Square::E)),
                    (true, false) => {vector.push(Move::QueenMove(x, y, *cx, *cy, board.board[*cx][*cy])); break},
                    _ => break
                }
            } else {
                break
            }
        }
    }
    for i in PRECOMPUTED_ROOK_MOVES[x][y].iter() {
        for j in i {
            if let Some((cx, cy)) = j {
                match board.board[*cx][*cy].can_move_to_and_through(board.white_to_move) {
                    (true, true) => vector.push(Move::QueenMove(x, y, *cx, *cy, Square::E)),
                    (true, false) => {vector.push(Move::QueenMove(x, y, *cx, *cy, board.board[*cx][*cy])); break},
                    _ => break
                }
            } else {
                break
            }
        }
    }
}

pub fn generate_queen_captures(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    for i in PRECOMPUTED_BISHOP_MOVES[x][y].iter() {
        for j in i {
            if let Some((cx, cy)) = j {
                match board.board[*cx][*cy].can_move_to_and_through(board.white_to_move) {
                    (true, true) => continue,
                    (true, false) => {vector.push(Move::QueenMove(x, y, *cx, *cy, board.board[*cx][*cy])); break},
                    _ => break
                }
            } else {
                break
            }
        }
    }
    for i in PRECOMPUTED_ROOK_MOVES[x][y].iter() {
        for j in i {
            if let Some((cx, cy)) = j {
                match board.board[*cx][*cy].can_move_to_and_through(board.white_to_move) {
                    (true, true) => continue,
                    (true, false) => {vector.push(Move::QueenMove(x, y, *cx, *cy, board.board[*cx][*cy])); break},
                    _ => break
                }
            } else {
                break
            }
        }
    }
}


pub fn generate_queen_checks(board: &Board, king_x: usize, king_y: usize, x: usize, y: usize, vector: &mut Vec<Move>) {
    let mut diagonal;
    'outer: for i in PRECOMPUTED_BISHOP_CHECKS[x][y][king_x][king_y].iter() {
        if let Some((a, b)) = *i {
            diagonal = match (a > x, b > y) {
                (true, true) => 0,
                (true, false) => 1,
                (false, true) => 2,
                (false, false) => 3
            };
            for j in PRECOMPUTED_BISHOP_MOVES[x][y][diagonal].iter() {
                if let Some((cx, cy)) = *j {
                    if !board.board[cx][cy].is_empty() {
                        continue 'outer;
                    }
                    if cx == a {
                        break;
                    }
                }
            }
            diagonal = match (a > king_x, b > king_y) {
                (true, true) => 0,
                (true, false) => 1,
                (false, true) => 2,
                (false, false) => 3
            };
            for j in PRECOMPUTED_BISHOP_MOVES[king_x][king_y][diagonal].iter() {
                if let Some((cx, cy)) = *j {
                    if cx == a {
                        break;
                    }
                    if !board.board[cx][cy].is_empty() {
                        continue 'outer;
                    }
                }
            }
            vector.push(Move::QueenMove(x, y, a, b, Square::E));
        } else {
            break;
        }
    }
    if king_x == x || king_y == y {
        return;
    }
    diagonal = if king_x > x {0} else {1};
    'outer: for i in PRECOMPUTED_ROOK_MOVES[x][y][diagonal].iter() {
        if let Some((cx, cy)) = *i {
            if !board.board[cx][cy].is_empty() {
                break;
            }
            if cx == king_x {
                diagonal = if king_y > y {2} else {3};
                for j in PRECOMPUTED_ROOK_MOVES[cx][cy][diagonal].iter() {
                    if let Some((cxx, cyy)) = *j {
                        if cyy == king_y {
                            vector.push(Move::QueenMove(x, y, king_x, y, Square::E));
                            break 'outer;
                        }
                        if !board.board[cxx][cyy].is_empty() {
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    diagonal = if king_y > y {2} else {3};
    'outer: for i in PRECOMPUTED_ROOK_MOVES[x][y][diagonal].iter() {
        if let Some((cx, cy)) = *i {
            if !board.board[cx][cy].is_empty() {
                break;
            }
            if cy == king_y {
                diagonal = if king_x > x {0} else {1};
                for j in PRECOMPUTED_ROOK_MOVES[cx][cy][diagonal].iter() {
                    if let Some((cxx, cyy)) = *j {
                        if cyy == king_y {
                            vector.push(Move::QueenMove(x, y, x, king_y, Square::E));
                            break 'outer;
                        }
                        if !board.board[cxx][cyy].is_empty() {
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
}

pub fn generate_king_moves(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    let white_black = if board.white_to_move {1} else {0};
    for i in PRECOMPUTED_BISHOP_MOVES[x][y].iter() {
        if let Some((cx, cy)) = i[0] {
            if board.visibility.map[cx][cy].total[white_black].is_empty() && board.board[cx][cy].can_move_to(board.white_to_move) {
                vector.push(Move::KingMove(x, y, cx, cy, board.board[cx][cy]));
            }
        }
    }
    for i in PRECOMPUTED_ROOK_MOVES[x][y].iter() {
        if let Some((cx, cy)) = i[0] {
            if board.visibility.map[cx][cy].total[white_black].is_empty() && board.board[cx][cy].can_move_to(board.white_to_move) {
                vector.push(Move::KingMove(x, y, cx, cy, board.board[cx][cy]));
            }
        }
    }
    if board.white_to_move {
        if board.king_moved[0] == 0 && !board.in_check() {
            if board.rook_moved[0] == 0 && board.can_white_castle_kingside() {
                vector.push(Move::Castling(7, 4, 7, 6));
            }
            if board.rook_moved[1] == 0 && board.can_white_castle_queenside() {
                vector.push(Move::Castling(7, 4, 7, 2));
            }
        }
    } else {
        if board.king_moved[1] == 0 && !board.in_check() {
            if board.rook_moved[2] == 0 && board.can_black_castle_kingside() {
                vector.push(Move::Castling(0, 4, 0, 6));
            }
            if board.rook_moved[3] == 0 && board.can_black_castle_queenside() {
                vector.push(Move::Castling(0, 4, 0, 2));
            }
        }
    }
}


pub fn generate_king_captures(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    for i in PRECOMPUTED_BISHOP_MOVES[x][y].iter() {
        if let Some((cx, cy)) = i[0] {
            if board.board[cx][cy].can_capture(board.white_to_move) {
                vector.push(Move::KingMove(x, y, cx, cy, board.board[cx][cy]));
            }
        }
    }
    for i in PRECOMPUTED_ROOK_MOVES[x][y].iter() {
        if let Some((cx, cy)) = i[0] {
            if board.board[cx][cy].can_capture(board.white_to_move) {
                vector.push(Move::KingMove(x, y, cx, cy, board.board[cx][cy]));
            }
        }
    }
}


pub fn generate_pawn_moves(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    let cx;
    let mut cy;
    if board.white_to_move {
        match x {
            // promoting
            1 => {
                if board.board[0][y].is_empty() {
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::QW));
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::RW));
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::BW));
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::NW));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[0][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::QW));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::RW));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::BW));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::NW));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[0][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::QW));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::RW));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::BW));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::NW));
                    }
                }
            }
            // en passant
            3 => {
                if board.board[2][y].is_empty() {
                    vector.push(Move::PawnPush(x, y, 2, y));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[2][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnCapture(x, y, 2, cy, board.board[2][cy]));
                    } else if board.en_passant_array[2][cy] == board.clock {
                        vector.push(Move::EnPassant(x, y, 2, cy, board.board[3][cy]));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[2][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnCapture(x, y, 2, cy, board.board[2][cy]));
                    } else if board.en_passant_array[2][cy] == board.clock {
                        vector.push(Move::EnPassant(x, y, 2, cy, board.board[3][cy]));
                    }
                }
            }
            // two squares
            6 => {
                if board.board[5][y].is_empty() {
                    if board.board[4][y].is_empty() {
                        vector.push(Move::TwoSquarePawnMove(x, y, 4, y));
                    }
                    vector.push(Move::PawnPush(x, y, 5, y));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[5][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnCapture(x, y, 5, cy, board.board[5][cy]));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[5][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnCapture(x, y, 5, cy, board.board[5][cy]));
                    }
                }
            }
            // else
            _ => {
                cx = x - 1;
                if board.board[cx][y].is_empty() {
                    vector.push(Move::PawnPush(x, y, cx, y));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[cx][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnCapture(x, y, cx, cy, board.board[cx][cy]));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[cx][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnCapture(x, y, cx, cy, board.board[cx][cy]));
                    }
                }
            }
        }
    } else {
        match x {
            // promoting
            6 => {
                if board.board[7][y].is_empty() {
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::QB));
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::RB));
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::BB));
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::NB));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[7][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::QB));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::RB));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::BB));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::NB));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[7][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::QB));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::RB));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::BB));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::NB));
                    }
                }
            }
            // en passant
            4 => {
                if board.board[5][y].is_empty() {
                    vector.push(Move::PawnPush(x, y, 5, y));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[5][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnCapture(x, y, 5, cy, board.board[5][cy]));
                    } else if board.en_passant_array[5][cy] == board.clock {
                        vector.push(Move::EnPassant(x, y, 5, cy, board.board[4][cy]));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[5][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnCapture(x, y, 5, cy, board.board[5][cy]));
                    } else if board.en_passant_array[5][cy] == board.clock {
                        vector.push(Move::EnPassant(x, y, 5, cy, board.board[4][cy]));
                    }
                }
            }
            // two squares
            1 => {
                if board.board[2][y].is_empty() {
                    if board.board[3][y].is_empty() {
                        vector.push(Move::TwoSquarePawnMove(x, y, 3, y));
                    }
                    vector.push(Move::PawnPush(x, y, 2, y));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[2][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnCapture(x, y, 2, cy, board.board[2][cy]));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[2][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnCapture(x, y, 2, cy, board.board[2][cy]));
                    }
                }
            }
            // else
            _ => {
                cx = x + 1;
                if board.board[cx][y].is_empty() {
                    vector.push(Move::PawnPush(x, y, cx, y));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[cx][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnCapture(x, y, cx, cy, board.board[cx][cy]));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[cx][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnCapture(x, y, cx, cy, board.board[cx][cy]));
                    }
                }
            }
        }
    }
}


pub fn generate_pawn_captures(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    // promotions are captures fuck you
    let cx;
    let mut cy;
    if board.white_to_move {
        match x {
            // promoting
            1 => {
                if board.board[0][y].is_empty() {
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::QW));
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::RW));
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::BW));
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::NW));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[0][cy].can_capture(true) {
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::QW));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::RW));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::BW));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::NW));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[0][cy].can_capture(true) {
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::QW));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::RW));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::BW));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::NW));
                    }
                }
            }
            // en passant
            3 => {
                if y > 0 {
                    cy = y - 1;
                    if board.board[2][cy].can_capture(true) {
                        vector.push(Move::PawnCapture(x, y, 2, cy, board.board[2][cy]));
                    } else if board.en_passant_array[2][cy] == board.clock {
                        vector.push(Move::EnPassant(x, y, 2, cy, board.board[3][cy]));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[2][cy].can_capture(true) {
                        vector.push(Move::PawnCapture(x, y, 2, cy, board.board[2][cy]));
                    } else if board.en_passant_array[2][cy] == board.clock {
                        vector.push(Move::EnPassant(x, y, 2, cy, board.board[3][cy]));
                    }
                }
            }
            // else
            _ => {
                cx = x - 1;
                if y > 0 {
                    cy = y - 1;
                    if board.board[cx][cy].can_capture(true) {
                        vector.push(Move::PawnCapture(x, y, cx, cy, board.board[cx][cy]));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[cx][cy].can_capture(true) {
                        vector.push(Move::PawnCapture(x, y, cx, cy, board.board[cx][cy]));
                    }
                }
            }
        }
    } else {
        match x {
            // promoting
            6 => {
                if board.board[7][y].is_empty() {
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::QB));
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::RB));
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::BB));
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::NB));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[7][cy].can_capture(false) {
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::QB));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::RB));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::BB));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::NB));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[7][cy].can_capture(false) {
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::QB));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::RB));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::BB));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::NB));
                    }
                }
            }
            // en passant
            4 => {
                if y > 0 {
                    cy = y - 1;
                    if board.board[5][cy].can_capture(false) {
                        vector.push(Move::PawnCapture(x, y, 5, cy, board.board[5][cy]));
                    } else if board.en_passant_array[5][cy] == board.clock {
                        vector.push(Move::EnPassant(x, y, 5, cy, board.board[4][cy]));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[5][cy].can_capture(false) {
                        vector.push(Move::PawnCapture(x, y, 5, cy, board.board[5][cy]));
                    } else if board.en_passant_array[5][cy] == board.clock {
                        vector.push(Move::EnPassant(x, y, 5, cy, board.board[4][cy]));
                    }
                }
            }
            // else
            _ => {
                cx = x + 1;
                if y > 0 {
                    cy = y - 1;
                    if board.board[cx][cy].can_capture(false) {
                        vector.push(Move::PawnCapture(x, y, cx, cy, board.board[cx][cy]));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[cx][cy].can_capture(false) {
                        vector.push(Move::PawnCapture(x, y, cx, cy, board.board[cx][cy]));
                    }
                }
            }
        }
    }
}


pub fn can_attack_square(board: &Board, x: usize, y: usize, white_black: usize) -> bool {
    // looking for black pieces to attack white and white pieces to attack black
    board.visibility.map[x][y].total[white_black].len() > 0
}


pub fn generate_defenders_into(board: &Board, vector: &mut Vec<Move>, x: usize, y: usize) {
    if board.white_to_move {
        for (cx, cy) in board.visibility.map[x][y].total[0].iter() {
            match board.board[*cx][*cy] {
                Square::PW if x == 0 => { vector.push(Move::PawnPromotion(1, *cy, 0, y, board.board[x][y], Square::QW)); },
                Square::PW => { vector.push(Move::PawnCapture(*cx, *cy, x, y, board.board[x][y])); },
                Square::NW => { vector.push(Move::KnightMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::BW => { vector.push(Move::BishopMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::RW => { vector.push(Move::RookMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::QW => { vector.push(Move::QueenMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::KW => { vector.push(Move::KingMove(*cx, *cy, x, y, board.board[x][y])); },
                _ => panic!("inside generate_defenders_into sus visibility: ({}, {}) ({}, {}) {:?}", x, y, cx, cy, board.board[*cx][*cy])
            }
        }
    } else {
        for (cx, cy) in board.visibility.map[x][y].total[1].iter() {
            match board.board[*cx][*cy] {
                Square::PB if x == 7 => { vector.push(Move::PawnPromotion(6, *cy, 7, y, board.board[x][y], Square::QB)); },
                Square::PB => { vector.push(Move::PawnCapture(*cx, *cy, x, y, board.board[x][y])); },
                Square::NB => { vector.push(Move::KnightMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::BB => { vector.push(Move::BishopMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::RB => { vector.push(Move::RookMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::QB => { vector.push(Move::QueenMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::KB => { vector.push(Move::KingMove(*cx, *cy, x, y, board.board[x][y])); },
                _ => panic!("inside generate_defenders_into sus visibility: ({}, {}) ({}, {}) {:?} -> {:?}\nfen: {}", cx, cy, x, y, board.board[*cx][*cy], board.board[x][y], board.to_fen())
                // other => panic!("inside generate_defenders_into sus visibility: {:?} ({}, {})", other, cx, cy)
            }
        }
    }
}

pub fn generate_blockers_into(board: &Board, vector: &mut Vec<Move>, x: usize, y: usize) {
    // ASSUMES TARGET SQUARE IS EMPTY
    if board.white_to_move {
        for (cx, cy) in board.visibility.map[x][y].total[0].iter() {
            match board.board[*cx][*cy] {
                Square::NW => { vector.push(Move::KnightMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::BW => { vector.push(Move::BishopMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::RW => { vector.push(Move::RookMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::QW => { vector.push(Move::QueenMove(*cx, *cy, x, y, board.board[x][y])); },
                _ => ()
            }
        }
        if x < 7 && board.board[x + 1][y] == Square::PW {
            vector.push(Move::PawnPush(x + 1, y, x, y));
        } else if x == 4 && board.board[6][y] == Square::PW && board.board[5][y].is_empty() {
            vector.push(Move::TwoSquarePawnMove(6, y, 4, y));
        }
    } else {
        for (cx, cy) in board.visibility.map[x][y].total[1].iter() {
            match board.board[*cx][*cy] {
                Square::NB => { vector.push(Move::KnightMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::BB => { vector.push(Move::BishopMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::RB => { vector.push(Move::RookMove(*cx, *cy, x, y, board.board[x][y])); },
                Square::QB => { vector.push(Move::QueenMove(*cx, *cy, x, y, board.board[x][y])); },
                _ => ()
            }
        }
        if x > 0 && board.board[x - 1][y] == Square::PB {
            vector.push(Move::PawnPush(x - 1, y, x, y));
        } else if x == 3 && board.board[1][y] == Square::PB && board.board[2][y].is_empty() {
            vector.push(Move::TwoSquarePawnMove(1, y, 3, y));
        }
    }
}