use crate::instruction::{Inst, InstIter};
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum PseudoInst {
    Noop,
    Plain(Inst),
    Repeat(Inst, usize),
    SetValue(u8),
}

fn optimize_combine_inc_dec(code: &mut Vec<PseudoInst>) -> usize {
    let mut dupes = VecDeque::new();
    let mut prev = None;
    for (idx, inst) in code.iter().enumerate() {
        match prev {
            None => prev = Some(inst),
            Some(_) => {
                if !matches!(
                    inst,
                    PseudoInst::Repeat(Inst::Increment | Inst::Decrement, _)
                ) {
                    prev = Some(inst);
                }
                if matches!(
                    prev,
                    Some(PseudoInst::Repeat(Inst::Increment | Inst::Decrement, _))
                ) {
                    dupes.push_back(idx);
                }
                prev = Some(inst);
            }
        }
    }
    let n_changes = dupes.len();
    while !dupes.is_empty() {
        let dupe = dupes.pop_front().unwrap();
        let prev = code[dupe - 1];
        let inst = code[dupe];
        assert!(matches!(
            prev,
            PseudoInst::Repeat(Inst::Increment | Inst::Decrement, _)
        ));
        assert!(matches!(
            inst,
            PseudoInst::Repeat(Inst::Increment | Inst::Decrement, _)
        ));
        let prev_n: i128 = match prev {
            PseudoInst::Repeat(Inst::Increment, n) => n as i128,
            PseudoInst::Repeat(Inst::Decrement, n) => -(n as i128),
            _ => unreachable!(),
        };
        let n: i128 = match inst {
            PseudoInst::Repeat(Inst::Increment, n) => n as i128,
            PseudoInst::Repeat(Inst::Decrement, n) => -(n as i128),
            _ => unreachable!(),
        };
        let new_n = prev_n + n;
        let new_n_abs = if new_n >= 0 {
            new_n as usize
        } else {
            -new_n as usize
        };
        let inc = if new_n >= 0 {
            Inst::Increment
        } else {
            Inst::Decrement
        };
        code[dupe - 1] = PseudoInst::Repeat(inc, new_n_abs);
        code.remove(dupe);
        for d in &mut dupes {
            *d -= 1;
        }
    }
    n_changes
}

fn optimize_remove_zero_inc_dec(code: &mut Vec<PseudoInst>) -> usize {
    let mut n_changes = 0;
    for inst in code {
        if matches!(
            inst,
            PseudoInst::Repeat(Inst::Increment | Inst::Decrement, 0)
        ) {
            *inst = PseudoInst::Noop;
            n_changes += 1;
        }
    }
    n_changes
}

fn optimize_set_zero(code: &mut Vec<PseudoInst>) -> usize {
    let mut idxes = vec![];
    for i in 0..code.len() - 2 {
        if matches!(code[i], PseudoInst::Plain(Inst::Begin))
            && matches!(code[i + 1], PseudoInst::Repeat(Inst::Decrement, 1))
            && matches!(code[i + 2], PseudoInst::Plain(Inst::End))
        {
            idxes.push(i);
        }
    }
    let n_changes = idxes.len();
    for i in idxes {
        code[i] = PseudoInst::SetValue(0);
        code[i + 1] = PseudoInst::Noop;
        code[i + 2] = PseudoInst::Noop;
    }
    n_changes
}

fn optimize_remove_noop(code: &mut Vec<PseudoInst>) -> usize {
    let old_len = code.len();
    code.retain(|i| !matches!(i, PseudoInst::Noop));
    let new_len = code.len();
    old_len - new_len
}

/// Try to optimize the given code, editing it in-place and returning true if an optimization was
/// made. Returns false if no optimizations were made
pub fn optimize(code: &mut Vec<PseudoInst>) -> usize {
    let mut n_changes = 0;
    n_changes += optimize_combine_inc_dec(code);
    n_changes += optimize_remove_zero_inc_dec(code);
    n_changes += optimize_set_zero(code);
    n_changes += optimize_remove_noop(code);
    eprintln!("Made {} optimizations", n_changes);
    n_changes
}

impl From<InstIter> for Vec<PseudoInst> {
    fn from(insts: InstIter) -> Self {
        let mut v = vec![];
        for inst in insts {
            let prev_pi = if !v.is_empty() {
                v[v.len() - 1]
            } else {
                PseudoInst::Noop
            };
            match inst {
                Inst::Left
                | Inst::Right
                | Inst::Increment
                | Inst::Decrement
                | Inst::In
                | Inst::Out => match prev_pi {
                    PseudoInst::Noop | PseudoInst::Plain(_) | PseudoInst::SetValue(_) => {
                        v.push(PseudoInst::Repeat(inst, 1));
                    }
                    PseudoInst::Repeat(prev_i, prev_n) => {
                        if prev_i == inst {
                            v.pop();
                            v.push(PseudoInst::Repeat(prev_i, prev_n + 1));
                        } else {
                            v.push(PseudoInst::Repeat(inst, 1));
                        }
                    }
                },
                Inst::Begin | Inst::End | Inst::EOF => {
                    v.push(PseudoInst::Plain(inst));
                }
            }
        }
        v.shrink_to_fit();
        v
    }
}
