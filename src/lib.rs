pub mod file_parser {
    use regex::Regex;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{ BufRead, BufReader, BufWriter, Lines, Write };
    use std::iter::Peekable;

    use crate::encoder;

    const INSTRUCTION_REGEX: &str =
        r"(?x)
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
";

    #[derive(Clone, Debug, PartialEq)]
    pub enum Instruction {
        AInstruction(String),
        CInstruction(String, String, String),
    }

    pub struct Assembler<'a> {
        pub filename: &'a str,
        lines: Peekable<Lines<BufReader<File>>>,
        cur_ram: u16,
        cur_line: usize,
        cur_instruction: u16,
        pub symbol_table: HashMap<String, u16>,
        out_file: BufWriter<File>,
        pub instructions: Vec<Instruction>,
        fp_flag: bool,
        instruction_regex: Regex,
    }

    impl Assembler<'_> {
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
        fn init(&mut self) {
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
        fn first_pass(&mut self) {
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
        fn parse_instruction(&mut self) {
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
        fn populate_default_symbols(&mut self) {
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
            if self.cur_instruction % 100 == 0 {
                println!("Encoded {} instructions", self.cur_instruction);
            }
            out
        }

        fn write_line(&mut self, encoded: String) {
            writeln!(self.out_file, "{}", encoded).unwrap();
        }
    }
}

pub mod encoder {
    use std::collections::HashMap;
    use crate::file_parser::Instruction;

     pub fn encode_instruction(
        instruction: &Instruction,
        symbol_table: &mut HashMap<String, u16>,
        cur_ram: &mut u16
    ) -> String {
        let mut encoded_instruction: Vec<char> = vec![];
        match instruction {
            Instruction::AInstruction(addr) => {
                encoded_instruction.push('0');
                let addr = if addr.chars().all(|char| char.is_digit(10)) {
                    let is_num = addr.parse::<u16>();
                    if let Ok(num) = is_num {
                        num
                    } else {
                        panic!("Invalid A-Instruction address or label: {}", addr);
                    }
                } else {
                    if !symbol_table.contains_key(&addr.to_string()) {
                        symbol_table.insert(addr.to_string(), *cur_ram);
                        *cur_ram += 1;
                    }
                    *symbol_table.get(&addr.to_string()).unwrap()
                };
                let binary_addr = format!("{:015b}", addr);
                encoded_instruction.extend(binary_addr.chars());
            }
            Instruction::CInstruction(dest_str, comp_str, jump_string) => {
                encoded_instruction.extend("111".chars());
                encoded_instruction.extend(comp(comp_str).chars());
                encoded_instruction.extend(dest(dest_str).chars());
                encoded_instruction.extend(jump(jump_string).chars());
            }
        }
        return encoded_instruction.iter().collect();
    }

    fn dest(mnemonic: &str) -> String {
        let mut dest: [u8; 3] = [0; 3];
        if mnemonic.contains("A") {
            dest[0] = 1;
        }
        if mnemonic.contains("D") {
            dest[1] = 1;
        }
        if mnemonic.contains("M") {
            dest[2] = 1;
        }
        dest.iter()
            .map(|x| format!("{}", x))
            .collect()
    }

    fn jump(mnemonic: &str) -> String {
        let out = match mnemonic {
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            "" => "000",
            _ => {
                panic!("Invalid Jump Mnemonic: {}", mnemonic);
            }
        };
        out.to_string()
    }

    fn comp(mnemonic: &str) -> String {
        let out = match mnemonic {
            "0" => "0101010",
            "1" => "0111111",
            "-1" => "0111010",
            "D" => "0001100",
            "A" => "0110000",
            "!D" => "0001101",
            "!A" => "0110001",
            "-D" => "0001111",
            "-A" => "0110011",
            "D+1" => "0011111",
            "A+1" => "0110111",
            "D-1" => "0001110",
            "A-1" => "0110010",
            "D+A" => "0000010",
            "D-A" => "0010011",
            "A-D" => "0000111",
            "D&A" => "0000000",
            "D|A" => "0010101",
            "M" => "1110000",
            "!M" => "1110001",
            "-M" => "1110011",
            "M+1" => "1110111",
            "M-1" => "1110010",
            "D+M" => "1000010",
            "D-M" => "1010011",
            "M-D" => "1000111",
            "D&M" => "1000000",
            "D|M" => "1010101",
            _ => {
                panic!("Invalid Computation Mnemonic: {}", mnemonic);
            }
        };
        out.to_string()
    }
}
