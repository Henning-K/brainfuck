use std::io::{self, Read, Write};

use super::*;

pub fn interpret(tokens: &[Token]) -> Result<()> {
    let mut cells = vec![0u8; 30_000];
    let mut ptr = 0usize;
    let mut instruction_ptr = 0usize;

    loop {
        match tokens[instruction_ptr] {
            Token::End => break,
            Token::Inc => cells[ptr] += 1,
            Token::Dec => cells[ptr] -= 1,
            Token::Fwd => ptr += 1,
            Token::Rev => ptr -= 1,
            Token::OpenWhile(n) => {
                if cells[ptr] == 0 {
                    instruction_ptr = n;
                }
            }
            Token::CloseWhile(n) => {
                if cells[ptr] != 0 {
                    instruction_ptr = n;
                }
            }
            Token::Inp => {
                cells[ptr] = io::stdin().bytes().next().unwrap()?;
            }
            Token::Prnt => {
                io::stdout().write(&[cells[ptr]])?;
            }
        }
        instruction_ptr += 1;
    }
    Ok(())
}
