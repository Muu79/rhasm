use crate::encoder;

use regex::Regex;

use std::collections::HashMap;

use std::fs::File;

use std::io::{ BufRead, BufReader, BufWriter, Lines, Write };

use std::iter::Peekable;

pub(crate) const INSTRUCTION_REGEX: &str = {
    r"(?x) # Ignore whitespace and allow comments
^
(?:
    @(?P<a_symbol>[a-zA-Z_\.\$:][\w\.\$:]*|\d+) # A-instruction (address or symbol)
  |
    \((?P<l_label>[a-zA-Z_\.\$:][\w\.\$:]+)\)   # L-instruction (label)
  |
    (?:
        (?P<c_dest>[ADM]{1,3})?  # Optional dest part for C-instruction
        =?
        (?P<c_comp>[AMD01!+\-&|]+) # Required comp part for C-instruction
        ;?
        (?P<c_jump>[A-Z]{3})?   # Optional jump part for C-instruction
    )
)
$
"
};

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    AInstruction(String),
    CInstruction(String, String, String),
}

pub struct Assembler<'a> {
    pub filename: &'a str,
    pub(crate) lines: Peekable<Lines<BufReader<File>>>,
    pub(crate) cur_ram: u16,
    pub(crate) cur_line: usize,
    pub(crate) cur_instruction: u16,
    pub symbol_table: HashMap<String, u16>,
    pub(crate) out_file: BufWriter<File>,
    pub instructions: Vec<Instruction>,
    pub(crate) fp_flag: bool,
    pub(crate) instruction_regex: Regex,
}

