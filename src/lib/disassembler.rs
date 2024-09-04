use std::{
    fs::File,
    io::{BufRead as _, BufReader, BufWriter, Error, Lines, Write as _},
    iter::{Filter, Peekable},
};
use crate::decode_instruction;

pub struct Disassembler<'a> {
    out_file: BufWriter<&'a File>,
    lines: Peekable<Filter<Lines<BufReader<&'a File>>, fn(&Result<String, Error>) -> bool>>,
    buffer: String,
    pub cur_encoded_instruction: Option<String>,
    pub cur_decoded_instruction: Option<String>,
    complete: bool,
}

impl<'a> Disassembler<'a> {
    pub fn new(in_file: &'a File, out_file: &'a File) -> Disassembler<'a> {
        let filtered_lines: fn(&Result<String, Error>) -> bool = |line: &Result<String, Error>| line.is_ok() && !line.as_ref().unwrap().is_empty();
        let lines: Peekable<Filter<Lines<BufReader<&'a File>>, fn(&Result<String, Error>) -> bool>> =
            BufReader::new(in_file)
                .lines()
                .filter(filtered_lines)
                .peekable();
        let out_file = BufWriter::new(out_file);
        Disassembler {
            out_file,
            lines,
            buffer: String::new(),
            cur_encoded_instruction: None,
            cur_decoded_instruction: None,
            complete: false,
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
        if let Some(decoded_instruction) = out.as_ref() {
            self.buffer.push_str(decoded_instruction);
            self.buffer.push('\n');
        } else {
            self.out_file.write_all(self.buffer.as_bytes()).unwrap();
            self.complete = true;
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