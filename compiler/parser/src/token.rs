//! Different token definitions.
//! Loosely based on token.h from CPython source:
use num_bigint::BigInt;
use std::fmt;

/// Python source code can be tokenized in a sequence of these tokens.
#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Name {
        name: String,
    },
    Int {
        value: BigInt,
    },
    Float {
        value: f64,
    },
    Complex {
        real: f64,
        imag: f64,
    },
    String {
        value: String,
        kind: StringKind,
        triple_quoted: bool,
    },
    Newline,
    Indent,
    Dedent,
    StartModule,
    StartInteractive,
    StartExpression,
    EndOfFile,
    Lpar,
    Rpar,
    Lsqb,
    Rsqb,
    Colon,
    Comma,
    Comment(String),
    Semi,
    Plus,
    Minus,
    Star,
    Slash,
    Vbar,  // '|'
    Amper, // '&'
    Less,
    Greater,
    Equal,
    Dot,
    Percent,
    Lbrace,
    Rbrace,
    EqEqual,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Tilde,
    CircumFlex,
    LeftShift,
    RightShift,
    DoubleStar,
    DoubleStarEqual, // '**='
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    AmperEqual, // '&='
    VbarEqual,
    CircumflexEqual, // '^='
    LeftShiftEqual,
    RightShiftEqual,
    DoubleSlash, // '//'
    DoubleSlashEqual,
    ColonEqual,
    At,
    AtEqual,
    Rarrow,
    Ellipsis,

    // Keywords (alphabetically):
    False,
    None,
    True,

    And,
    As,
    Assert,
    Async,
    Await,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    Nonlocal,
    Not,
    Or,
    Pass,
    Raise,
    Return,
    Try,
    While,
    With,
    Yield,
}

