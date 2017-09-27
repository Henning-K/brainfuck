#[macro_use]
extern crate serde_derive;

extern crate docopt;
use docopt::Docopt;

#[macro_use]
extern crate error_chain;

extern crate bflib;
use bflib::*;

use std::fs::File;
use std::path::Path;
use std::io::{self, Read};

const USAGE: &'static str = "
Usage: brainfuck [options] INPUT
       brainfuck (--help | --version)

Options:
    -h, --help      Show this message.
    --version       Show the version of this program.
";

error_chain! {
    links {
        BFLib(::bflib::Error, ::bflib::ErrorKind);
    }

    foreign_links {
        Io(io::Error);
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Args {
    arg_INPUT: String,
    flag_version: bool,
}

fn handle_input<P: AsRef<Path>>(inp: P) -> Result<Vec<Token>> {
    let mut f = try!(File::open(inp));
    let mut buf = String::new();

    f.read_to_string(&mut buf)?;
    buf.tokenize().chain_err(|| "Tokenization failed.")
}

fn run() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(version.to_string())).options_first(true).deserialize())
        .unwrap_or_else(|e| e.exit());
    // println!("{:?}", args);
    
    let tokens = handle_input(&args.arg_INPUT)?;

    interpret(&tokens).chain_err(|| "Interpretation failed.")
}

quick_main!(run);
