use crate::instruction::OpCode;

/// Custom Error type for errors we throw as a library
#[derive(Debug)]
pub enum Error {
    InvalidOpCode(u8),
    UnsupportedOpCode(OpCode),
    Op0OutOfRange,
    Op1OutOfRange,
    AdditionOverflow(u16, u16),
    MemoryOverflow(u16),
    ParseOpError,
}

// do-core register indexes range from 0 to 7.
pub const MAX_REGISTER_INDEX: u8 = 7;

// do-core only support 4K of memory
pub const MEMORY_SIZE: usize = 0x1000;

// We declare our modules here
pub mod core;
pub mod instruction;
