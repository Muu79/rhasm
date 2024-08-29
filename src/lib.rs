// Define our library structure here
mod lib{
    pub mod assembler;
    pub mod encoder;
}

// Here we declare what parts of the library are exposed to the user
// The assembler Struct and the Instruction Enum
pub use lib::assembler::{Assembler, Instruction};