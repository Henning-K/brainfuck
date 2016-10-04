extern crate getopts;
use getopts::Options;

extern crate brainfuck;
use brainfuck::tokenize::*;

use std::io;
use std::env;
use std::mem;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::iter::Iterator;

fn execute_block(input: &Vec<Token>, data: &Vec<u8>, pos: usize) -> (Vec<u8>, usize) {
    let mut offset = pos;
    let mut data = data;
    let mut it = input.iter().skip(offset);
    for tok in it {
        match *tok {
            Token::CloseWhile => {
                if data[pos] == 0u8 {
                    return (*data, offset);
                } else {
                    it = input.iter().skip(pos);
                }
            }
            Token::OpenWhile => {
                return execute_block(&input, &data, offset);
            }
            Token::Fwd => {
                offset += 1;
            }
            Token::Rev => {
                offset -= 1;
            }
            Token::Inc => {
                data[offset] += 1;
            }
            Token::Dec => {
                data[offset] -= 1;
            }
            Token::Prnt => {
                print!("{}", data[offset]);
            }
            Token::Inp => {
                let mut buffer = [0u8; 1];
                match io::stdin().read_exact(&mut buffer) {
                    Err(err) => println!("error occured while reading a char: {}", err.to_string()),
                    Ok(_) => {}
                }
                data[offset] = buffer[0];
            }
        }
        offset += 1;
    }
    (*data, offset)
}

fn execute_program(input: Vec<Token>) {
    let temp = input.clone();
    let mut it = temp.iter().enumerate();
    let mut it_ = it.clone();
    let mut data = vec![0u8; 30000];
    let mut ptr = 0usize;
    let (opened, closed) = (input.iter()
                                 .filter(|c| match **c {
                                     Token::OpenWhile => true,
                                     _ => false,
                                 })
                                 .count(),
                            input.iter()
                                 .filter(|c| match **c {
                                     Token::CloseWhile => true,
                                     _ => false,
                                 })
                                 .count());
    if opened != closed {
        panic!("number of opening/closing braces is not balanced!");
    }
    loop {
        let (offset, tok) = match it.next() {
            Some((o, t)) => (o, t),
            _ => break,
        };
        match *tok {
            Token::CloseWhile => {}
            Token::OpenWhile => {
                let (data_, new_pos) = execute_block(&input, &data, offset);
                mem::replace(&mut data, data_);
                for _ in 0..(new_pos - offset) {
                    it.next();
                }
            }
            Token::Fwd => {
                ptr += 1;
            }
            Token::Rev => {
                ptr -= 1;
            }
            Token::Inc => {
                data[ptr] += 1;
            }
            Token::Dec => {
                data[ptr] -= 1;
            }
            Token::Prnt => {
                print!("{}", data[ptr]);
            }
            Token::Inp => {
                let mut buffer = [0u8];
                match io::stdin().read_exact(&mut buffer) {
                    Err(err) => println!("error occured while reading a char: {}", err.to_string()),
                    Ok(_) => {}
                }
                data[ptr] = buffer[0];
            }
        }
    }
}

fn handle_input<P: AsRef<Path>>(inp: P) -> io::Result<Vec<Token>> {
    let mut f = try!(File::open(inp));
    let mut buf = String::new();

    try!(f.read_to_string(&mut buf));
    let output = buf.tokenize();
    Ok(output)
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => panic!(err.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let input: &str = match matches.free.is_empty() {
        false => &matches.free[0],
        true => {
            print_usage(&program, opts);
            return;
        }
    };
    let tokens = match handle_input(&input) {
        Ok(t) => t,
        Err(err) => panic!(err.to_string()),
    };

    execute_program(tokens);
}
