pub mod instruction;
pub mod pseudo_instruction;
pub mod tape;

pub enum EOFAction {
    Zero,
    NegativeOne,
    NoChange,
}
