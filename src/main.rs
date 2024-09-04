use std::{ io, path::PathBuf };
use rhasm::{Assembler, Disassembler};
use clap::{ Parser, ArgAction };

#[derive(Parser, Debug)]
#[command(
    name = "rhasm",
    version = "0.1.1",
    about = "A simple assembler/disassembler for the Hack computer from the Nand2Tetris course",
    author = "Muaaz Bhyat muu794@gmail.com"
)]
struct Cli {
    /// The input file to read from
    /// Is required and does not have an option switch
    #[arg(required = true)]
    in_file_path: PathBuf,

    /// The output file to write
    /// Can be specified with the -o or --output option
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Disassemble the input file
    #[arg(short, long, action = ArgAction::SetTrue)]
    disassemble: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    
    let disassemble = args.disassemble;
    let in_file = args.in_file_path;
    let out_file = match args.output {
        Some(filename) => filename.clone(),
        None => {
            let mut out_file = in_file.clone();
            match disassemble {
                true => {
                    out_file.set_extension("asm");
                }
                false => {
                    out_file.set_extension("hack");
                }
            }
            out_file
        }
    };

    let in_file = std::fs::File::open(in_file)?;
    let out_file = std::fs::File::create(out_file)?;
    if !disassemble {
        let assembler = Assembler::build(&in_file, &out_file);
        assembler.unwrap().advance_to_end();
    }else {
        let disassembler = Disassembler::new(&in_file, &out_file);
        let mut disassembler = disassembler;
        println!("{}", disassembler.advance_to_end());
    }
    Ok(())
}
