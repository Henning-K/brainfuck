use std::iter::Iterator;

#[derive(Clone, PartialEq)]
pub enum Token {
    Inc,
    Dec,
    Fwd,
    Rev,
    Prnt,
    Inp,
    OpenWhile,
    CloseWhile,
}

pub trait TokenizeBF {
    fn tokenize(&self) -> Vec<Token>;
}


impl TokenizeBF for String {
    fn tokenize(&self) -> Vec<Token> {
        // get a peekable iterator over the chars
        let mut it = self.chars().peekable();

        let mut tokens: Vec<Token> = vec![];

        loop {
            match it.peek() {
                Some(&ch) => {
                    match ch {
                        '+' => tokens.push(Token::Inc),
                        '-' => tokens.push(Token::Dec),
                        '<' => tokens.push(Token::Rev),
                        '>' => tokens.push(Token::Fwd),
                        '.' => tokens.push(Token::Prnt),
                        ',' => tokens.push(Token::Inp),
                        '[' => tokens.push(Token::OpenWhile),
                        ']' => tokens.push(Token::CloseWhile),
                        _ => {
                            // any other char, consider it as comment
                        }
                    }
                }
                None => {
                    // No more chars left after the current one, so the loop can end
                    break;
                }
            }
        }
        tokens
    }
}
