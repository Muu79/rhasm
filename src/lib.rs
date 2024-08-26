pub mod file_parser {
    use regex::Regex;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{ BufRead, BufReader, BufWriter, Lines, Write };
    use std::iter::Peekable;

    use crate::encoder;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Instruction {
        AInstructionUnresolved(String),
        AInstruction(u16),
        CInstruction(String),
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
            let line = line.split("//").next().unwrap().trim();

            if !line.is_empty() {
                // Handle Address Instructions
                if line.starts_with("@") {
                    let expr = Regex::new(r"(?:@)(\S+)").unwrap();

                    let addr: String;

                    if let Some(cap) = expr.captures(line) {
                        if let Some(matched) = cap.get(1) {
                            addr = matched.as_str().to_string();
                        } else {
                            panic!(
                                "Invalid A-Instruction address at line: {} {}",
                                self.cur_line,
                                line
                            );
                        }
                    } else {
                        panic!("Invalid A-Instruction address at line: {} {}", self.cur_line, line);
                    }
                    let num_address = addr.parse::<u16>();
                    match num_address {
                        Ok(addr) => {
                            self.instructions.push(Instruction::AInstruction(addr));
                        }
                        Err(_) => {
                            self.instructions.push(Instruction::AInstructionUnresolved(addr));
                        }
                    }
                } else if
                    // Handle Label Pseudo Instructions
                    // We don't actually need a label-instruction Enum,
                    // We just need to make sure it's symbol is in the symbol table
                    line.starts_with("(")
                {
                    let expr: Regex = Regex::new(r"\((\S+)\)").unwrap();
                    let symbol: String;
                    // This complex statement uses the regex to extract the symbol from the line
                    // It does this through "if let" pattern matching which is somewhat confusing
                    // It's simply a shorthand since we need to unwrap multiple times
                    if let Some(capture) = expr.captures(line) {
                        if let Some(matched) = capture.get(1) {
                            symbol = matched.as_str().to_string();
                            self.symbol_table.insert(
                                symbol.clone(),
                                // We use the current instruction count as the address of the label
                                self.instructions.len().try_into().unwrap()
                            );
                        }
                    }
                } else {
                    // Finally, handle Compute Instructions
                    self.instructions.push(Instruction::CInstruction(line.to_owned()));
                }
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
            if out.len() != 16 {
                match self.instructions.get((self.cur_instruction as usize) - 1).unwrap() {
                    Instruction::AInstructionUnresolved(symbol) => {
                        panic!("Unresolved Symbol: {}", symbol);
                    }
                    _ => {
                        panic!(
                            "Invalid Instruction: {:?}",
                            self.instructions.get((self.cur_instruction as usize) - 1).unwrap()
                        );
                    }
                }
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
    use regex::Regex;
    use crate::file_parser::Instruction;

    const COMP_REGEX: &str =
        r"(?:(?P<dest>(?:A?M?D?)|(?:A?D?M?)|(?:D?A?M?)|(?:D?M?A?)|(?:M?A?D?)|(?:M?D?A?))=)?(?P<comp>[01\-ADM!+&|]+)(?:;(?P<jmp>[a-zA-Z]+))?";
    pub fn encode_instruction(
        instruction: &Instruction,
        symbol_table: &mut HashMap<String, u16>,
        cur_ram: &mut u16
    ) -> String {
        let mut encoded_instruction: Vec<char> = vec![];
        match instruction {
            Instruction::AInstruction(addr) => {
                encoded_instruction.push('0');
                let binary_addr = format!("{:015b}", addr);
                encoded_instruction.extend(binary_addr.chars());
            }
            Instruction::AInstructionUnresolved(symbol) => {
                let addr = symbol_table.get(symbol);
                encoded_instruction.push('0');
                match addr {
                    Some(addr) => {
                        let binary_addr = format!("{:015b}", addr);
                        encoded_instruction.extend(binary_addr.chars());
                    }
                    None => {
                        symbol_table.insert(symbol.clone(), *cur_ram);
                        let binary_addr = format!("{:015b}", cur_ram);
                        encoded_instruction.extend(binary_addr.chars());
                        *cur_ram += 1;
                    }
                }
            }
            Instruction::CInstruction(c_inst_str) => {
                let expression = Regex::new(COMP_REGEX).unwrap();
                encoded_instruction.extend("111".chars());
                if let Some(captures) = expression.captures(&c_inst_str) {

                    let comp_str = captures.name("comp");
                    if let Some(comp_str) = comp_str {
                        let comp_str = comp_str.as_str();
                        encoded_instruction.extend(comp(&comp_str).chars());
                    } else {
                        panic!("Invalid C-Instruction: {}", c_inst_str);
                    }

                    let dest_str = captures.name("dest");
                    if let Some(dest_str) = dest_str {
                        let dest_str = dest_str.as_str();
                        encoded_instruction.extend(dest(&dest_str).chars());
                    }else{
                        encoded_instruction.extend("000".chars());
                    }

                    let jump_str = captures.name("jmp");
                    if let Some(jump_str) = jump_str {
                        let jump_str = jump_str.as_str();
                        encoded_instruction.extend(jump(&jump_str).chars());
                    }else{
                        encoded_instruction.extend("000".chars());
                    }
                } else {
                    panic!("Invalid C-Instruction: {}", c_inst_str);
                }
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
