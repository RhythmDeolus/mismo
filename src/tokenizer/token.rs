use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TokenType {
    Comment,
    // Numerical
    Plus,
    Minus,
    Slash,
    Mul,
    Mod,
    PlusPlus,
    MinusMinus,
    // Logical
    True,
    False,
    Bang,
    Or,
    And,
    EqualEqual,
    BangEqual,
    LessEqual,
    GreatEqual,
    Less,
    Greater,
    // Assign
    Equal,
    MinusEqual,
    PlusEqual,
    SlashEqual,
    MulEqual,
    ModEqual,
    // Keywords
    Var,
    Function,
    If,
    Else,
    While,
    For,
    Return,
    Print,
    // Brackets
    OpenSquare,
    CloseSquare,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    // Literals
    String,
    Number,
    Identifer,
    None,
    // Others
    Dot,
    Comma,
    Semicolon,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Comment => write!(f, "Comment"),
            // Numerical
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Mul => write!(f, "Mul"),
            TokenType::Mod => write!(f, "Mod"),
            TokenType::PlusPlus => write!(f, "PlusPlus"),
            TokenType::MinusMinus => write!(f, "MinusMinus"),
            // Logical
            TokenType::True => write!(f, "True"),
            TokenType::False => write!(f, "False"),
            TokenType::Bang => write!(f, "Bang"),
            TokenType::Or => write!(f, "Or"),
            TokenType::And => write!(f, "And"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::BangEqual => write!(f, "BangEqual"),
            TokenType::LessEqual => write!(f, "LessEqual"),
            TokenType::GreatEqual => write!(f, "GreatEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::Greater => write!(f, "Greater"),
            // Assign
            TokenType::Equal => write!(f, "Equal"),
            TokenType::MinusEqual => write!(f, "MinusEqual"),
            TokenType::PlusEqual => write!(f, "PlusEqual"),
            TokenType::SlashEqual => write!(f, "SlashEqual"),
            TokenType::MulEqual => write!(f, "MulEqual"),
            TokenType::ModEqual => write!(f, "ModEqual"),
            // Keywords
            TokenType::Var => write!(f, "Var"),
            TokenType::Function => write!(f, "Function"),
            TokenType::If => write!(f, "If"),
            TokenType::Else => write!(f, "Else"),
            TokenType::While => write!(f, "While"),
            TokenType::For => write!(f, "For"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Print => write!(f, "Print"),
            // Brackets
            TokenType::OpenSquare => write!(f, "OpenSquare"),
            TokenType::CloseSquare => write!(f, "CloseSquare"),
            TokenType::OpenParen => write!(f, "OpenParen"),
            TokenType::CloseParen => write!(f, "CloseParen"),
            TokenType::OpenCurly => write!(f, "OpenCurly"),
            TokenType::CloseCurly => write!(f, "CloseCurly"),
            // Literals
            TokenType::String => write!(f, "String"),
            TokenType::Number => write!(f, "Number"),
            TokenType::Identifer => write!(f, "Identifer"),
            TokenType::None => write!(f, "None"),
            // Others
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Semicolon => write!(f, "Semicolon"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub literal: String,
    pub t_type: TokenType,
    pub line_no: usize,
    pub col_no: usize,
}
