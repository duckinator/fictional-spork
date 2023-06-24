pub type Symbol = [u8; 8 * 8];

pub const CHAR_IDX_OFFSET: usize = ' ' as usize;

/*
const CHAR_LIST: [char; 95] = [
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?',
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_',
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~',
];
*/

#[rustfmt::skip]
const MISSING_SYMBOL: Symbol = [
    //1,2,3,4,5,6,7
    0,1,0,1,0,1,0,1,
    1,0,1,0,1,0,1,0,
    0,1,0,1,0,1,0,1,
    1,0,1,0,1,0,1,0,
    0,1,0,1,0,1,0,1,
    1,0,1,0,1,0,1,0,
    0,1,0,1,0,1,0,1,
    1,0,1,0,1,0,1,0,
];

#[rustfmt::skip]
pub const CHARS: [Symbol; 95] = [
    // Space
    [0; 64],

    // !
    [
        //1,2,3,4,5,6,7
        0,0,1,0,0,0,0,0,
        0,0,1,0,0,0,0,0,
        0,0,1,0,0,0,0,0,
        0,0,1,0,0,0,0,0,
        0,0,1,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,0,1,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
    ],

    // "
    MISSING_SYMBOL,

    // #
    MISSING_SYMBOL,

    // $
    MISSING_SYMBOL,

    // %
    MISSING_SYMBOL,

    // &
    MISSING_SYMBOL,

    // '
    MISSING_SYMBOL,

    // (
    MISSING_SYMBOL,

    // )
    MISSING_SYMBOL,

    // *
    MISSING_SYMBOL,

    // +
    MISSING_SYMBOL,

    // ,
    [
        //1,2,3,4,5,6,7
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,0,0,1,0,0,0,0,
        0,0,1,0,0,0,0,0,
    ],

    // -
    MISSING_SYMBOL,

    // .
    MISSING_SYMBOL,

    // /
    MISSING_SYMBOL,

    // 0
    MISSING_SYMBOL,

    // 1
    MISSING_SYMBOL,

    // 2
    MISSING_SYMBOL,

    // 3
    MISSING_SYMBOL,

    // 4
    MISSING_SYMBOL,

    // 5
    MISSING_SYMBOL,

    // 6
    MISSING_SYMBOL,

    // 7
    MISSING_SYMBOL,

    // 8
    MISSING_SYMBOL,

    // 9
    MISSING_SYMBOL,

    // :
    MISSING_SYMBOL,

    // ;
    MISSING_SYMBOL,

    // >
    MISSING_SYMBOL,

    // =
    MISSING_SYMBOL,

    // >
    MISSING_SYMBOL,

    // ?
    MISSING_SYMBOL,

    // @
    MISSING_SYMBOL,

    // A
    [
     //1 2 3 4 5 6 7
     0,1,1,1,1,1,0,0,
     0,1,0,0,0,1,0,0,
     0,1,0,0,0,1,0,0,
     0,1,0,0,0,1,0,0,
     0,1,1,1,1,1,0,0,
     0,1,0,0,0,1,0,0,
     0,1,0,0,0,1,0,0,
     0,0,0,0,0,0,0,0,
    ],

    // B
    MISSING_SYMBOL,

    // C
    MISSING_SYMBOL,

    // D
    MISSING_SYMBOL,

    // E
    MISSING_SYMBOL,

    // F
    MISSING_SYMBOL,

    // G
    MISSING_SYMBOL,

    // H
    [
        //1,2,3,4,5,6,7
        0,1,0,0,0,1,0,0,
        0,1,0,0,0,1,0,0,
        0,1,0,0,0,1,0,0,
        0,1,0,0,0,1,0,0,
        0,1,1,1,1,1,0,0,
        0,1,0,0,0,1,0,0,
        0,1,0,0,0,1,0,0,
        0,0,0,0,0,0,0,0,
    ],

    // I
    MISSING_SYMBOL,

    // J
    MISSING_SYMBOL,

    // K
    MISSING_SYMBOL,

    // L
    MISSING_SYMBOL,

    // M
    MISSING_SYMBOL,

    // N
    MISSING_SYMBOL,

    // O
    MISSING_SYMBOL,

    // P
    MISSING_SYMBOL,

    // Q
    MISSING_SYMBOL,

    // R
    MISSING_SYMBOL,

    // S
    MISSING_SYMBOL,

    // T
    MISSING_SYMBOL,

    // U
    MISSING_SYMBOL,

    // V
    MISSING_SYMBOL,

    // W
    MISSING_SYMBOL,

    // X
    MISSING_SYMBOL,

    // Y
    MISSING_SYMBOL,

    // Z
    MISSING_SYMBOL,

    // [
    MISSING_SYMBOL,

    // \
    MISSING_SYMBOL,

    // ]
    MISSING_SYMBOL,

    // ^
    MISSING_SYMBOL,

    // _
    MISSING_SYMBOL,

    // `
    MISSING_SYMBOL,

    // a
    MISSING_SYMBOL,

    // b
    MISSING_SYMBOL,

    // c
    MISSING_SYMBOL,

    // d
    [
        //1,2,3,4,5,6,7
        0,0,0,0,0,1,0,0,
        0,0,0,0,0,1,0,0,
        0,0,1,1,1,1,0,0,
        0,1,0,0,0,1,0,0,
        0,1,0,0,0,1,0,0,
        0,1,0,0,0,1,0,0,
        0,0,1,1,1,1,0,0,
        0,0,0,0,0,0,0,0,
    ],

    // e
    [
        //1,2,3,4,5,6,7
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,0,1,1,1,0,0,0,
        0,1,0,0,0,1,0,0,
        0,1,1,1,1,0,0,0,
        0,1,0,0,0,0,0,0,
        0,0,1,1,1,1,0,0,
        0,0,0,0,0,0,0,0,
    ],

    // f
    MISSING_SYMBOL,

    // g
    MISSING_SYMBOL,

    // h
    MISSING_SYMBOL,

    // i
    MISSING_SYMBOL,

    // j
    MISSING_SYMBOL,

    // k
    MISSING_SYMBOL,

    // l
    [
        //1,2,3,4,5,6,7
        0,0,1,0,0,0,0,0,
        0,0,1,0,0,0,0,0,
        0,0,1,0,0,0,0,0,
        0,0,1,0,0,0,0,0,
        0,0,1,0,0,0,0,0,
        0,0,1,0,0,0,0,0,
        0,0,1,1,0,0,0,0,
        0,0,0,0,0,0,0,0,
    ],

    // m
    MISSING_SYMBOL,

    // n
    MISSING_SYMBOL,


    // o
    [
        //1,2,3,4,5,6,7
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,0,0,1,1,0,0,0,
        0,0,1,0,0,1,0,0,
        0,0,1,0,0,1,0,0,
        0,0,1,0,0,1,0,0,
        0,0,0,1,1,0,0,0,
        0,0,0,0,0,0,0,0,
    ],

    // p
    MISSING_SYMBOL,

    // q
    MISSING_SYMBOL,

    // r
    [
        //1,2,3,4,5,6,7
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,1,0,1,1,0,0,0,
        0,1,1,0,0,1,0,0,
        0,1,0,0,0,0,0,0,
        0,1,0,0,0,0,0,0,
        0,1,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
    ],

    // s
    MISSING_SYMBOL,

    // t
    MISSING_SYMBOL,

    // u
    MISSING_SYMBOL,

    // v
    MISSING_SYMBOL,

    // w
    [
        //1,2,3,4,5,6,7
        0,0,0,0,0,0,0,0,
        0,0,0,0,0,0,0,0,
        0,1,0,1,0,1,0,0,
        0,1,0,1,0,1,0,0,
        0,1,0,1,0,1,0,0,
        0,1,0,1,0,1,0,0,
        0,1,1,0,1,1,0,0,
        0,0,0,0,0,0,0,0,
    ],

    // x
    MISSING_SYMBOL,

    // y
    MISSING_SYMBOL,

    // z
    MISSING_SYMBOL,

    // {
    MISSING_SYMBOL,

    // |
    MISSING_SYMBOL,

    // }
    MISSING_SYMBOL,

    // ~
    MISSING_SYMBOL,
];
