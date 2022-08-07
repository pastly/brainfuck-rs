use brainfuck::instruction::*;
use brainfuck::pseudo_instruction::PseudoInst;
use brainfuck::tape::Tape;
use brainfuck::EOFAction;
use std::collections::HashMap;
use std::default::Default;
use std::fs::File;
use std::io::{self, Read, Write};

fn begin_end_pairs(insts: &[PseudoInst]) -> HashMap<usize, usize> {
    let n_begins = insts
        .iter()
        .filter(|&&i| i == PseudoInst::Plain(Inst::Begin))
        .count();
    let mut begin_stack = Vec::with_capacity(n_begins/2);
    let mut map = HashMap::with_capacity(n_begins);
    for (idx, pi) in insts.iter().enumerate() {
        match pi {
            PseudoInst::Plain(i) => match i {
                Inst::Begin => begin_stack.push(idx),
                Inst::End => {
                    let b = begin_stack.pop().expect("Unbalance [ and ] in code");
                    let e = idx;
                    map.insert(b, e);
                }
                _ => continue,
            },
            _ => continue,
        }
    }
    println!("be map has {} keys", map.len());
    map
}

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let code_fname = &args[1];

    let mut tape: Tape = Default::default();
    let mut loop_starts: Vec<usize> = vec![];
    let eof_action = EOFAction::Zero;

    let mut out = std::io::stdout();

    let stdin = io::stdin();
    let mut input_bytes = stdin.lock().bytes();
    let code = Vec::from(InstIter::new(Box::new(File::open(code_fname)?)));
    let begin_end = begin_end_pairs(&code);
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
                        code_ptr = begin_end[&code_ptr];
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