impl Assembler<'_> {
    // We take in a filename and an output file name
    // May change to take in a BufReader and BufWriter instead
    // Or use generics to allow for any type that implements Read and Write
    pub fn new<'a>(filename: &'a str, out_file: &'a str) -> Assembler<'a> {
        let file: File = File::open(filename).unwrap_or(File::open("default.asm").unwrap());
        let out_file = BufWriter::new(File::create(out_file).unwrap());
        let lines: Peekable<Lines<BufReader<File>>> = BufReader::new(file).lines().peekable();
        let symbol_table: HashMap<String, u16> = HashMap::new();
        let mut assembler = Assembler {
            filename,
            lines,
            cur_ram: 16,
            cur_line: 0,
            cur_instruction: 0,
            symbol_table,
            out_file,
            instructions: Vec::new(),
            fp_flag: false,
            instruction_regex: Regex::new(INSTRUCTION_REGEX).unwrap(),
        };
        assembler.init();
        assembler
    }

    // Function to initialize the assembler and its symbol table
    // Called by constructor to ensure symbol table is populated
    pub(crate) fn init(&mut self) {
        if !self.fp_flag {
            self.first_pass();
            println!("First Pass Completed!");
        } else {
            println!("First Pass Already Completed!");
        }
    }

    // Function to check if there are more commands to read
    // Uses the Peekable iterator to safe-check if there are more lines
    pub fn can_read_more_instructions(&mut self) -> bool {
        // only returns none on EOF not on empty lines
        self.lines.peek().is_some()
    }

    // Function to run the first pass of the assembler
    // Populates the symbol table with default symbols
    // Additionally parses through the source file and creates a vector of Instructions
    pub(crate) fn first_pass(&mut self) {
        self.populate_default_symbols();
        println!("Generated Default Symbol Table!");
        while self.can_read_more_instructions() {
            self.parse_instruction();
            self.cur_line += 1;
        }
        self.fp_flag = true;
    }

    // Function dedicated to parsing through our source file and creating a vector of Instructions
    // This allows for address labels to be resolved in the second pass
    // As well as us extracting the instructions from the file into enums
    pub(crate) fn parse_instruction(&mut self) {
        // We only parse when has_more_commands() is true so we can unwrap safely
        let line = self.lines.next().unwrap().unwrap();
        // Remove comments and trim whitespace
        let line = line.split("//").next().unwrap().trim().to_owned();
        if line.is_empty() {
            return;
        }

        let captures = self.instruction_regex.captures(&line);
        if let Some(captures) = captures {
            if let Some(a_symbol) = captures.name("a_symbol") {
                let addr = a_symbol.as_str();
                self.instructions.push(Instruction::AInstruction(addr.to_string()));
            } else if let Some(c_comp) = captures.name("c_comp") {
                let c_comp = c_comp.as_str();
                let c_dest = captures.name("c_dest").map_or("", |m| m.as_str());
                let c_jump = captures.name("c_jump").map_or("", |m| m.as_str());
                self.instructions.push(
                    Instruction::CInstruction(
                        c_dest.to_string(),
                        c_comp.to_string(),
                        c_jump.to_string()
                    )
                );
            } else if let Some(l_label) = captures.name("l_label") {
                let label = l_label.as_str();
                self.symbol_table.insert(
                    label.to_string(),
                    self.instructions.len().try_into().unwrap()
                );
            } else {
                panic!("Invalid Instruction @ line [{}]: {}", self.cur_line, line);
            }
        } else {
            panic!("Invalid Instruction @ line [{}]: {}", self.cur_line, line);
        }
    }

    // Subroutine to populate the default symbols
    // Symbol names as per the Hack Assembly Language Specification
    pub(crate) fn populate_default_symbols(&mut self) {
        self.symbol_table.insert("SP".to_string(), 0);
        self.symbol_table.insert("LCL".to_string(), 1);
        self.symbol_table.insert("ARG".to_string(), 2);
        self.symbol_table.insert("THIS".to_string(), 3);
        self.symbol_table.insert("THAT".to_string(), 4);
        self.symbol_table.insert("R0".to_string(), 0);
        self.symbol_table.insert("R1".to_string(), 1);
        self.symbol_table.insert("R2".to_string(), 2);
        self.symbol_table.insert("R3".to_string(), 3);
        self.symbol_table.insert("R4".to_string(), 4);
        self.symbol_table.insert("R5".to_string(), 5);
        self.symbol_table.insert("R6".to_string(), 6);
        self.symbol_table.insert("R7".to_string(), 7);
        self.symbol_table.insert("R8".to_string(), 8);
        self.symbol_table.insert("R9".to_string(), 9);
        self.symbol_table.insert("R10".to_string(), 10);
        self.symbol_table.insert("R11".to_string(), 11);
        self.symbol_table.insert("R12".to_string(), 12);
        self.symbol_table.insert("R13".to_string(), 13);
        self.symbol_table.insert("R14".to_string(), 14);
        self.symbol_table.insert("R15".to_string(), 15);
        self.symbol_table.insert("SCREEN".to_string(), 16384);
        self.symbol_table.insert("KBD".to_string(), 24576);
    }

    pub fn advance_once(&mut self) {
        let encoded_instruction = self.get_next_encoded_instruction();
        self.write_line(encoded_instruction);
    }

    pub fn advance_to_end(&mut self) {
        if !self.fp_flag {
            self.init();
        }
        let mut buffer = String::new();
        while self.cur_instruction < (self.instructions.len() as u16) {
            buffer.push_str(&format!("{}\n", self.get_next_encoded_instruction()));
        }
        self.write_line(buffer);
    }

    pub fn get_next_encoded_instruction(&mut self) -> String {
        let out = encoder::encode_instruction(
            self.instructions.get(self.cur_instruction as usize).unwrap(),
            &mut self.symbol_table,
            &mut self.cur_ram
        );
        self.cur_instruction += 1;
        if self.cur_instruction % ((self.instructions.len() / 10) as u16) == 0 {
            println!("Encoded {} instructions", self.cur_instruction);
        }
        out
    }

    pub(crate) fn write_line(&mut self, encoded: String) {
        writeln!(self.out_file, "{}", encoded).unwrap();
    }
}
