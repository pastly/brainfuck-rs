use std::default::Default;

const TAPE_LEN: usize = 40_000;
const TAPE_START: usize = 10_000;

#[derive(Debug)]
pub struct Tape {
    cells: [u8; TAPE_LEN],
    ptr: usize,
}

impl Tape {
    pub fn left(&mut self) {
        self.ptr -= 1;
    }
    pub fn right(&mut self) {
        self.ptr += 1;
    }
    pub fn inc(&mut self) {
        self.cells[self.ptr] = self.cells[self.ptr].wrapping_add(1);
    }
    pub fn dec(&mut self) {
        self.cells[self.ptr] = self.cells[self.ptr].wrapping_sub(1);
    }
    pub fn put(&mut self, v: u8) {
        self.cells[self.ptr] = v;
    }
    pub fn get(&self) -> u8 {
        self.cells[self.ptr]
    }
}

impl Default for Tape {
    fn default() -> Self {
        Self {
            cells: [0; TAPE_LEN],
            ptr: TAPE_START,
        }
    }
}
