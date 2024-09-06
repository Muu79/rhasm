//! Module for disassembling Hack machine code into human readable instructions.

use std::{
    io::{ BufRead, BufReader, BufWriter, Error, Lines, Read, Write },
    iter::{ Filter, FusedIterator, Peekable },
};
use crate::decode_instruction;

/// Struct to disassemble a binary file into human readable instructions.
/// The disassembler will not be able to recover labels or variables.
/// Uses the Hack instruction set.
pub struct Disassembler<'a, R: Read, W: Write> {
    writer: Option<BufWriter<Box<&'a mut W>>>,
    lines: Peekable<Filter<Lines<BufReader<&'a mut R>>, fn(&Result<String, Error>) -> bool>>,
}

/// Config used to create a new Disassembler instance.
/// Takes two generics (`R` and `W`) that implement the [`Read`] and [`Write`] traits.
///
/// When the passed [`DisassemblerConfig::writer`] is [`None`]:
/// * the disassembler will return an error when using functions that attempt to write to the output.
pub struct DisassemblerConfig<'a, R: Read, W: Write> {
    pub reader: &'a mut R,
    pub writer: Option<&'a mut W>,
}

impl<'a, R, W> Disassembler<'a, R, W> where R: Read, W: Write {
    /// ## Arguments
    ///
    /// * `args` - A [`DisassemblerConfig`] struct that contain mutable references to the input read source, output write destination, and a boolean to determine whether the disassembler should write to the output in addition to returning the decoded instructions.
    ///
    /// ### Generics
    ///
    /// * `R` - A generic that implements the [`Read`] trait.
    /// * `W` - A generic that implements the [`Write`] trait.
    ///
    /// ## Returns
    ///
    /// Returns a new [`Disassembler`] instance. Calling any disassemble or write methods will advance the disassembler to the next instruction.
    /// The disassembler's methods will return [`None`] when it reaches the end of the input file.
    pub fn new(args: DisassemblerConfig<'a, R, W>) -> Disassembler<'a, R, W> {
        let DisassemblerConfig { reader, writer } = args;

        let filter: fn(&Result<String, Error>) -> bool = |line: &Result<String, Error>| {
            line.is_ok() && !line.as_ref().unwrap().is_empty()
        };

        let lines: Peekable<
            Filter<Lines<BufReader<&mut R>>, fn(&Result<String, Error>) -> bool>
        > = BufReader::new(reader).lines().filter(filter).peekable();

        let writer = match writer {
            Some(file) => Some(BufWriter::new(Box::new(file))),
            None => None,
        };

        Disassembler {
            writer,
            lines,
        }
    }

    /// Check if [`Disassembler::lines`] has more instructions to disassemble.
    fn has_next(&mut self) -> bool {
        self.lines.peek().is_some()
    }

    /// Disassemble and return the next instruction, advancing the disassembler.
    ///
    /// Returns [`None`] if there are no more instructions to disassemble.
    pub fn get_next(&mut self) -> Option<String> {
        let out: Option<String> = {
            if !self.has_next() {
                return None;
            }
            // we can unwrap here because of the peekable check in has_next() i.e. line will always match Some(T)
            let line = self.lines.next().unwrap();
            // Check if reading the line is an error
            if let Err(err) = line {
                eprintln!("Error reading line: {}", err);
                None
            } else {
                let instruction = match decode_instruction(line.unwrap().trim()) {
                    Ok(decoded) => decoded,
                    Err(err) => {
                        eprintln!("Error decoding instruction: {}", err);
                        return None;
                    }
                };
                Some(instruction)
            }
        };
        out
    }

    /// Disassemble and return all remaining instructions, advancing the disassembler to the end.
    ///
    /// ### Returns
    ///
    /// * Returns a [`Option`] wrapping all remaining instructions if there are any.
    /// * If there are no instructions to disassemble, will return [`None`].
    pub fn get_to_end(&mut self) -> Option<String> {
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
        match buffer.is_empty() {
            true => None,
            false => Some(buffer),
        }
    }

    /// Disassemble and write the next instruction to the writer in [`DisassemblerConfig::writer`], advancing the disassembler.
    ///
    /// ### Errors
    ///
    /// * Returns an error if there are issues writing to the output file.
    /// * Returns an error if there are no more instructions to disassemble.
    /// * Returns an error if the writer passed in [`DisassemblerConfig::writer`] is [`None`].
    pub fn write_next(&mut self) -> Result<(), Error> {
        let out = self.get_next();
        if out.is_some() {
            self.write_to_output(out.as_ref().unwrap())?;
            return Ok(());
        } else {
            return Err(Error::new(std::io::ErrorKind::Other, "No more lines to disassemble"));
        }
    }

    /// Disassemble and write all remaining instructions to the writer passed in [`DisassemblerConfig::writer`]
    ///
    /// * Advances the [`Disassembler`] to the end.
    ///
    /// ### Errors
    ///
    /// * Returns an error if there are issues writing to the output file.
    /// * Returns an error if there are no more instructions to disassemble.
    pub fn write_to_end(&mut self) -> Result<(), Error> {
        let out = self.get_to_end();
        if out.is_some() {
            let out = out.unwrap();
            self.write_to_output(out.as_ref())?;
            return Ok(());
        } else {
            return Err(Error::new(std::io::ErrorKind::Other, "No more lines to disassemble"));
        }
    }

    /// Disassemble, write and return the next instruction.
    /// * Writes to output referenced by [`DisassemblerConfig::writer`] if it is [`Some`].
    /// ### Returns
    ///
    /// * Returns a [`Result<Option>`] wrapping the next instruction if there is one.
    /// * If there is no instruction to disassemble, the result will wrap a [`None`].
    ///
    /// ### Errors
    ///
    /// * Returns an error if the reference passed by [`DisassemblerConfig::writer`] is [`None`]
    /// * Returns an error if there are issues writing to the output.
    pub fn get_and_write_next(&mut self) -> Result<Option<String>, Error> {
        let out = self.get_next();
        if let Some(instruction) = &out {
            self.write_to_output(instruction)?;
            return Ok(out);
        } else {
            return Ok(None);
        }
    }

    /// Disassemble, write and return all remaining instructions.
    /// * Writes to [`DisassemblerConfig::writer`] if it is [`Some`].
    /// ### Returns
    ///
    /// * Returns a [`Result<Option>`] wrapping all remaining instructions if there are any.
    /// * If there are no instructions to disassemble, the result will wrap a [`None`].
    ///
    /// ### Errors
    ///
    /// * Returns an error if the reference passed by [`DisassemblerConfig::writer`] is [`None`] 
    /// * Returns an error if there are issues writing to the output.
    pub fn get_and_write_to_end(&mut self) -> Result<Option<String>, Error> {
        let out = self.get_to_end();
        if out.is_some() {
            let out = out.unwrap();
            self.write_to_output(out.as_ref())?;
            return Ok(Some(out));
        } else {
            return Ok(None);
        }
    }

    fn write_to_output(&mut self, contents: &str) -> Result<(), Error> {
        if let Some(writer) = self.writer.as_mut() {
            if let Err(error) = write!(writer, "{}\n", contents.trim()) {
                eprintln!("Error writing to output: {}", error);
                return Err(error);
            }
            writer.flush().unwrap();
            return Ok(());
        } else {
            return Err(Error::new(std::io::ErrorKind::NotFound, "No writeable output specified"));
        }
    }
}

/// Implement the [`Iterator`] trait for [`Disassembler`]. Disassembler will yield each instruction as an [`Option<String>`].
impl<'a, R, W> Iterator for Disassembler<'a, R, W> where R: Read + 'a, W: Write + 'a {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next()
    }
}

impl<'a, R, W> FusedIterator for Disassembler<'a, R, W> where R: Read + 'a, W: Write + 'a {}
