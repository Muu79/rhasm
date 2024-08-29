use std::collections::HashMap;
use crate::Instruction;

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
            encoded_instruction.extend(get_comp_code(comp_str).chars());
            encoded_instruction.extend(get_dest_code(dest_str).chars());
            encoded_instruction.extend(get_jump_code(jump_string).chars());
        }
    }
    return encoded_instruction.iter().collect();
}

fn get_dest_code(mnemonic: &str) -> String {
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

fn get_jump_code(mnemonic: &str) -> String {
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

fn get_comp_code(mnemonic: &str) -> String {
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
