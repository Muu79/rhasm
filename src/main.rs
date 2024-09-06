use std::{ borrow::BorrowMut, io::{ self, Write as _ }, path::PathBuf };
use rhasm::{ Assembler, Disassembler };
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
    let in_file_path = args.in_file_path;
    let out_file_path = match args.output.as_ref() {
        Some(filename) => filename.clone(),
        None => {
            let mut out_file = in_file_path.clone();
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

    let mut in_file = std::fs::File::open(in_file_path)?;

    let out_file_create_result = std::fs::File::create_new(&out_file_path);
    let mut out_file = out_file_create_result.unwrap_or_else(|_| {
        eprint!(
            "Could not create output file, file {} already exists
            Would you like to overwrite the file? (y/n)",
            out_file_path.display()
        );
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "y" {
            std::fs::File::create(out_file_path).unwrap()
        } else {
            std::process::exit(1);
        }
    });

    let reader = &mut in_file;
    let writer = Some(out_file.borrow_mut());

    if disassemble {
        let args = rhasm::DisassemblerConfig {
            reader,
            writer,
        };
        let mut disassembler = Disassembler::new(args);
        disassembler.write_to_end()?;
        
    } else {
        let assembler = Assembler::build(&in_file, &out_file);
        assembler.unwrap().advance_to_end();
    }
    Ok(())
}
