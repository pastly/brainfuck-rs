use crate::instruction::{Inst, InstIter};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum PseudoInst {
    Noop,
    Plain(Inst),
    Repeat(Inst, usize),
}

pub struct PseudoInstIter {
    insts: Vec<PseudoInst>,
    n: usize,
}

impl PseudoInstIter {
    pub fn new(insts: InstIter) -> Self {
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
                    PseudoInst::Noop | PseudoInst::Plain(_) => {
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
        Self { insts: v, n: 0 }
    }
}

impl Iterator for PseudoInstIter {
    type Item = PseudoInst;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n < self.insts.len() {
            self.n += 1;
            Some(self.insts[self.n - 1])
        } else {
            None
        }
    }
}
