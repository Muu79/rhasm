use std::{ env, io, path::PathBuf };
use rhasm::Assembler;
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
    #[arg(required = true)]
    in_file_path: PathBuf,

    /// The output file to write to
    #[arg(short, long)]
    out_file_path: Option<PathBuf>,

    /// Disassemble the input file
    #[arg(short, long, action = ArgAction::SetTrue)]
    disassemble: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    
    let in_file = args.in_file_path;
    let out_file = match args.out_file_path {
        Some(filename) => filename.clone(),
        None => {
            let mut out_file = in_file.clone();
            out_file.set_extension("hack");
            out_file
        },
    };
    let dissassemble = args.disassemble;

    let in_file = std::fs::File::open(in_file)?;
    let out_file = std::fs::File::create(out_file)?;
    let assembler = Assembler::build(&in_file, &out_file);
    assembler.unwrap().advance_to_end();
    Ok(())
}