impl fmt::Display for Tok {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Tok::*;
        match self {
            Name { name } => write!(f, "'{name}'"),
            Int { value } => write!(f, "'{value}'"),
            Float { value } => write!(f, "'{value}'"),
            Complex { real, imag } => write!(f, "{real}j{imag}"),
            String {
                value,
                kind,
                triple_quoted,
            } => {
                let quotes = "\"".repeat(if *triple_quoted { 3 } else { 1 });
                write!(f, "{kind}{quotes}{value}{quotes}")
            }
            Newline => f.write_str("Newline"),
            Indent => f.write_str("Indent"),
            Dedent => f.write_str("Dedent"),
            StartModule => f.write_str("StartProgram"),
            StartInteractive => f.write_str("StartInteractive"),
            StartExpression => f.write_str("StartExpression"),
            EndOfFile => f.write_str("EOF"),
            Lpar => f.write_str("'('"),
            Rpar => f.write_str("')'"),
            Lsqb => f.write_str("'['"),
            Rsqb => f.write_str("']'"),
            Colon => f.write_str("':'"),
            Comma => f.write_str("','"),
            Comment(value) => f.write_str(value),
            Semi => f.write_str("';'"),
            Plus => f.write_str("'+'"),
            Minus => f.write_str("'-'"),
            Star => f.write_str("'*'"),
            Slash => f.write_str("'/'"),
            Vbar => f.write_str("'|'"),
            Amper => f.write_str("'&'"),
            Less => f.write_str("'<'"),
            Greater => f.write_str("'>'"),
            Equal => f.write_str("'='"),
            Dot => f.write_str("'.'"),
            Percent => f.write_str("'%'"),
            Lbrace => f.write_str("'{'"),
            Rbrace => f.write_str("'}'"),
            EqEqual => f.write_str("'=='"),
            NotEqual => f.write_str("'!='"),
            LessEqual => f.write_str("'<='"),
            GreaterEqual => f.write_str("'>='"),
            Tilde => f.write_str("'~'"),
            CircumFlex => f.write_str("'^'"),
            LeftShift => f.write_str("'<<'"),
            RightShift => f.write_str("'>>'"),
            DoubleStar => f.write_str("'**'"),
            DoubleStarEqual => f.write_str("'**='"),
            PlusEqual => f.write_str("'+='"),
            MinusEqual => f.write_str("'-='"),
            StarEqual => f.write_str("'*='"),
            SlashEqual => f.write_str("'/='"),
            PercentEqual => f.write_str("'%='"),
            AmperEqual => f.write_str("'&='"),
            VbarEqual => f.write_str("'|='"),
            CircumflexEqual => f.write_str("'^='"),
            LeftShiftEqual => f.write_str("'<<='"),
            RightShiftEqual => f.write_str("'>>='"),
            DoubleSlash => f.write_str("'//'"),
            DoubleSlashEqual => f.write_str("'//='"),
            At => f.write_str("'@'"),
            AtEqual => f.write_str("'@='"),
            Rarrow => f.write_str("'->'"),
            Ellipsis => f.write_str("'...'"),
            False => f.write_str("'False'"),
            None => f.write_str("'None'"),
            True => f.write_str("'True'"),
            And => f.write_str("'and'"),
            As => f.write_str("'as'"),
            Assert => f.write_str("'assert'"),
            Async => f.write_str("'async'"),
            Await => f.write_str("'await'"),
            Break => f.write_str("'break'"),
            Class => f.write_str("'class'"),
            Continue => f.write_str("'continue'"),
            Def => f.write_str("'def'"),
            Del => f.write_str("'del'"),
            Elif => f.write_str("'elif'"),
            Else => f.write_str("'else'"),
            Except => f.write_str("'except'"),
            Finally => f.write_str("'finally'"),
            For => f.write_str("'for'"),
            From => f.write_str("'from'"),
            Global => f.write_str("'global'"),
            If => f.write_str("'if'"),
            Import => f.write_str("'import'"),
            In => f.write_str("'in'"),
            Is => f.write_str("'is'"),
            Lambda => f.write_str("'lambda'"),
            Nonlocal => f.write_str("'nonlocal'"),
            Not => f.write_str("'not'"),
            Or => f.write_str("'or'"),
            Pass => f.write_str("'pass'"),
            Raise => f.write_str("'raise'"),
            Return => f.write_str("'return'"),
            Try => f.write_str("'try'"),
            While => f.write_str("'while'"),
            With => f.write_str("'with'"),
            Yield => f.write_str("'yield'"),
            ColonEqual => f.write_str("':='"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum StringKind {
    String,
    FString,
    Bytes,
    RawString,
    RawFString,
    RawBytes,
    Unicode,
}

impl TryFrom<char> for StringKind {
    type Error = String;

    fn try_from(ch: char) -> Result<Self, String> {
        match ch {
            'r' | 'R' => Ok(StringKind::RawString),
            'f' | 'F' => Ok(StringKind::FString),
            'u' | 'U' => Ok(StringKind::Unicode),
            'b' | 'B' => Ok(StringKind::Bytes),
            c => Err(format!("Unexpected string prefix: {c}")),
        }
    }
}

impl TryFrom<[char; 2]> for StringKind {
    type Error = String;

    fn try_from(chars: [char; 2]) -> Result<Self, String> {
        match chars {
            ['r' | 'R', 'f' | 'F'] => Ok(StringKind::RawFString),
            ['f' | 'F', 'r' | 'R'] => Ok(StringKind::RawFString),
            ['r' | 'R', 'b' | 'B'] => Ok(StringKind::RawBytes),
            ['b' | 'B', 'r' | 'R'] => Ok(StringKind::RawBytes),
            [c1, c2] => Err(format!("Unexpected string prefix: {c1}{c2}")),
        }
    }
}

impl fmt::Display for StringKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StringKind::*;
        match self {
            String => f.write_str(""),
            FString => f.write_str("f"),
            Bytes => f.write_str("b"),
            RawString => f.write_str("r"),
            RawFString => f.write_str("rf"),
            RawBytes => f.write_str("rb"),
            Unicode => f.write_str("u"),
        }
    }
}

impl StringKind {
    pub fn is_raw(&self) -> bool {
        use StringKind::{RawBytes, RawFString, RawString};
        matches!(self, RawString | RawFString | RawBytes)
    }

    pub fn is_fstring(&self) -> bool {
        use StringKind::{FString, RawFString};
        matches!(self, FString | RawFString)
    }

    pub fn is_bytes(&self) -> bool {
        use StringKind::{Bytes, RawBytes};
        matches!(self, Bytes | RawBytes)
    }

    pub fn is_unicode(&self) -> bool {
        matches!(self, StringKind::Unicode)
    }

    pub fn prefix_len(&self) -> usize {
        use StringKind::*;
        match self {
            String => 0,
            RawString | FString | Unicode | Bytes => 1,
            RawFString | RawBytes => 2,
        }
    }
}
