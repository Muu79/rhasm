pub mod assembler;
pub mod encoder;
use std::{env, io};
use assembler::Assembler;

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let filename: &str = match args.get(1){
        Some(filename) => filename,
        None => "default.asm",
    };
    let out_file = &env::args().nth(2).unwrap_or("default.hack".to_string());
    println!("Reading file {}", filename);
    let mut assembler = Assembler::new(filename, out_file);
    assembler.advance_to_end();
    Ok(())
}
