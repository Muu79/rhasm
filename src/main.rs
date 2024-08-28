pub mod assembler;
pub mod encoder;
use std::{ env, io };
use assembler::Assembler;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let in_file: &str = match args.get(1) {
        Some(filename) => filename,
        None => "sample.asm",
    };
    let out_file: &str = match args.get(2) {
        Some(filename) => &filename,
        None => "sample.hack"
    };
    println!("Reading file {}", in_file);

    let in_file = std::fs::File::open(in_file)?;
    let out_file = std::fs::File::create(out_file)?;
    let mut assembler = Assembler::new(Some(in_file), Some(out_file));
    assembler.advance_to_end();
    Ok(())
}
