#[derive(Debug, PartialEq, Clone)]
pub enum NumberLiteral {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Clone)]
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
    Mut,            // @
    Colon,          // :
    Scope,          // ::
    Semicolon,      // ;
    RightArrow,     // ->
    Dot,            // .
    Range,          // ..
    RangeInclusive, // ..=
    Comma,          // ,
    Ampersand,      // &
    Star,           // *
    Pipe,           // |
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    LeftParen,      // (
    RightParen,     // )
    LessThan,       // <
    GreaterThan,    // >
    SingleQuote,    // '
    DoubleQuote,    // "
    Newline,        // Newline character

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
    Number(NumberLiteral), // e.g., 123, 45.67

                        // Other:
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Span,
}

impl Token {
    /// Create a new token with the given kind, start position, and length.
    pub fn new(kind: TokenKind, start: usize, len: usize) -> Self {
        Self {
            kind,
            position: Span {
                start,
                end: start + len - 1,
            },
        }
    }

    pub fn is_single_char_token(&self) -> bool {
        self.position.start == self.position.end
    }
}
