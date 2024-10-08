# RHASM (Rust Hack Assembler)

This is a simple assembler for the Hack assembly language, written in Rust. The assembler can be used as a cli tool or as a library in your Rust project. Rhasm requires the input file to be a valid Hack assembly file, and it will output a Hack machine code file. The assembler supports all Hack assembly instructions, including A-instructions, C-instructions, and L-instructions. An example of a valid Hack assembly file is included as [`sample.asm`](https://github.com/Muu79/rhasm/blob/main/sample.asm).

## Installation

### As a CLI tool

To install the cli tool make sure cargo is installed (you can install cargo from [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)), then run the following command:

```bash
cargo install rhasm
```

### As a library

To use the library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
rhasm = "0.1.0"
```

Or you can use cargo to add the dependency:

```bash
cargo add rhasm
```

Then import the library in your project:

```rust
use rhasm;

let asm = rhasm::Assembler::build(&in_file, &out_file);
// Then you can use the asm object to assemble the file
```

## Usage

Rhasm exposes two ways to assemble Hack assembly code, the first is through a binary cli tool and the second is through a library.

### CLI Example

To use rhasm as a cli tool, you can run the following command:

```bash
rhasm <input_file> [-o | --output <output_file>] [-d | --disassemble [--with_symbols <symbol_file>]]
```
### Library Examples

Code examples can be found in the crate's [documentation](https://docs.rs/rhasm/0.1.2)
