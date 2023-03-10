
use crate::parser::token::TokenSymbol;

// Data structure for object ///////////////////////////////////////////////////////////////////////

pub struct PythonCoreTokenizer {
    source_buffer: Box<Vec<char>>,
    index: u32,
    tab_size: u8
}

// Declaration of trait for Tokenizer //////////////////////////////////////////////////////////////

pub trait Tokenizer {
    fn new(buffer: String, tab_size: u8) -> PythonCoreTokenizer;
    fn tokenize(&self) -> Result<Box<Vec<Box<TokenSymbol>>>, String>;
    fn is_keyword(&self, text: &str, start: u32, end: u32) -> Option<TokenSymbol>;
    fn is_operator_or_delimiter(&self, c1: char, c2: char, c3: char, start_pos: u32) -> Option<(TokenSymbol, u8)>;
}


// Start of implementation of trait Tokenizer //////////////////////////////////////////////////////

impl Tokenizer for PythonCoreTokenizer {
    fn new(buffer: String, tab_size: u8) -> PythonCoreTokenizer {
        PythonCoreTokenizer {
            source_buffer: Box::new( buffer.chars().collect() ),
            index: 0,
            tab_size
        }
    }

    fn tokenize(&self) -> Result<Box<Vec<Box<TokenSymbol>>>, String> {
        let a = Box::new(Vec::from( [ Box::new(TokenSymbol::PyEof) ] ));
        Ok(a)
    }

    // Matches reserved keywords and returns token with start and end position.
    fn is_keyword(&self, text: &str, start_pos: u32, end_pos: u32) -> Option<TokenSymbol> {
        match text {
            "False"     => Some(TokenSymbol::PyFalse(start_pos, end_pos)),
            "None"      => Some(TokenSymbol::PyNone(start_pos, end_pos)),
            "True"      => Some(TokenSymbol::PyTrue(start_pos, end_pos)),
            "and"       => Some(TokenSymbol::PyAnd(start_pos, end_pos)),
            "as"        => Some(TokenSymbol::PyAs(start_pos, end_pos)),
            "assert"    => Some(TokenSymbol::PyAssert(start_pos, end_pos)),
            "async"     => Some(TokenSymbol::PyAsync(start_pos, end_pos)),
            "await"     => Some(TokenSymbol::PyAwait(start_pos, end_pos)),
            "break"     => Some(TokenSymbol::PyBreak(start_pos, end_pos)),
            "class"     => Some(TokenSymbol::PyClass(start_pos, end_pos)),
            "continue"  => Some(TokenSymbol::PyContinue(start_pos, end_pos)),
            "def"       => Some(TokenSymbol::PyDef(start_pos, end_pos)),
            "del"       => Some(TokenSymbol::PyDel(start_pos, end_pos)),
            "elif"      => Some(TokenSymbol::PyElif(start_pos, end_pos)),
            "else"      => Some(TokenSymbol::PyElse(start_pos, end_pos)),
            "except"    => Some(TokenSymbol::PyExcept(start_pos, end_pos)),
            "finally"    => Some(TokenSymbol::PyFinally(start_pos, end_pos)),
            "for"       => Some(TokenSymbol::PyFor(start_pos, end_pos)),
            "from"      => Some(TokenSymbol::PyFrom(start_pos, end_pos)),
            "global"    => Some(TokenSymbol::PyGlobal(start_pos, end_pos)),
            "if"        => Some(TokenSymbol::PyIf(start_pos, end_pos)),
            "import"    => Some(TokenSymbol::PyImport(start_pos, end_pos)),
            "in"        => Some(TokenSymbol::PyIn(start_pos, end_pos)),
            "is"        => Some(TokenSymbol::PyIs(start_pos, end_pos)),
            "lambda"    => Some(TokenSymbol::PyLambda(start_pos, end_pos)),
            "nonlocal"  => Some(TokenSymbol::PyNonlocal(start_pos, end_pos)),
            "not"       => Some(TokenSymbol::PyNot(start_pos, end_pos)),
            "or"        => Some(TokenSymbol::PyOr(start_pos, end_pos)),
            "pass"      => Some(TokenSymbol::PyPass(start_pos, end_pos)),
            "raise"     => Some(TokenSymbol::PyRaise(start_pos, end_pos)),
            "return"    => Some(TokenSymbol::PyReturn(start_pos, end_pos)),
            "try"       => Some(TokenSymbol::PyTry(start_pos, end_pos)),
            "while"     => Some(TokenSymbol::PyWhile(start_pos, end_pos)),
            "with"      => Some(TokenSymbol::PyWith(start_pos, end_pos)),
            "yield"     => Some(TokenSymbol::PyYield(start_pos, end_pos)),
            _ => None
        }
    }

