use std::{ io::{ self, Write as _ }, path::PathBuf };
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

    let out_file = match args.output.as_ref() {
        Some(_) =>
            Some(match std::fs::File::create_new(&out_file_path) {
                Ok(file) => file,
                Err(_) => {
                    print!(
                        "File {} already exists\n\
                        If you would like to specify an output file use:\
                        \nrhasm <input_file_path> [-o | --output] <output_file_path>\n\n\
                        Would you like to overwrite {} instead? (y/n): ",
                        out_file_path.file_name().unwrap().to_str().unwrap(),
                        out_file_path.display()
                    );
                    io::stdout().flush().unwrap();
                    let mut response = String::new();
                    io::stdin().read_line(&mut response).unwrap();
                    if response.trim().to_ascii_lowercase() == "y" {
                        std::fs::File::create(&out_file_path).unwrap()
                    } else {
                        return Ok(());
                    }
                }
            }),
        None => None,
    };

    if disassemble {
        let args = rhasm::DisassemblerConfig {
            in_file: &mut in_file,
            out_file: out_file,
        };
        let mut disassembler = Disassembler::new(args);
        disassembler.write_to_end();
    } else {
        let assembler = Assembler::build(&in_file, out_file.as_ref().unwrap());
        assembler.unwrap().advance_to_end();
    }
    Ok(())
}
