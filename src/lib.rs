pub mod instruction;
pub mod tape;

pub enum EOFAction {
    Zero,
    NegativeOne,
    NoChange,
}
