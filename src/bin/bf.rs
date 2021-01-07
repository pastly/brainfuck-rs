use brainfuck::instruction::*;
use brainfuck::tape::Tape;
use std::default::Default;
use std::fs::File;
use std::io::{self, Read};

struct InstructionIter {
    fd: Box<dyn Read>,
}

impl InstructionIter {
    fn new(fd: Box<dyn Read>) -> Self {
        Self { fd }
    }
}

impl Iterator for InstructionIter {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: [u8; 1] = [0];
        loop {
            match self.fd.read(&mut buf) {
                Ok(n) => match n {
                    // 0 => break Some(Instruction::EOF),
                    0 => break None,
                    _ => match buf[0] as char {
                        CHAR_LEFT => break Some(Instruction::Left),
                        CHAR_RIGHT => break Some(Instruction::Right),
                        CHAR_INC => break Some(Instruction::Increment),
                        CHAR_DEC => break Some(Instruction::Decrement),
                        CHAR_IN => break Some(Instruction::In),
                        CHAR_OUT => break Some(Instruction::Out),
                        CHAR_BEGIN => break Some(Instruction::Begin),
                        CHAR_END => break Some(Instruction::End),
                        _ => continue,
                    },
                },
                Err(_e) => break None,
            }
        }
    }
}

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let code_fname = &args[1];

    let mut tape: Tape = Default::default();
    let mut loop_starts: Vec<usize> = vec![];

    let stdin = io::stdin();
    let mut input_bytes = stdin.lock().bytes();
    let code = InstructionIter::new(Box::new(File::open(code_fname)?)).collect::<Vec<_>>();
    let mut code_ptr = 0;
    while code_ptr < code.len() {
        let inst = &code[code_ptr];
        match inst {
            Instruction::Left => tape.left(),
            Instruction::Right => tape.right(),
            Instruction::Increment => tape.inc(),
            Instruction::Decrement => tape.dec(),
            Instruction::In => tape.put(input_bytes.next().unwrap().unwrap()),
            Instruction::Out => print!("{}", tape.get() as char),
            Instruction::Begin => {
                if tape.get() != 0 {
                    loop_starts.push(code_ptr);
                } else {
                    let mut ends_needed = 1;
                    loop {
                        code_ptr += 1;
                        if code[code_ptr] == Instruction::Begin {
                            ends_needed += 1;
                        } else if code[code_ptr] == Instruction::End {
                            ends_needed -= 1;
                        }
                        if ends_needed == 0 {
                            break;
                        }
                    }
                    assert_eq!(code[code_ptr], Instruction::End);
                }
            }
            Instruction::End => {
                if tape.get() != 0 {
                    code_ptr = loop_starts[loop_starts.len() - 1];
                } else {
                    loop_starts.pop();
                }
            }
            _ => unreachable!(),
        };
        code_ptr += 1;
        //println!("{:?}", inst);
    }
    Ok(())
}