    // Matches operators or delimiters and returns tuple with token and steps to go forward in buffer.
    fn is_operator_or_delimiter(&self, c1: char, c2: char, c3: char, start_pos: u32) -> Option<(TokenSymbol, u8)> {
        match ( c1, c2, c3 ) {
            ( '*', '*', '=' )   => Some( (TokenSymbol::PyPowerAssign(start_pos, start_pos + 3), 3) ),
            ( '*', '*', _ )     => Some( (TokenSymbol::PyPower(start_pos, start_pos + 2), 2) ),
            ( '*', '=', _ )     => Some( (TokenSymbol::PyMulAssign(start_pos, start_pos + 2), 2) ),
            ( '*', _ , _ )      => Some( (TokenSymbol::PyMul(start_pos, start_pos + 1), 1) ),
            ( '/', '/', '=' )   => Some( (TokenSymbol::PyFloorDivAssign(start_pos, start_pos + 3), 3) ),
            ( '/', '/', _ )     => Some( (TokenSymbol::PyFloorDiv(start_pos, start_pos + 2), 2) ),
            ( '/', '=', _ )     => Some( (TokenSymbol::PyDivAssign(start_pos, start_pos + 2), 2) ),
            ( '/', _ , _ )      => Some( (TokenSymbol::PyDiv(start_pos, start_pos + 1), 1) ),
            ( '<', '<', '=' )   => Some( (TokenSymbol::PyShiftLeftAssign(start_pos, start_pos + 3), 3) ),
            ( '<', '<', _ )     => Some( (TokenSymbol::PyShiftLeft(start_pos, start_pos + 2), 2) ),
            ( '<', '=', _ )     => Some( (TokenSymbol::PyLessEqual(start_pos, start_pos + 2), 2) ),
            ( '<', _ , _ )      => Some( (TokenSymbol::PyLess(start_pos, start_pos + 1), 1) ),
            ( '>', '>', '=' )   => Some( (TokenSymbol::PyShiftRightAssign(start_pos, start_pos + 3), 3) ),
            ( '>', '>', _ )     => Some( (TokenSymbol::PyShiftRight(start_pos, start_pos + 2), 2) ),
            ( '>', '=', _ )     => Some( (TokenSymbol::PyGreaterEqual(start_pos, start_pos + 2), 2) ),
            ( '>', _ , _ )      => Some( (TokenSymbol::PyGreater(start_pos, start_pos + 1), 1) ),
            ( '.', '.', '.' )   => Some( (TokenSymbol::PyEllipsis(start_pos, start_pos + 3), 3) ),
            ( '.', _ , _ )      => Some( (TokenSymbol::PyDot(start_pos, start_pos + 1), 1) ),
            ( '+', '=', _ )     => Some( (TokenSymbol::PyPlusAssign(start_pos, start_pos + 2), 2) ),
            ( '+', _ , _ )      => Some( (TokenSymbol::PyPlus(start_pos, start_pos + 1), 1) ),
            ( '-', '=', _ )     => Some( (TokenSymbol::PyMinusAssign(start_pos, start_pos + 2), 2) ),
            ( '-', '>', _ )     => Some( (TokenSymbol::PyArrow(start_pos, start_pos + 2), 2) ),
            ( '-', _ , _ )      => Some( (TokenSymbol::PyMinus(start_pos, start_pos + 1), 1) ),
            ( '%', '=', _ )     => Some( (TokenSymbol::PyModuloAssign(start_pos, start_pos + 2), 2) ),
            ( '%', _ , _ )      => Some( (TokenSymbol::PyModulo(start_pos, start_pos + 1), 1) ),
            ( '@', '=', _ )     => Some( (TokenSymbol::PyMatricesAssign(start_pos, start_pos + 2), 2) ),
            ( '@', _ , _ )      => Some( (TokenSymbol::PyMatrices(start_pos, start_pos + 1), 1) ),
            ( ':', '=', _ )     => Some( (TokenSymbol::PyColonAssign(start_pos, start_pos + 2), 2) ),
            ( ':', _ , _ )      => Some( (TokenSymbol::PyColon(start_pos, start_pos + 1), 1) ),
            ( '&', '=', _ )     => Some( (TokenSymbol::PyBitAndAssign(start_pos, start_pos + 2), 2) ),
            ( '&', _ , _ )      => Some( (TokenSymbol::PyBitAnd(start_pos, start_pos + 1), 1) ),
            ( '|', '=', _ )     => Some( (TokenSymbol::PyBitOrAssign(start_pos, start_pos + 2), 2) ),
            ( '|', _ , _ )      => Some( (TokenSymbol::PyBitOr(start_pos, start_pos + 1), 1) ),
            ( '^', '=', _ )     => Some( (TokenSymbol::PyBitXorAssign(start_pos, start_pos + 2), 2) ),
            ( '^', _ , _ )      => Some( (TokenSymbol::PyBitXor(start_pos, start_pos + 1), 1) ),
            ( '~', _ , _ )      => Some( (TokenSymbol::PyBitInvert(start_pos, start_pos + 1), 1) ),
            ( ';', _ , _ )      => Some( (TokenSymbol::PySemiColon(start_pos, start_pos + 1), 1) ),
            ( ',', _ , _ )      => Some( (TokenSymbol::PyComma(start_pos, start_pos + 1), 1) ),
            ( '=', '=', _ )     => Some( (TokenSymbol::PyEqual(start_pos, start_pos + 2), 2) ),
            ( '=', _ , _ )      => Some( (TokenSymbol::PyAssign(start_pos, start_pos + 1), 1) ),
            ( '!', '=', _ )     => Some( (TokenSymbol::PyNotEqual(start_pos, start_pos + 2), 2) ),
            ( '(', _ , _ )      => Some( (TokenSymbol::PyLeftParen(start_pos, start_pos + 1), 1) ),
            ( ')', _ , _ )      => Some( (TokenSymbol::PyRightParen(start_pos, start_pos + 1), 1) ),
            ( '[', _ , _ )      => Some( (TokenSymbol::PyLeftBracket(start_pos, start_pos + 1), 1) ),
            ( ']', _ , _ )      => Some( (TokenSymbol::PyRightBracket(start_pos, start_pos + 1), 1) ),
            ( '{', _ , _ )      => Some( (TokenSymbol::PyLeftCurly(start_pos, start_pos + 1), 1) ),
            ( '}', _ , _ )      => Some( (TokenSymbol::PyRightCurly(start_pos, start_pos + 1), 1) ),
            _ => None
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserved_keyword_false() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("False", 1, 6);
        match symbol {
            Some(TokenSymbol::PyFalse(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 6)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_none() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("None", 1, 5);
        match symbol {
            Some(TokenSymbol::PyNone(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 5)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_true() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("True", 1, 5);
        match symbol {
            Some(TokenSymbol::PyTrue(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 5)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_and() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("and", 1, 4);
        match symbol {
            Some(TokenSymbol::PyAnd(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 4)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_as() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("as", 1, 3);
        match symbol {
            Some(TokenSymbol::PyAs(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 3)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_assert() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("assert", 1, 7);
        match symbol {
            Some(TokenSymbol::PyAssert(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 7)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_async() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("async", 1, 6);
        match symbol {
            Some(TokenSymbol::PyAsync(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 6)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_await() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("await", 1, 6);
        match symbol {
            Some(TokenSymbol::PyAwait(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 6)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_break() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("break", 1, 6);
        match symbol {
            Some(TokenSymbol::PyBreak(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 6)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_class() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("class", 1, 6);
        match symbol {
            Some(TokenSymbol::PyClass(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 6)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_continue() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("continue", 1, 9);
        match symbol {
            Some(TokenSymbol::PyContinue(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 9)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_def() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("def", 1, 4);
        match symbol {
            Some(TokenSymbol::PyDef(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 4)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_del() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("del", 1, 4);
        match symbol {
            Some(TokenSymbol::PyDel(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 4)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_elif() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("elif", 1, 5);
        match symbol {
            Some(TokenSymbol::PyElif(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 5)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_else() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("else", 1, 5);
        match symbol {
            Some(TokenSymbol::PyElse(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 5)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_except() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("except", 1, 7);
        match symbol {
            Some(TokenSymbol::PyExcept(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 7)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_finally() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("finally", 1, 8);
        match symbol {
            Some(TokenSymbol::PyFinally(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 8)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_for() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("for", 1, 4);
        match symbol {
            Some(TokenSymbol::PyFor(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 4)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_from() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("from", 1, 5);
        match symbol {
            Some(TokenSymbol::PyFrom(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 5)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_global() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("global", 1, 7);
        match symbol {
            Some(TokenSymbol::PyGlobal(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 7)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_if() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("if", 1, 3);
        match symbol {
            Some(TokenSymbol::PyIf(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 3)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_import() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("import", 1, 7);
        match symbol {
            Some(TokenSymbol::PyImport(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 7)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_in() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("in", 1, 3);
        match symbol {
            Some(TokenSymbol::PyIn(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 3)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_is() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("is", 1, 3);
        match symbol {
            Some(TokenSymbol::PyIs(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 3)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_lambda() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("lambda", 1, 7);
        match symbol {
            Some(TokenSymbol::PyLambda(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 7)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_nonlocal() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("nonlocal", 1, 9);
        match symbol {
            Some(TokenSymbol::PyNonlocal(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 9)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_not() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("not", 1, 4);
        match symbol {
            Some(TokenSymbol::PyNot(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 4)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_or() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("or", 1, 3);
        match symbol {
            Some(TokenSymbol::PyOr(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 3)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_pass() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("pass", 1, 5);
        match symbol {
            Some(TokenSymbol::PyPass(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 5)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_raise() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("raise", 1, 6);
        match symbol {
            Some(TokenSymbol::PyRaise(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 6)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_return() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("return", 1, 7);
        match symbol {
            Some(TokenSymbol::PyReturn(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 7)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_try() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("try", 1, 4);
        match symbol {
            Some(TokenSymbol::PyTry(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 4)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_while() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("while", 1, 6);
        match symbol {
            Some(TokenSymbol::PyWhile(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 6)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_with() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("with", 1, 5);
        match symbol {
            Some(TokenSymbol::PyWith(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 5)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_yield() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("yield", 1, 6);
        match symbol {
            Some(TokenSymbol::PyYield(s, e)) => {
                assert_eq!(s, 1);
                assert_eq!(e, 6)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keyword_not_a_keyword() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_keyword("__init__", 1, 9);
        match symbol {
            None => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_power_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('*', '*', '=', 1);
        match symbol {
            Some( ( TokenSymbol::PyPowerAssign(1, 4), 3 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_power() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('*', '*', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyPower(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_mul_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('*', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyMulAssign(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_mul() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('*', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyMul(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_floor_div_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('/', '/', '=', 1);
        match symbol {
            Some( ( TokenSymbol::PyFloorDivAssign(1, 4), 3 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_floor_div() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('/', '/', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyFloorDiv(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_div_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('/', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyDivAssign(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_div() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('/', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyDiv(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_shift_left_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('<', '<', '=', 1);
        match symbol {
            Some( ( TokenSymbol::PyShiftLeftAssign(1, 4), 3 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_shift_left() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('<', '<', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyShiftLeft(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_less_equal() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('<', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyLessEqual(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_less() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('<', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyLess(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_shift_right_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('>', '>', '=', 1);
        match symbol {
            Some( ( TokenSymbol::PyShiftRightAssign(1, 4), 3 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_shift_right() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('>', '>', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyShiftRight(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_greater_equal() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('>', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyGreaterEqual(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_greater() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('>', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyGreater(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_ellipsis() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('.', '.', '.', 1);
        match symbol {
            Some( ( TokenSymbol::PyEllipsis(1, 4), 3 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_dot() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('.', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyDot(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_plus_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('+', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyPlusAssign(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_plus() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('+', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyPlus(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_minus_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('-', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyMinusAssign(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_arrow() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('-', '>', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyArrow(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_minus() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('-', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyMinus(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_modulo_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('%', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyModuloAssign(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_modulo() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('%', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyModulo(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_matrices_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('@', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyMatricesAssign(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_matrices() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('@', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyMatrices(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_colon_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter(':', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyColonAssign(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_colon() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter(':', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyColon(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_bit_and_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('&', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyBitAndAssign(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_bit_and() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('&', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyBitAnd(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_bit_or_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('|', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyBitOrAssign(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_bit_or() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('|', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyBitOr(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_bit_xor_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('^', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyBitXorAssign(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_bit_xor() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('^', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyBitXor(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_equal() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('=', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyEqual(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_assign() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('=', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyAssign(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_not_equal() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('!', '=', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyNotEqual(1, 3), 2 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_bit_invert() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('~', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyBitInvert(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_semicolon() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter(';', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PySemiColon(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_comma() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter(',', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyComma(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_left_paren() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('(', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyLeftParen(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_left_bracket() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('[', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyLeftBracket(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_left_curly() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('{', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyLeftCurly(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_right_paren() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter(')', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyRightParen(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_right_bracket() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter(']', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyRightBracket(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_right_curly() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter('}', ' ', ' ', 1);
        match symbol {
            Some( ( TokenSymbol::PyRightCurly(1, 2), 1 ) ) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_unknown() {
        let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new(String::from("Unused!"), 4);
        let symbol = lexer.is_operator_or_delimiter(' ', ' ', ' ', 1);
        match symbol {
            None => assert!(true),
            _ => assert!(false)
        }
    }
}
