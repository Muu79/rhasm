//! # Rhasm
//!
//! `rhasm` is a simple assembler for the Hack Assembly Language from the book
//! "The Elements of Computing Systems" by Noam Nisan and Shimon Schocken.
//! This project is the 6th project from their [Nand2Tetris course](https://www.nand2tetris.org/course).
//!
//! ## Installation
//! 
//! ### As A Binary
//! 
//! rhasm can be installed as both a Rust library and as a standalone binary.
//! 
//! To install the binary, you can run the following command:
//! 
//! ```bash
//! cargo install rhasm
//! ```
//! 
//! #### Usage
//! 
//! To use the binary, you can run the following command:
//! 
//! ```bash
//! rhasm <input_file> <output_file>
//! ```
//! ### As A Library
//! 
//! To install rhasm as a  library, you can add the following to your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! rhasm = "0.1.0"
//! ```
//! 
//! or use the following command:
//! 
//! ```bash
//! cargo add rhasm
//! ```
//! 
//! then import the library in your code:
//! 
//! ```rust
//! use rhasm::*;
//! ```
//! 
//! #### Usage
//!
//! As a library rhasm exposes both the `Assembler` struct and the `Instruction` enum.
//! 
//! By using the `Assembler` struct you can build an assembler instance and call the `advance_to_end` method to assemble the entire bin file or use `advance_once` to write to the file one line at a time.
//! 
//! ```rust
//! use rhasm::*;
//! use std::fs::File;
//! 
//! let in_file = File::open("sample.asm").unwrap();
//! let out_file = File::create("sample.hack").unwrap();
//! let mut assembler = Assembler::build(Some(in_file), Some(out_file));
//! assembler.unwrap().advance_to_end();
//! ```
//! 
//! Alternatively you can call the `get_next_encoded_instruction` method to return the next encoded instruction as a string that you can use as you see fit.
//! 
//! ```rust
//! use rhasm::*;
//! use std::fs::File;
//! 
//! let in_file = File::open("sample.asm").unwrap();
//! let mut assembler = Assembler::build(Some(in_file), None).unwrap();
//! let mut buffer = String::new();
//! 
//! while let Some(encoded_instruction) = assembler.get_next_encoded_instruction() {
//!    buffer.push_str(&encoded_instruction);
//! }
//! // Do something with the buffer
//! ```
// Define our library structure here
mod lib {
    pub mod assembler;
    pub mod encoder;
}

// Here we declare what parts of the library are exposed to the user
// Namely the Assembler Struct and the Instruction Enum
pub use lib::assembler::{ Assembler, Instruction };
