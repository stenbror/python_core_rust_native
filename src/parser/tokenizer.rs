
use crate::parser::token::TokenSymbol;

// Data structure for object ///////////////////////////////////////////////////////////////////////

pub struct PythonCoreTokenizer {

}

// Declaration of trait for Tokenizer //////////////////////////////////////////////////////////////

pub trait Tokenizer {
    fn new() -> PythonCoreTokenizer;
    fn tokenize(buffer: &str) -> Result<Box<Vec<Box<TokenSymbol>>>, String>;
    fn is_keyword(text: &str, start: u32, end: u32) -> Option<TokenSymbol>;
}


// Start of implementation of trait Tokenizer //////////////////////////////////////////////////////

impl Tokenizer for PythonCoreTokenizer {
    fn new() -> PythonCoreTokenizer {
        PythonCoreTokenizer {

        }
    }

    fn tokenize(buffer: &str) -> Result<Box<Vec<Box<TokenSymbol>>>, String> {
        let a = Box::new(Vec::from( [ Box::new(TokenSymbol::PyEof) ] ));
        Ok(a)
    }

    fn is_keyword(text: &str, start_pos: u32, end_pos: u32) -> Option<TokenSymbol> {
        match text {
            "False" => Some(TokenSymbol::PyFalse(start_pos, end_pos)),
            "None" => Some(TokenSymbol::PyNone(start_pos, end_pos)),
            "True" => Some(TokenSymbol::PyTrue(start_pos, end_pos)),
            "and" => Some(TokenSymbol::PyAnd(start_pos, end_pos)),
            "as" => Some(TokenSymbol::PyAs(start_pos, end_pos)),
            "assert" => Some(TokenSymbol::PyAssert(start_pos, end_pos)),
            "async" => Some(TokenSymbol::PyAsync(start_pos, end_pos)),
            "await" => Some(TokenSymbol::PyAwait(start_pos, end_pos)),
            "break" => Some(TokenSymbol::PyBreak(start_pos, end_pos)),
            "class" => Some(TokenSymbol::PyClass(start_pos, end_pos)),
            "continue" => Some(TokenSymbol::PyContinue(start_pos, end_pos)),
            "def" => Some(TokenSymbol::PyDef(start_pos, end_pos)),
            "del" => Some(TokenSymbol::PyDel(start_pos, end_pos)),
            "elif" => Some(TokenSymbol::PyElif(start_pos, end_pos)),
            "else" => Some(TokenSymbol::PyElse(start_pos, end_pos)),
            "except" => Some(TokenSymbol::PyExcept(start_pos, end_pos)),
            "finally" => Some(TokenSymbol::PyFinally(start_pos, end_pos)),
            "for" => Some(TokenSymbol::PyFor(start_pos, end_pos)),
            "from" => Some(TokenSymbol::PyFrom(start_pos, end_pos)),
            "global" => Some(TokenSymbol::PyGlobal(start_pos, end_pos)),
            "if" => Some(TokenSymbol::PyIf(start_pos, end_pos)),
            "import" => Some(TokenSymbol::PyImport(start_pos, end_pos)),
            "in" => Some(TokenSymbol::PyIn(start_pos, end_pos)),
            "is" => Some(TokenSymbol::PyIs(start_pos, end_pos)),
            "lambda" => Some(TokenSymbol::PyLambda(start_pos, end_pos)),
            "nonlocal" => Some(TokenSymbol::PyNonlocal(start_pos, end_pos)),
            "not" => Some(TokenSymbol::PyNot(start_pos, end_pos)),
            "or" => Some(TokenSymbol::PyOr(start_pos, end_pos)),
            "pass" => Some(TokenSymbol::PyPass(start_pos, end_pos)),
            "raise" => Some(TokenSymbol::PyRaise(start_pos, end_pos)),
            "return" => Some(TokenSymbol::PyReturn(start_pos, end_pos)),
            "try" => Some(TokenSymbol::PyTry(start_pos, end_pos)),
            "while" => Some(TokenSymbol::PyWhile(start_pos, end_pos)),
            "with" => Some(TokenSymbol::PyWith(start_pos, end_pos)),
            "yield" => Some(TokenSymbol::PyYield(start_pos, end_pos)),
            _ => None
        }
    }


}
