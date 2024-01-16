extern crate logos;

use std::fmt::{Debug, Display};

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq, Clone)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

    // Tokens
    #[regex("[A-Za-zΑ-Ωα-ωͰ-Ͽ][A-Za-zΑ-Ωα-ωͰ-Ͽ0-9]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[token("\n")]
    TokenNewLine,

    #[token("<-")]
    TokenAssign,

    #[token("+")]
    TokenPlus,

    #[token("-")]
    TokenMinus,

    #[token("*")]
    TokenMultiply,

    #[token("/")]
    TokenDivide,

    #[token("^")]
    TokenRaise,

    #[token("<")]
    TokenLessThan,

    #[token("<=")]
    TokenLessEq,

    #[token("=")]
    TokenEq,

    #[token("<>")]
    TokenNotEq,

    #[token(">=")]
    TokenMoreEq,

    #[token(">")]
    TokenMoreThan,

    #[token("(")]
    TokenParenthLeft,

    #[token(")")]
    TokenParenthRight,

    #[token("[")]
    TokenBracketLeft,

    #[token("]")]
    TokenBracketRight,

    #[token(",")]
    TokenComma,

    #[token(".")]
    TokenDot,

    #[token(":")]
    TokenColon,

    #[token("&")]
    TokenAmpersand,

    #[token("..")]
    TokenRange,

    // Keywords - General
    #[token("ΔΙΑΒΑΣΕ")]
    KeywordRead,

    #[token("ΓΡΑΨΕ")]
    KeywordWrite,

    #[token("ΕΚΤΥΠΩΣΕ")]
    KeywordPrint,

    /// Operands
    #[token("DIV")]
    KeywordDiv,

    #[token("MOD")]
    KeywordMod,

    #[token("ΚΑΙ")]
    KeywordAnd,

    #[token("Η")]
    KeywordOr,

    #[token("ΟΧΙ")]
    KeywordNot,

    #[token("ΑΛΗΘΕΣ")]
    KeywordTrue,

    #[token("ΨΕΥΔΕΣ")]
    KeywordFalse,

    /// Control flow
    #[token("ΑΝ")]
    KeywordIf,

    #[token("ΤΟΤΕ")]
    KeywordThen,

    #[token("ΑΛΛΙΩΣ_ΑΝ")]
    KeywordElseIf,

    #[token("ΑΛΛΙΩΣ")]
    KeywordElse,

    #[token("ΤΕΛΟΣ_ΑΝ")]
    KeywordEndIf,

    /// Loops
    #[token("ΓΙΑ")]
    KeywordFor,

    #[token("ΑΠΟ")]
    KeywordFrom,

    #[token("ΜΕΧΡΙ")]
    KeywordUpTo,

    #[token("ΜΕ_ΒΗΜΑ")]
    KeywordWithStep,

    #[token("ΟΣΟ")]
    KeywordWhile,

    #[token("ΕΠΑΝΑΛΑΒΕ")]
    KeywordRepeat,

    #[token("ΑΡΧΗ_ΕΠΑΝΑΛΗΨΗΣ")]
    KeywordBeginLoop,

    #[token("ΜΕΧΡΙΣ_ΟΤΟΥ")]
    KeywordUntil,

    #[token("ΕΠΙΛΕΞΕ")]
    KeywordChoose,

    #[token("ΠΕΡΙΠΤΩΣΗ")]
    KeywordCase,

    #[token("ΤΕΛΟΣ_ΕΠΙΛΟΓΩΝ")]
    KeywordEndChoose,

    // Keywords - Program
    #[token("ΠΡΟΓΡΑΜΜΑ")]
    KeywordProgram,

    #[token("ΤΕΛΟΣ_ΠΡΟΓΡΑΜΜΑΤΟΣ")]
    KeywordEndProgram,

    #[token("ΑΡΧΗ")]
    KeywordBegin,

    #[token("ΣΤΑΘΕΡΕΣ")]
    KeywordConstants,

    #[token("ΜΕΤΑΒΛΗΤΕΣ")]
    KeywordVariables,

    #[token("ΑΚΕΡΑΙΕΣ")]
    KeywordProgInt,

    #[token("ΠΡΑΓΜΑΤΙΚΕΣ")]
    KeywordProgReal,

    #[token("ΧΑΡΑΚΤΗΡΕΣ")]
    KeywordProgChar,

    #[token("ΛΟΓΙΚΕΣ")]
    KeywordProgBool,

    // Keywords - Function
    #[token("ΣΥΝΑΡΤΗΣΗ")]
    KeywordFunction,

    #[token("ΑΚΕΡΑΙΑ")]
    KeywordInt,

    #[token("ΠΡΑΓΜΑΤΙΚΗ")]
    KeywordReal,

    #[token("ΧΑΡΑΚΤΗΡΑΣ")]
    KeywordChar,

    #[token("ΛΟΓΙΚΗ")]
    KeywordBool,

    #[token("ΤΕΛΟΣ_ΣΥΝΑΡΤΗΣΗΣ")]
    KeywordEndFunction,

    // Keywords - Procedure
    #[token("ΔΙΑΔΙΚΑΣΙΑ")]
    KeywordProcedure,

    #[token("ΚΑΛΕΣΕ")]
    KeywordCall,

    #[token("ΤΕΛΟΣ_ΔΙΑΔΙΚΑΣΙΑΣ")]
    KeywordEndProcedure,

    // Keywords - Algorithm
    #[token("ΑΛΓΟΡΙΘΜΟΣ")]
    KeywordAlgorithm,

    #[token("ΤΕΛΟΣ_ΑΛΓΟΡΙΘΜΟΥ")]
    KeywordEndAlgorithm,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Error => "unexpected token",
                Token::Identifier(_) => "identifier",
                Token::TokenNewLine => "\n",
                Token::TokenAssign => "<-",
                Token::TokenPlus => "+",
                Token::TokenMinus => "-",
                Token::TokenMultiply => "*",
                Token::TokenDivide => "/",
                Token::TokenRaise => "^",
                Token::TokenLessThan => "<",
                Token::TokenLessEq => "<=",
                Token::TokenEq => "=",
                Token::TokenNotEq => "<>",
                Token::TokenMoreEq => ">=",
                Token::TokenMoreThan => ">",
                Token::TokenParenthLeft => "(",
                Token::TokenParenthRight => ")",
                Token::TokenBracketLeft => "[",
                Token::TokenBracketRight => "]",
                Token::TokenComma => ",",
                Token::TokenDot => ".",
                Token::TokenColon => ":",
                Token::TokenAmpersand => "&",
                Token::TokenRange => "..",
                Token::KeywordRead => "ΔΙΑΒΑΣΕ",
                Token::KeywordWrite => "ΓΡΑΨΕ",
                Token::KeywordPrint => "ΕΚΤΥΠΩΣΕ",
                Token::KeywordDiv => "DIV",
                Token::KeywordMod => "MOD",
                Token::KeywordAnd => "ΚΑΙ",
                Token::KeywordOr => "Η",
                Token::KeywordNot => "ΟΧΙ",
                Token::KeywordTrue => "ΑΛΗΘΕΣ",
                Token::KeywordFalse => "ΨΕΥΔΕΣ",
                Token::KeywordIf => "ΑΝ",
                Token::KeywordThen => "ΤΟΤΕ",
                Token::KeywordElseIf => "ΑΛΛΙΩΣ_ΑΝ",
                Token::KeywordElse => "ΑΛΛΙΩΣ",
                Token::KeywordEndIf => "ΤΕΛΟΣ_ΑΝ",
                Token::KeywordFor => "ΓΙΑ",
                Token::KeywordFrom => "ΑΠΟ",
                Token::KeywordUpTo => "ΜΕΧΡΙ",
                Token::KeywordWithStep => "ΜΕ_ΒΗΜΑ",
                Token::KeywordWhile => "ΟΣΟ",
                Token::KeywordRepeat => "ΕΠΑΝΑΛΑΒΕ",
                Token::KeywordBeginLoop => "ΑΡΧΗ_ΕΠΑΝΑΛΗΨΗΣ",
                Token::KeywordUntil => "ΜΕΧΡΙΣ_ΟΤΟΥ",
                Token::KeywordChoose => "ΕΠΙΛΕΞΕ",
                Token::KeywordCase => "ΠΕΡΙΠΤΩΣΗ",
                Token::KeywordEndChoose => "",
                Token::KeywordProgram => "ΠΡΟΓΡΑΜΜΑ",
                Token::KeywordEndProgram => "ΤΕΛΟΣ_ΠΡΟΓΡΑΜΜΑΤΟΣ",
                Token::KeywordBegin => "ΑΡΧΗ",
                Token::KeywordConstants => "ΣΤΑΘΕΡΕΣ",
                Token::KeywordVariables => "ΜΕΤΑΒΛΗΤΕΣ",
                Token::KeywordProgInt => "ΑΚΕΡΑΙΕΣ",
                Token::KeywordProgReal => "ΠΡΑΓΜΑΤΙΚΕΣ",
                Token::KeywordProgChar => "ΧΑΡΑΚΤΗΡΕΣ",
                Token::KeywordProgBool => "ΛΟΓΙΚΕΣ",
                Token::KeywordFunction => "ΣΥΝΑΡΤΗΣΗ",
                Token::KeywordInt => "ΑΚΕΡΑΙΑ",
                Token::KeywordReal => "ΠΡΑΓΜΑΤΙΚΗ",
                Token::KeywordChar => "ΧΑΡΑΚΤΗΡΑΣ",
                Token::KeywordBool => "ΛΟΓΙΚΗ",
                Token::KeywordEndFunction => "ΤΕΛΟΣ_ΣΥΝΑΡΤΗΣΗΣ",
                Token::KeywordProcedure => "ΔΙΑΔΙΚΑΣΙΑ",
                Token::KeywordCall => "ΚΑΛΕΣΕ",
                Token::KeywordEndProcedure => "ΤΕΛΟΣ_ΔΙΑΔΙΚΑΣΙΑΣ",
                Token::KeywordAlgorithm => "ΑΛΓΟΡΙΘΜΟΣ",
                Token::KeywordEndAlgorithm => "ΤΕΛΟΣ_ΑΛΓΟΡΙΘΜΟΥ",
            }
        )
    }
}
