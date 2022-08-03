use std::io::Read;

pub const CHAR_LEFT: char = '<';
pub const CHAR_RIGHT: char = '>';
pub const CHAR_INC: char = '+';
pub const CHAR_DEC: char = '-';
pub const CHAR_IN: char = ',';
pub const CHAR_OUT: char = '.';
pub const CHAR_BEGIN: char = '[';
pub const CHAR_END: char = ']';

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Inst {
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

pub struct InstIter {
    fd: Box<dyn Read>,
}

impl InstIter {
    pub fn new(fd: Box<dyn Read>) -> Self {
        Self { fd }
    }
}

impl Iterator for InstIter {
    type Item = Inst;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: [u8; 1] = [0];
        loop {
            match self.fd.read(&mut buf) {
                Ok(n) => match n {
                    // 0 => break Some(Instruction::EOF),
                    0 => break None,
                    _ => match buf[0] as char {
                        CHAR_LEFT => break Some(Inst::Left),
                        CHAR_RIGHT => break Some(Inst::Right),
                        CHAR_INC => break Some(Inst::Increment),
                        CHAR_DEC => break Some(Inst::Decrement),
                        CHAR_IN => break Some(Inst::In),
                        CHAR_OUT => break Some(Inst::Out),
                        CHAR_BEGIN => break Some(Inst::Begin),
                        CHAR_END => break Some(Inst::End),
                        _ => continue,
                    },
                },
                Err(_e) => break None,
            }
        }
    }
}
