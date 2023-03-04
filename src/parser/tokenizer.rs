
pub struct PythonCoreTokenizer {

}

pub trait Tokenizer {
    fn new() -> PythonCoreTokenizer;
    fn is_keyword(text: &str, start: u32, end: u32) -> Option<u32>;
}

impl Tokenizer for PythonCoreTokenizer {
    fn new() -> PythonCoreTokenizer {
        PythonCoreTokenizer {

        }
    }

    fn is_keyword(text: &str, start_pos: u32, end_pos: u32) -> Option<u32> {
        match text {
            "False" => Some(1),
            "None" => Some(2),
            "True" => Some(3),
            "and" => Some(4),
            "as" => Some(5),
            "assert" => Some(6),
            "async" => Some(7),
            "await" => Some(8),
            "break" => Some(9),
            "class" => Some(10),
            "continue" => Some(11),
            "def" => Some(12),
            "del" => Some(13),
            "elif" => Some(14),
            "else" => Some(15),
            "except" => Some(16),
            "finally" => Some(17),
            "for" => Some(18),
            "from" => Some(19),
            "global" => Some(20),
            "if" => Some(21),
            "import" => Some(22),
            "in" => Some(23),
            "is" => Some(24),
            "lambda" => Some(25),
            "nonlocal" => Some(26),
            "not" => Some(27),
            "or" => Some(28),
            "pass" => Some(29),
            "raise" => Some(30),
            "return" => Some(31),
            "try" => Some(32),
            "while" => Some(33),
            "with" => Some(34),
            "yield" => Some(35),
            _ => None
        }
    }


}
