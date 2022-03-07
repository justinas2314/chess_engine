use crate::utils::{Board, Move, Square};

// TODO can sort the precomputed moves to be able to break
static PRECOMPUTED_KNIGHT_MOVES: [[[Option<(usize, usize)>; 8]; 8]; 8] =
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


static PRECOMPUTED_BISHOP_MOVES: [[[[Option<(usize, usize)>; 7]; 4]; 8]; 8] =
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


static PRECOMPUTED_ROOK_MOVES: [[[[Option<(usize, usize)>; 7]; 4]; 8]; 8] =
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


pub fn generate_knight_moves(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    for i in PRECOMPUTED_KNIGHT_MOVES[x][y].iter() {
        if let Some((a, b)) = i {
            if board.board[*a][*b].can_move_to(board.white_to_move) {
                vector.push(Move::KnightMove(x, y, *a, *b, board.board[*a][*b]))
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
                vector.push(Move::KnightMove(x, y, *a, *b, board.board[*a][*b]))
            }
        } else {
            return;
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


pub fn generate_king_moves(board: &Board, x: usize, y: usize, vector: &mut Vec<Move>) {
    for i in PRECOMPUTED_BISHOP_MOVES[x][y].iter() {
        if let Some((cx, cy)) = i[0] {
            if board.board[cx][cy].can_move_to(board.white_to_move) {
                vector.push(Move::KingMove(x, y, cx, cy, board.board[cx][cy]));
            }
        }
    }
    for i in PRECOMPUTED_ROOK_MOVES[x][y].iter() {
        if let Some((cx, cy)) = i[0] {
            if board.board[cx][cy].can_move_to(board.white_to_move) {
                vector.push(Move::KingMove(x, y, cx, cy, board.board[cx][cy]));
            }
        }
    }
    if board.white_to_move {
        if board.king_moved[0] == 0 {
            if board.rook_moved[0] == 0 && board.can_castle(7, 5, 6) {
                vector.push(Move::Castling(7, 4, 7, 6));
            }
            if board.rook_moved[1] == 0 && board.can_castle(7, 3, 2) {
                vector.push(Move::Castling(7, 4, 7, 2));
            }
        }
    } else {
        if board.king_moved[1] == 0 {
            if board.rook_moved[2] == 0 && board.can_castle(0, 5, 6) {
                vector.push(Move::Castling(0, 4, 0, 6));
            }
            if board.rook_moved[3] == 0 && board.can_castle(0, 3, 2) {
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
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::Q { white: true }));
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::R { white: true }));
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::B { white: true }));
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::N { white: true }));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[0][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::Q { white: true }));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::R { white: true }));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::B { white: true }));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::N { white: true }));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[0][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::Q { white: true }));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::R { white: true }));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::B { white: true }));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::N { white: true }));
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
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::Q { white: false }));
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::R { white: false }));
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::B { white: false }));
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::N { white: false }));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[7][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::Q { white: false }));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::R { white: false }));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::B { white: false }));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::N { white: false }));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[7][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::Q { white: false }));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::R { white: false }));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::B { white: false }));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::N { white: false }));
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
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::Q { white: true }));
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::R { white: true }));
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::B { white: true }));
                    vector.push(Move::PawnPromotion(x, y, 0, y, Square::E, Square::N { white: true }));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[0][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::Q { white: true }));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::R { white: true }));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::B { white: true }));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::N { white: true }));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[0][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::Q { white: true }));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::R { white: true }));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::B { white: true }));
                        vector.push(Move::PawnPromotion(x, y, 0, cy, board.board[0][cy], Square::N { white: true }));
                    }
                }
            }
            // en passant
            3 => {
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
            // else
            _ => {
                cx = x - 1;
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
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::Q { white: false }));
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::R { white: false }));
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::B { white: false }));
                    vector.push(Move::PawnPromotion(x, y, 7, y, Square::E, Square::N { white: false }));
                }
                if y > 0 {
                    cy = y - 1;
                    if board.board[7][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::Q { white: false }));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::R { white: false }));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::B { white: false }));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::N { white: false }));
                    }
                }
                if y < 7 {
                    cy = y + 1;
                    if board.board[7][cy].can_capture(board.white_to_move) {
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::Q { white: false }));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::R { white: false }));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::B { white: false }));
                        vector.push(Move::PawnPromotion(x, y, 7, cy, board.board[7][cy], Square::N { white: false }));
                    }
                }
            }
            // en passant
            4 => {
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
            // else
            _ => {
                cx = x + 1;
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

pub fn can_attack_square(board: &Board, x: usize, y: usize, white: bool) -> bool {
    for i in PRECOMPUTED_BISHOP_MOVES[x][y].iter() {
        for j in i {
            if let Some((cx, cy)) = j {
                match board.board[*cx][*cy].bishopy_enemy(white) {
                    0 => break,
                    1 => continue,
                    _ => return true
                }
            } else {
                break
            }
        }
    }
    for i in PRECOMPUTED_ROOK_MOVES[x][y].iter() {
        for j in i {
            if let Some((cx, cy)) = j {
                match board.board[*cx][*cy].rooky_enemy(white) {
                    0 => break,
                    1 => continue,
                    _ => return true
                }
            } else {
                break
            }
        }
    }
    for i in PRECOMPUTED_KNIGHT_MOVES[x][y].iter() {
        if let Some((a, b)) = i {
            match board.board[*a][*b] {
                Square::N { white } if white != board.white_to_move => return true,
                _ => continue
            }
        } else {
            return false;
        }
    }
    false
}


pub fn generate_defenders_into(board: &Board, vector: &mut Vec<Move>, x: usize, y: usize) {
    vector.clear();
    for i in PRECOMPUTED_BISHOP_MOVES[x][y].iter() {
        if let Some((cx, cy)) = i[0] {
            match board.board[cx][cy] {
                Square::E => continue,
                Square::K{white} if white == board.white_to_move => { vector.push(Move::KingMove(cx, cy, x, y, board.board[x][y])); break },
                Square::B{white} if white == board.white_to_move => { vector.push(Move::BishopMove(cx, cy, x, y, board.board[x][y])); break },
                Square::Q{white} if white == board.white_to_move => { vector.push(Move::QueenMove(cx, cy, x, y, board.board[x][y])); break },
                _ => break
            }
        }
        for j in &i[1..] {
            if let Some((cx, cy)) = j {
                match board.board[*cx][*cy] {
                    Square::E => continue,
                    Square::B{white} if white == board.white_to_move => { vector.push(Move::BishopMove(*cx, *cy, x, y, board.board[x][y])); break },
                    Square::Q{white} if white == board.white_to_move => { vector.push(Move::QueenMove(*cx, *cy, x, y, board.board[x][y])); break },
                    _ => break
                }
            } else {
                break
            }
        }
    }
    for i in PRECOMPUTED_ROOK_MOVES[x][y].iter() {
        if let Some((cx, cy)) = i[0] {
            match board.board[cx][cy] {
                Square::E => continue,
                Square::K{white} if white == board.white_to_move => { vector.push(Move::KingMove(cx, cy, x, y, board.board[x][y])); break },
                Square::R{white} if white == board.white_to_move => { vector.push(Move::RookMove(cx, cy, x, y, board.board[x][y])); break },
                Square::Q{white} if white == board.white_to_move => { vector.push(Move::QueenMove(cx, cy, x, y, board.board[x][y])); break },
                _ => break
            }
        }
        for j in &i[1..] {
            if let Some((cx, cy)) = j {
                match board.board[*cx][*cy] {
                    Square::E => continue,
                    Square::R{white} if white == board.white_to_move => { vector.push(Move::RookMove(*cx, *cy, x, y, board.board[x][y])); break },
                    Square::Q{white} if white == board.white_to_move => { vector.push(Move::QueenMove(*cx, *cy, x, y, board.board[x][y])); break },
                    _ => break
                }
            } else {
                break
            }
        }
    }
    for i in PRECOMPUTED_KNIGHT_MOVES[x][y].iter() {
        if let Some((cx, cy)) = i {
            match board.board[*cx][*cy] {
                Square::N { white } if white == board.white_to_move => { vector.push(Move::KnightMove(*cx, *cy, x, y, board.board[x][y])) },
                _ => continue
            }
        } else {
            break
        }
    }
    let cx;
    let mut cy;
    if board.white_to_move && x < 6 {
        cx = x + 1;
        if y > 0 {
            cy = y - 1;
            match board.board[cx][cy] {
                Square::P{white:true} => vector.push(Move::PawnCapture(cx, cy, x, y, board.board[x][y])),
                _ => ()
            }
        }
        if y < 7 {
            cy = y + 1;
            match board.board[cx][cy] {
                Square::P{white:true} => vector.push(Move::PawnCapture(cx, cy, x, y, board.board[x][y])),
                _ => ()
            }
        }
    } else if x > 1 {
        cx = x - 1;
        if y > 0 {
            cy = y - 1;
            match board.board[cx][cy] {
                Square::P{white:false} => vector.push(Move::PawnCapture(cx, cy, x, y, board.board[x][y])),
                _ => ()
            }
        }
        if y < 7 {
            cy = y + 1;
            match board.board[cx][cy] {
                Square::P{white:false} => vector.push(Move::PawnCapture(cx, cy, x, y, board.board[x][y])),
                _ => ()
            }
        }
    }
}