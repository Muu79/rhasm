//! `rhasm` is a simple assembler for the Hack Assembly Language from the book
//! "*The Elements of Computing Systems*" by Noam Nisan and Shimon Schocken.
//! This project is the 6th project from their [Nand2Tetris course](https://www.nand2tetris.org/course).
//!
//! # Usage
//!
//! rhasm can be used as both a Rust library and as a standalone binary.
//!
//! ## As A Binary
//!
//!
//! To install the binary, you can run the following command:
//!
//! ```bash
//! cargo install rhasm
//! ```
//!
//! To then use the binary, you can run the following command:
//!
//! ```bash
//! rhasm <input_file> <output_file>
//! ```
//! ## As A Library
//!
//! To install rhasm as a library, you can add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! rhasm = "0.1.2"
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
//! As a library rhasm exposes both an [`Assembler`] and [`Disassembler`] struct that are able to read over the lines of some source file.
//! You can then use them to either write to a file or to return the decoded instructions as a string, both line by line or all at once.
//!
//! # Examples
//!
//! ## Assembling
//!
//! By using the [`Assembler`] struct you can build an assembler instance and call the [`Assembler::advance_to_end`] method to assemble the entire bin file or use [`Assembler::advance_once`] to write to the file one line at a time.
//!
//! ```rust
//! use rhasm::*;
//! use std::fs::File;
//!
//! let in_file = File::open("sample.asm").unwrap();
//! let out_file = File::create("sample.hack").unwrap();
//! let mut assembler_result = Assembler::build(&in_file, &out_file);
//! if let Ok(mut assembler) = assembler_result {
//!     assembler.advance_once();
//!     assembler.advance_to_end();
//! }
//! ```
//!
//! Alternatively you can call the [`Assembler::get_next_encoded_instruction`] method to return the next encoded instruction as a string that you can use as you see fit.
//!
//! ```rust
//! use rhasm::*;
//! use std::fs::File;
//!
//! let in_file = File::open("sample.asm").unwrap();
//! let out_file = File::create("sample.hack").unwrap();
//! let mut assembler = Assembler::build(&in_file, &out_file).unwrap();
//! let mut buffer = String::new();
//!
//! while let Some(encoded_instruction) = assembler.get_next_encoded_instruction() {
//!    buffer.push_str(&encoded_instruction);
//! }
//! // Do something with the buffer
//! ```
//!
//! ## Disassembling
//!
//! By using the [`Disassembler`] struct you can build a disassembler instance and call the [`Disassembler::write_to_end`] or [`Disassembler::write_next`] method to write the decoded instructions all at once, or line by line.
//! 
//! Alternatively you can call the [`Disassembler::get_next`] or [`Disassembler::get_to_end`] method to return the instructions as [`String`].
//! 
//! Lastly you can call the [`Disassembler::get_and_write_next`] or [`Disassembler::get_and_write_to_end`] method to both write to the output and return a [`String`].
//!
//! ### Examples
//! 
//! Consider the sample input and expected output below
//! ```rust
//! // Note the use of a Cursor here to simulate a file
//! // Any Type that implements io::Read can be used here
//! let mut reader = std::io::Cursor::new("
//! 0000000100000000
//! 1110110000010000
//! 0000000000000000
//! 1110101010000111
//! ");
//! let expected_output = "\
//! @256
//! D=A
//! @0
//! 0;JMP
//! ";
//! ```
//! We can pass these to the disassembler and get the disassembled instructions as a string, write them to a file, or both!
//! 
//! Furthermore, we can get instructions one at a time, or till the end of the file.
//! 
//! ```rust
//! # use rhasm::*;
//! # use std::io::{Cursor, Read, Write};
//! # use std::borrow::BorrowMut;
//! # // Note the use of a Cursor here to simulate a file
//! # // Any Type that implements [`io::Read`] can be used here
//! # let mut reader = Cursor::new("
//! # 0000000100000000
//! # 1110110000010000
//! # 0000000000000000
//! # 1110101010000111
//! # ");
//! # let expected_output = "\
//! # @256
//! # D=A
//! # @0
//! # 0;JMP
//! # ";
//! let args = DisassemblerConfig {
//!     reader: &mut reader,
//!     writer: None::<&mut Cursor<&mut [u8]>>,
//! };
//! let mut disassembler = Disassembler::new(args);
//!
//! let mut actual_output = String::new();
//! let first_line = match disassembler.get_next(){
//!     Some(line) => line + "\n",
//!     None => "".to_string(), // This would mean the reader had no valid instructions
//! };
//! let the_rest = disassembler.get_to_end();
//! actual_output.push_str(&first_line);
//! actual_output.push_str(&(the_rest.unwrap()));
//!
//! assert_eq!(expected_output, actual_output);
//! ```
//! Alternatively you can use the write methods to directly write to a file and skip handling the return values
//! 
//! ```rust
//! # use rhasm::*;
//! # use std::io::{Cursor, Read, Write};
//! # use std::borrow::BorrowMut;
//! # // Note the use of a Cursor here to simulate a file
//! # let mut reader = Cursor::new("
//! # 0000000100000000
//! # 1110110000010000
//! # 0000000000000000
//! # 1110101010000111
//! # ");
//! # let expected_output = "\
//! # @256
//! # D=A
//! # @0
//! # 0;JMP
//! # ";
//! let mut output = Cursor::new(Vec::new());
//!
//! let args = DisassemblerConfig {
//!    reader: &mut reader,
//!   writer: Some(output.borrow_mut()),
//! };
//! 
//! {
//! let mut disassembler = Disassembler::new(args);
//! disassembler.write_to_end();
//! } 
//!
//! let mut actual_output = String::new();
//! output.set_position(0);
//! output.read_to_string(&mut actual_output).unwrap();
//!
//! assert_eq!(expected_output, actual_output);
//!
//! ```
//! Alternatively you can both write to a file and get the disassembled instructions as a string
//!
//! ```rust
//! # use rhasm::*;
//! # use std::io::{Cursor, Read, Write};
//! # use std::borrow::BorrowMut;
//! # // Note the use of a Cursor here to simulate a file
//! # // Any Type that implements [`io::Read`] can be used here
//! # let mut reader = Cursor::new("
//! # 0000000100000000
//! # 1110110000010000
//! # 0000000000000000
//! # 1110101010000111
//! # ");
//! # let expected_output = "\
//! # @256
//! # D=A
//! # @0
//! # 0;JMP
//! # ";
//! let mut output = Cursor::new(Vec::new());
//! let mut out_string = String::new();
//! // This code block allows us to borrow output without explicitly dropping the disassembler
//! // You could replace the code block by dropping the disassembler explicitly
//! { 
//!     let args = DisassemblerConfig {
//!        reader: &mut reader,
//!        writer: Some(output.borrow_mut()),
//!     };
//! 
//!     let mut disassembler = Disassembler::new(args);
//!     
//!     while let Ok(Some(line)) = disassembler.get_and_write_next() {
//!         out_string.push_str(&line);
//!         out_string.push('\n');
//!     }
//! }
//! output.set_position(0); // rewind the cursor to the beginning of the file
//! 
//! let mut actual_output = String::new();
//! output.read_to_string(&mut actual_output).unwrap();
//! 
//! assert_eq!(expected_output, actual_output);
//! assert_eq!(expected_output, out_string);
//! ```
//! # License
//!
//! This project is licensed under the MIT or Apache-2.0 license, at your option.
//Define our library structure here
mod lib {
    pub mod assembler;
    pub mod encoder;
    pub mod disassembler;
    pub mod decoder;
}

// Here we declare what parts of the library are exposed to the user
// Namely the Assembler Struct and the Instruction Enum
pub use lib::{
    assembler::{ Assembler, Instruction },
    decoder::decode_instruction,
    disassembler::{ Disassembler, DisassemblerConfig },
    encoder::encode_instruction,
    assembler,
    disassembler,
};
