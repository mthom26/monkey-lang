mod code;
mod compiler;
pub use code::{make_op, two_u8_to_usize, OpCode};
pub use compiler::{ByteCode, Compiler};
