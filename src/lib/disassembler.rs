use std::{
    io::{ BufRead, BufReader, BufWriter, Error, Lines, Read, Write, stdout },
    iter::{ Filter, Peekable },
};
use crate::decode_instruction;

/// Struct to disassemble a binary file into human readable instructions.
/// The disassembler will not be able to recover labels or variables.
/// Uses the Hack instruction set.
pub struct Disassembler<'a, R: Read> {
    out_file: Box<dyn Write + 'a>,
    lines: Peekable<Filter<Lines<BufReader<R>>, fn(&Result<String, Error>) -> bool>>,
    pub cur_encoded_instruction: Option<String>,
    pub cur_decoded_instruction: Option<String>,
    complete: bool,
    write_to_file: bool,
}

pub struct DisassemblerConfig<R: Read, W: Write> {
    pub in_file: R,
    pub out_file: Option<W>,
    pub write_to_file: bool,
}

/// Implementation of the Disassembler struct. Takes a generic that implements the Read trait as an input.
impl<'a, R> Disassembler<'a, R> where R: Read {
    /// Creates a new Disassembler instance.
    /// Notably cannot fail given two valid files, thus is new and not build.
    pub fn new<W>(args: DisassemblerConfig<R, W>) -> Disassembler<'a, R>
        where &'a mut R: Read + 'a, W: Write + 'a
    {
        let DisassemblerConfig { in_file, out_file, write_to_file } = args;
        let line_filter: fn(&Result<String, Error>) -> bool = |line: &Result<String, Error>| {
            line.is_ok() && !line.as_ref().unwrap().is_empty()
        };

        let lines: Peekable<
            Filter<Lines<BufReader<R>>, fn(&Result<String, Error>) -> bool>
        > = BufReader::new(in_file).lines().filter(line_filter).peekable();

        let out_file: Box<dyn Write + 'a> = match out_file {
            Some(file) => Box::new(BufWriter::new(file)),
            None => Box::new(BufWriter::new(stdout())),
        };

        Disassembler {
            out_file,
            lines,
            cur_encoded_instruction: None,
            cur_decoded_instruction: None,
            complete: false,
            write_to_file,
        }
    }

    pub fn has_next(&mut self) -> bool {
        self.lines.peek().is_some()
    }

    pub fn advance_once(&mut self) -> Option<String> {
        if self.complete {
            return None;
        }
        let out = {
            if self.has_next() {
                let line = self.lines.next().unwrap().unwrap();
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
        if !self.write_to_file {
            return out;
        }
        if let Some(decoded_instruction) = out.as_ref() {
            write!(self.out_file, "{}\n", decoded_instruction).unwrap();
        }
        out
    }

    pub fn advance_to_end(&mut self) -> String {
        let buffer = &mut String::new();
        while let Some(decoded_instruction) = self.advance_once() {
            buffer.push_str(&decoded_instruction);
            buffer.push('\n');
        }
        buffer.trim_end().to_string()
    }
}
