use std::default::Default;
use std::ops::{AddAssign, SubAssign};

const TAPE_LEN: usize = 40_000;
const TAPE_START: usize = 10_000;

#[derive(Debug)]
pub struct Tape<T> {
    cells: [T; TAPE_LEN],
    ptr: usize,
}

impl<T> Tape<T>
where
    T: Incrementable,
{
    pub fn left(&mut self) {
        self.ptr -= 1;
    }
    pub fn right(&mut self) {
        self.ptr += 1;
    }
    pub fn inc(&mut self) {
        self.cells[self.ptr].post_inc();
    }
    pub fn dec(&mut self) {
        self.cells[self.ptr].post_dec();
    }
    pub fn put(&mut self, v: T) {
        self.cells[self.ptr] = v;
    }
    pub fn get(&self) -> T {
        self.cells[self.ptr]
    }
}

impl<T> Default for Tape<T>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            cells: [T::default(); TAPE_LEN],
            ptr: TAPE_START,
        }
    }
}

pub trait Incrementable: Copy + AddAssign<Self> + SubAssign<Self> {
    fn one() -> Self;

    fn post_inc(&mut self) -> Self {
        self.post_inc_by(Self::one())
    }

    fn post_inc_by(&mut self, n: Self) -> Self {
        let tmp = *self;
        *self += n;
        tmp
    }

    fn post_dec(&mut self) -> Self {
        self.post_dec_by(Self::one())
    }

    fn post_dec_by(&mut self, n: Self) -> Self {
        let tmp = *self;
        *self -= n;
        tmp
    }
}

macro_rules! impl_Incrementable{
    ($($m:ty),*) => {$( impl Incrementable for $m  { fn one() -> Self { 1 as $m } })*}
}

impl_Incrementable! {u8, u16, u32, u64, i8, i16, i32, i64, f32, f64}
