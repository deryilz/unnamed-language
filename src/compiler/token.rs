#[derive(Debug, Clone)]
pub enum TokenKind {
    Invalid,
    End,
    WhiteSpace,
    LineBreak,
    Comment,
    Identifier,
    Struct,
    Union,
    Trait,
    Or,
    And,
    Colon,
    DoubleColon,
    Dot,
    ParenL,
    ParenR,
    SquareL,
    SquareR,
    CurlyL,
    CurlyR,
    At,
    Hashtag,
    Plus,
    DoublePlus,
    Equals,
    DoubleEquals,
    ThinArrow,
    ThickArrow,
    Comma,
    Minus,
    Times,
    Div,
    String,
    Char,
    Int,
    Float,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Token {
        Token { kind, start, end }
    }
}
