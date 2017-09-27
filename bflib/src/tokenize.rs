use super::*;

#[derive(Clone, PartialEq)]
pub enum Token {
    Inc,
    Dec,
    Fwd,
    Rev,
    Prnt,
    Inp,
    OpenWhile(usize),
    CloseWhile(usize),
    End,
}

pub trait TokenizeBF {
    fn tokenize(&self) -> Result<Vec<Token>>;
}


impl TokenizeBF for String {
    fn tokenize(&self) -> Result<Vec<Token>> {
        let mut tokens: Vec<Token> = vec![];
        let mut stack: Vec<usize> = vec![];

        for (i, ch) in self.chars().enumerate() {
            match ch {
                '+' => tokens.push(Token::Inc),
                '-' => tokens.push(Token::Dec),
                '<' => tokens.push(Token::Rev),
                '>' => tokens.push(Token::Fwd),
                '.' => tokens.push(Token::Prnt),
                ',' => tokens.push(Token::Inp),
                '[' => {
                    let l = tokens.len();
                    tokens.push(Token::OpenWhile(0));
                    stack.push(l);
                    // println!("[ at {}", l);
                },
                ']' => {
                    let n = stack.pop().chain_err(|| "Unbalanced square brackets")?;
                    let l = tokens.len();
                    tokens.push(Token::CloseWhile(n));
                    // println!("[ at {}, ] at {}", n, l);
                    tokens[n] = Token::OpenWhile(l);
                },
                _ => {
                    // any other char, consider it a comment
                }
            }
        }
        tokens.push(Token::End);
        Ok(tokens)
    }
}
