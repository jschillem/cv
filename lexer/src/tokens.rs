#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    // Keywords:
    Break,  // break
    Else,   // else
    End,    // end
    False,  // false
    Fun,    // fn
    For,    // for
    If,     // if
    In,     // in
    Loop,   // loop
    Patch,  // patch
    Record, // record
    Return, // return
    True,   // true
    Union,  // union
    When,   // when

    // Syntax:
    Colon,        // :
    Scope,        // ::
    RightArrow,   // ->
    Dot,          // .
    DoubleDot,    // ..
    Comma,        // ,
    Ampersand,    // &
    Star,         // *
    Semicolon,    // ;
    Pipe,         // |
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )
    LessThan,     // <
    GreaterThan,  // >

    // Operators:
    And,          // and
    Or,           // or
    Not,          // not
    Plus,         // +
    Minus,        // -
    Divide,       // /
    Modulo,       // %
    Equal,        // =
    DoubleEqual,  // ==
    NotEqual,     // !=
    LessEqual,    // <=
    GreaterEqual, // >=
    PlusEqual,    // +=
    MinusEqual,   // -=
    TimesEqual,   // *=
    DivideEqual,  // /=
    ModuloEqual,  // %=

    // Identifiers and literals:
    Identifier(String), // e.g., variable, function, and type names
    Number(String),     // e.g., 123, 45.67 (will be parsed later into actual numbers)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Span,
}
