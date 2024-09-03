use std::{ env, io };
use rhasm::Assembler;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let in_file: &str = match args.get(1) {
        Some(filename) => filename,
        None => "sample.asm",
    };
    let out_file: &str = match args.get(2) {
        Some(filename) => &filename,
        None => "sample.hack",
    };
    println!("Reading file {}", in_file);

    let in_file = std::fs::File::open(in_file)?;
    let out_file = std::fs::File::create(out_file)?;
    let assembler = Assembler::build(&in_file, &out_file);
    assembler.unwrap().advance_to_end();
    Ok(())
}
