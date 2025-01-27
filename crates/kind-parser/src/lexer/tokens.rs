//! Describes all of the tokens required
//! to parse kind2 after version 0.2.8.

use core::fmt;

use crate::diagnostic::SyntaxDiagnostic;

#[derive(Debug, Clone)]
pub enum Token {
    LPar,       // (
    RPar,       // )
    LBracket,   // [
    RBracket,   // ]
    LBrace,     // {
    RBrace,     // }
    Eq,         // =
    Colon,      // :
    Semi,       // ;
    FatArrow,   // =>
    Dollar,     // $
    Comma,      // ,
    RightArrow, // ->
    DotDot,     // ..
    Dot,        // .
    Tilde,      // ~
    ColonColon, // ::

    Help(String),
    LowerId(String),
    UpperId(String, Option<String>),

    // Strong keywords because they lead to better
    // error messages.
    Return,
    Ask,
    With,

    // Keywords
    // Do,
    // If,
    // Else,
    // Match,
    // Let,
    // Type,
    // Record,
    // Constructor,
    // Use,
    // As,

    // Literals
    Char(char),
    Str(String),
    Num60(u64),
    Num120(u128),
    Nat(u128),
    Float(u64, u64),
    Hole,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    Bar,
    Hat,
    GreaterGreater,
    LessLess,
    Less,
    LessEq,
    EqEq,
    GreaterEq,
    Greater,
    BangEq,
    Bang,

    HashHash,
    Hash,

    Comment(bool, String),

    Eof,

    // The error token that is useful to error recovery.
    Error(Box<SyntaxDiagnostic>),
}

impl Token {
    pub fn same_variant(&self, b: &Token) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(b)
    }

    pub fn is_lower_id(&self) -> bool {
        matches!(self, Token::LowerId(_))
    }

    pub fn is_doc(&self) -> bool {
        matches!(self, Token::Comment(true, _))
    }

    pub fn is_upper_id(&self) -> bool {
        matches!(self, Token::UpperId(_, _))
    }

    pub fn is_str(&self) -> bool {
        matches!(self, Token::Str(_))
    }

    pub fn is_num60(&self) -> bool {
        matches!(self, Token::Num60(_))
    }

    pub fn is_num120(&self) -> bool {
        matches!(self, Token::Num120(_))
    }

    pub fn is_char(&self) -> bool {
        matches!(self, Token::Char(_))
    }

    pub fn is_eof(&self) -> bool {
        matches!(self, Token::Eof)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LPar => write!(f, "("),
            Token::RPar => write!(f, ")"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Eq => write!(f, "="),
            Token::Colon => write!(f, ":"),
            Token::Semi => write!(f, ";"),
            Token::FatArrow => write!(f, "=>"),
            Token::Dollar => write!(f, "$"),
            Token::Comma => write!(f, ","),
            Token::RightArrow => write!(f, "->"),
            Token::DotDot => write!(f, ".."),
            Token::Dot => write!(f, "."),
            Token::Tilde => write!(f, "~"),
            Token::ColonColon => write!(f, "::"),
            Token::Help(text) => write!(f, "?{}", text),
            Token::LowerId(id) => write!(f, "{}", id),
            Token::UpperId(main, Some(aux)) => write!(f, "{}/{}", main, aux),
            Token::UpperId(main, None) => write!(f, "{}", main),
            Token::Char(c) => write!(f, "'{}'", c),
            Token::Str(s) => write!(f, "\"{}\"", s),
            Token::Num60(n) => write!(f, "{}", n),
            Token::Num120(n) => write!(f, "{}u120", n),
            Token::Nat(n) => write!(f, "{}n", n),
            Token::Float(start, end) => write!(f, "{}.{}", start, end),
            Token::Hole => write!(f, "_"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Ampersand => write!(f, "&"),
            Token::Bar => write!(f, "|"),
            Token::Hat => write!(f, "^"),
            Token::GreaterGreater => write!(f, ">>"),
            Token::LessLess => write!(f, "<<"),
            Token::Less => write!(f, "<"),
            Token::LessEq => write!(f, "<="),
            Token::EqEq => write!(f, "=="),
            Token::GreaterEq => write!(f, ">="),
            Token::Greater => write!(f, ">"),
            Token::BangEq => write!(f, "!="),
            Token::Bang => write!(f, "!"),
            Token::HashHash => write!(f, "##"),
            Token::Hash => write!(f, "#"),
            Token::Comment(true, comment) => write!(f, "docstring '{comment}'"),
            Token::Comment(false, comment) => write!(f, "comment '{comment}'"),
            Token::Eof => write!(f, "End of file"),
            Token::Error(_) => write!(f, "ERROR"),
            Token::With => write!(f, "with"),
            Token::Return => write!(f, "return"),
            Token::Ask => write!(f, "ask"),
        }
    }
}
