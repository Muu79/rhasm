//! Module for disassembling Hack machine code into human readable instructions.

use std::{
    io::{ BufRead, BufReader, BufWriter, Error, Lines, Read, Write, stdout },
    iter::{ Filter, Peekable },
};
use crate::decode_instruction;

/// Struct to disassemble a binary file into human readable instructions.
/// The disassembler will not be able to recover labels or variables.
/// Uses the Hack instruction set.
pub struct Disassembler<'a, R: Read + 'a> {
    out_file: Box<BufWriter<dyn Write>>,
    lines: Peekable<Filter<Lines<BufReader<&'a mut R>>, fn(&Result<String, Error>) -> bool>>,
    pub cur_encoded_instruction: Option<String>,
    pub cur_decoded_instruction: Option<String>,
    complete: bool,
}

/// Config used to create a new Disassembler instance.
/// Takes two generics (`R` and `W`) that implement the [`Read`] and [`Write`] traits.
///
/// When [`DisassemblerConfig::out_file`] is [`None`], the disassembler will attempt to write to stdout.
///
pub struct DisassemblerConfig<'a, R: Read, W: Write> {
    pub in_file: &'a mut R,
    pub out_file: Option<W>,
}

impl<'a, R> Disassembler<'a, R> where R: Read {
    /// # Arguments
    ///
    /// * `args` - A [`DisassemblerConfig`] struct that contains the input read source, output write destination, and a boolean to determine whether the disassembler should write to the output in addition to returning the decoded instructions.
    ///
    /// ## Generics
    ///
    /// * `R` - A generic that implements the [`Read`] trait.
    /// * `W` - A generic that implements the [`Write`] trait.
    ///
    /// # Returns
    ///
    /// Returns a new [`Disassembler`] instance. Calling any disassemble or write methods will advance the disassembler to the next instruction.
    /// The disassembler's methods will return [`None`] when it reaches the end of the input file.
    pub fn new<W: Write + 'static>(args: DisassemblerConfig<'a, R, W>) -> Disassembler<'a, R> {
        let DisassemblerConfig { in_file, out_file } = args;

        let line_filter: fn(&Result<String, Error>) -> bool = |line: &Result<String, Error>| {
            line.is_ok() && !line.as_ref().unwrap().is_empty()
        };

        let lines = BufReader::new(in_file).lines().filter(line_filter).peekable();

        let out_file: Box<BufWriter<dyn Write>> = match out_file {
            Some(file) => Box::new(BufWriter::new(file)),
            None => Box::new(BufWriter::new(stdout())),
        };

        Disassembler {
            out_file,
            lines,
            cur_encoded_instruction: None,
            cur_decoded_instruction: None,
            complete: false,
        }
    }

    pub fn has_next(&mut self) -> bool {
        self.lines.peek().is_some() && !self.complete
    }

    pub fn disassemble_one_line(&mut self) -> Option<String> {
        if self.complete {
            return None;
        }
        let out: Option<String> = {
            if self.has_next() {
                // we can unwrap here because of the peekable check in has_next() i.e. line will always match Some(T)
                let line = self.lines.next().unwrap();
                if let Err(err) = line {
                    eprintln!("Error reading line: {}", err);
                    return None;
                }
                let line = line.unwrap();
                self.cur_decoded_instruction = match decode_instruction(&line) {
                    Ok(decoded) => Some(decoded),
                    Err(err) => {
                        eprintln!("Error decoding instruction: {}", err);
                        None
                    }
                };
                self.cur_encoded_instruction = Some(line);
                self.cur_decoded_instruction.clone()
            } else {
                None
            }
        };
        out
    }

    pub fn disassemble_and_write_one_line(&mut self) -> Option<String> {
        let out = self.disassemble_one_line();
        if out.is_some() {
            if let Err(error) = writeln!(self.out_file, "{}", out.as_ref().unwrap()) {
                eprintln!("Error writing to file {}", error);
            }
        }
        out
    }

    pub fn write_one_line(&mut self) {
        let out = self.disassemble_one_line();
        if let Err(error) = write!(self.out_file, "{}", out.as_ref().unwrap()) {
            eprintln!("Error writing to file {}", error);
        }
    }

    pub fn disassemble_to_end(&mut self) -> Option<String> {
        let mut buffer = String::new();
        while let Some(line) = self.lines.next() {
            let instruction = match decode_instruction(line.unwrap().trim()) {
                Ok(decoded) => decoded,
                Err(err) => {
                    eprintln!("Error decoding instruction: {}", err);
                    continue;
                }
            };
            buffer.push_str(&instruction);
            buffer.push('\n');
        }
        self.complete = true;
        match buffer.is_empty() {
            true => None,
            false => Some(buffer),
        }
    }

    pub fn disassemble_and_write_to_end(&mut self) -> Option<String> {
        let out = self.disassemble_to_end();
        if out.is_some() {
            if let Err(error) = write!(self.out_file, "{}", out.as_ref().unwrap()) {
                eprintln!("Error writing to file: {}", error);
            }
        }
        out
    }

    pub fn write_to_end(&mut self) {
        let out = self.disassemble_to_end();
        if out.is_some() {
            if let Err(error) = write!(self.out_file, "{}", out.as_ref().unwrap()) {
                eprintln!("Error writing to file: {}", error);
            }
        }
    }
}
