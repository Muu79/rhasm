use std::error::Error;

/// Decode an encoded instruction into a human readable instruction.
/// Labels and variables are lost in the encoding process.
/// Thus the decoded instruction will not be the same as the original instruction.
/// But will still assemble back into the same machine code.
pub fn decode_instruction(encoded_instruction: &str) -> Result<String, Box<dyn Error>> {
    let mut decoded_instruction = String::new();
    let char_count = encoded_instruction.len();
    if char_count != 16 {
        return Err(format!("Invalid encoded instruction length - Expected 16 found {}", char_count).into());
    } else if !encoded_instruction.chars().all(|char| char.is_digit(2)) {
        return Err("Invalid encoded instruction, please make sure instruction is in binary".into());
    }

    let first_char = encoded_instruction.chars().next().unwrap();
    if first_char == '0' {
        let addr = &encoded_instruction[1..];
        decoded_instruction.push_str(&format!("@{}", u16::from_str_radix(addr, 2).unwrap()));
    } else {
        let comp = decode_comp(&encoded_instruction[3..10]);
        let dest = decode_dest(&encoded_instruction[10..13]);
        let jump = decode_jump(&encoded_instruction[13..]);
        if let Some(dest) = dest {
            decoded_instruction.push_str(dest);
            decoded_instruction.push('=');
        }
        if let None = comp {
            return Err(format!("Invalid comp mnemonic {}", &encoded_instruction[3..9]).into());
        } else {
            decoded_instruction.push_str(comp.unwrap());
        }
        if let Some(jump) = jump {
            decoded_instruction.push(';');
            decoded_instruction.push_str(jump);
        }
    }
    Ok(decoded_instruction)
}

fn decode_dest(encoded_dest: &str) -> Option<&str> {
    match encoded_dest {
        "000" => None,
        "001" => Some("M"),
        "010" => Some("D"),
        "011" => Some("MD"),
        "100" => Some("A"),
        "101" => Some("AM"),
        "110" => Some("AD"),
        "111" => Some("AMD"),
        _ => None,
    }
}

fn decode_comp(encoded_comp: &str) -> Option<&str> {
    match encoded_comp {
        "0101010" => Some("0"),
        "0111111" => Some("1"),
        "0111010" => Some("-1"),
        "0001100" => Some("D"),
        "0110000" => Some("A"),
        "0001101" => Some("!D"),
        "0110001" => Some("!A"),
        "0001111" => Some("-D"),
        "0110011" => Some("-A"),
        "0011111" => Some("D+1"),
        "0110111" => Some("A+1"),
        "0001110" => Some("D-1"),
        "0110010" => Some("A-1"),
        "0000010" => Some("D+A"),
        "0010011" => Some("D-A"),
        "0000111" => Some("A-D"),
        "0000000" => Some("D&A"),
        "0010101" => Some("D|A"),
        "1110000" => Some("M"),
        "1110001" => Some("!M"),
        "1110011" => Some("-M"),
        "1110111" => Some("M+1"),
        "1110010" => Some("M-1"),
        "1000010" => Some("D+M"),
        "1010011" => Some("D-M"),
        "1000111" => Some("M-D"),
        "1000000" => Some("D&M"),
        "1010101" => Some("D|M"),
        _ => None,
    }
}

fn decode_jump(encoded_jump: &str) -> Option<&str> {
    match encoded_jump {
        "000" => None,
        "001" => Some("JGT"),
        "010" => Some("JEQ"),
        "011" => Some("JGE"),
        "100" => Some("JLT"),
        "101" => Some("JNE"),
        "110" => Some("JLE"),
        "111" => Some("JMP"),
        _ => None,
    }
}
