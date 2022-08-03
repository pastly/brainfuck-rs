use brainfuck::instruction::*;
use brainfuck::pseudo_instruction::{PseudoInst, PseudoInstIter};
use brainfuck::tape::Tape;
use brainfuck::EOFAction;
use std::default::Default;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let code_fname = &args[1];

    let mut tape: Tape = Default::default();
    let mut loop_starts: Vec<usize> = vec![];
    let eof_action = EOFAction::Zero;

    let mut out = std::io::stdout();

    let stdin = io::stdin();
    let mut input_bytes = stdin.lock().bytes();
    let code = InstIter::new(Box::new(File::open(code_fname)?));
    let code: Vec<PseudoInst> = PseudoInstIter::new(code).collect();
    let mut code_ptr = 0;
    while code_ptr < code.len() {
        let pi = &code[code_ptr];
        match pi {
            PseudoInst::Plain(inst) => match inst {
                Inst::Left
                | Inst::Right
                | Inst::Increment
                | Inst::Decrement
                | Inst::In
                | Inst::Out => unreachable!(),
                Inst::Begin => {
                    if tape.get() != 0 {
                        loop_starts.push(code_ptr);
                    } else {
                        let mut ends_needed = 1;
                        loop {
                            code_ptr += 1;
                            if code[code_ptr] == PseudoInst::Plain(Inst::Begin) {
                                ends_needed += 1;
                            } else if code[code_ptr] == PseudoInst::Plain(Inst::End) {
                                ends_needed -= 1;
                            }
                            if ends_needed == 0 {
                                break;
                            }
                        }
                        assert_eq!(code[code_ptr], PseudoInst::Plain(Inst::End));
                    }
                }
                Inst::End => {
                    if tape.get() != 0 {
                        code_ptr = loop_starts[loop_starts.len() - 1];
                    } else {
                        loop_starts.pop();
                    }
                }
                _ => unreachable!(),
            },
            PseudoInst::Repeat(inst, n) => {
                match inst {
                    Inst::Left => tape.left(*n),
                    Inst::Right => tape.right(*n),
                    Inst::Increment => tape.inc(*n as u8),
                    Inst::Decrement => tape.dec(*n as u8),
                    Inst::In => {
                        for _ in 0..*n {
                            if let Some(Ok(next_byte)) = input_bytes.next() {
                                tape.put(next_byte)
                            } else {
                                match eof_action {
                                    EOFAction::Zero => tape.put(0),
                                    EOFAction::NegativeOne => tape.put(255),
                                    EOFAction::NoChange => {}
                                }
                            }
                        }
                    }
                    Inst::Out => {
                        for _ in 0..*n {
                            out.write_all(&[tape.get()]).unwrap();
                            //out.flush().unwrap();
                        }
                    }
                    Inst::Begin | Inst::End | Inst::EOF => unreachable!(),
                }
            }
            _ => unreachable!(),
        };
        code_ptr += 1;
        //println!("{:?}", inst);
    }
    Ok(())
}
