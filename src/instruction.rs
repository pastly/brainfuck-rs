pub const CHAR_LEFT: char = '<';
pub const CHAR_RIGHT: char = '>';
pub const CHAR_INC: char = '+';
pub const CHAR_DEC: char = '-';
pub const CHAR_IN: char = ',';
pub const CHAR_OUT: char = '.';
pub const CHAR_BEGIN: char = '[';
pub const CHAR_END: char = ']';

#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
    Increment,
    Decrement,
    In,
    Out,
    Begin,
    End,
    EOF,
}
