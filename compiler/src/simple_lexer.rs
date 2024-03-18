// case
// age >= 45
// int age = 40
// 2+3*5

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleToken {
    Initial,
    Identifier,
    GT,
    GE,
    IntLiteral,
    Assignment,
    Int,
    Plus,
    Minus,
    Star,
    Slash,
}
pub type TokenGroup = (SimpleToken, String);

#[derive(Debug)]
pub struct SimpleLexer {
    pub value: String,
    pub tokens: Vec<TokenGroup>,
    pub token_now: SimpleToken,
}

impl SimpleLexer {
    pub fn new(value: String) -> SimpleLexer {
        SimpleLexer {
            value,
            tokens: vec![],
            token_now: SimpleToken::Initial,
        }
    }
    pub fn execute(&mut self) {
        self.value.chars().for_each(|x| match x {
            'a'..='z' | 'A'..='Z' => match self.token_now {
                SimpleToken::Identifier | SimpleToken::Int => {
                    let last = self.tokens.last_mut().unwrap();
                    if let SimpleToken::Int = self.token_now {
                        if last.1 == "i" && x == 'n' {
                            self.token_now = SimpleToken::Int;
                        } else if last.1 == "in" && x == 't' {
                            self.token_now = SimpleToken::Int;
                        } else {
                            self.token_now = SimpleToken::Identifier;
                        }
                    }
                    last.1.push(x);
                }
                _ => {
                    if x == 'i' {
                        self.token_now = SimpleToken::Int;
                        self.tokens.push((SimpleToken::Int, x.to_string()));
                    } else {
                        self.token_now = SimpleToken::Identifier;
                        self.tokens.push((SimpleToken::Identifier, x.to_string()));
                    }
                }
            },
            '0'..='9' => {
                if let SimpleToken::IntLiteral = self.token_now {
                    let last = self.tokens.last_mut().unwrap();
                    last.1.push(x);
                } else {
                    self.tokens.push((SimpleToken::IntLiteral, x.to_string()));
                }
                self.token_now = SimpleToken::IntLiteral;
            }
            '>' => {
                self.tokens.push((SimpleToken::GT, x.to_string()));
                self.token_now = SimpleToken::GT;
            }
            '=' => {
                if let SimpleToken::Initial = self.token_now {
                    self.tokens.push((SimpleToken::Assignment, "=".to_string()));
                } else {
                    let last = self.tokens.last_mut().unwrap();
                    if let SimpleToken::GT = self.token_now {
                        last.0 = SimpleToken::GE;
                        last.1.push(x);
                    } else {
                        last.0 = SimpleToken::Assignment;
                        last.1.push(x);
                    }
                }
                self.token_now = SimpleToken::Initial;
            }
            '+' => {
                self.token_now = SimpleToken::Initial;
                self.tokens.push((SimpleToken::Plus, x.to_string()));
            }
            '-' => {
                self.token_now = SimpleToken::Initial;
                self.tokens.push((SimpleToken::Minus, x.to_string()));
            }
            '*' => {
                self.token_now = SimpleToken::Initial;
                self.tokens.push((SimpleToken::Star, x.to_string()));
            }
            '/' => {
                self.token_now = SimpleToken::Initial;
                self.tokens.push((SimpleToken::Slash, x.to_string()));
            }
            _ => {
                self.token_now = SimpleToken::Initial;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut lexer = SimpleLexer::new("ageY >= 45".to_string());
        lexer.execute();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 3);
        assert_eq!(lexer.tokens[0].0, SimpleToken::Identifier);
        assert_eq!(lexer.tokens[1].0, SimpleToken::GE);
        assert_eq!(lexer.tokens[2].0, SimpleToken::IntLiteral);
    }
    #[test]
    fn test_2() {
        let mut lexer = SimpleLexer::new("AsomeZ > 40".to_string());
        lexer.execute();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 3);
        assert_eq!(lexer.tokens[0].0, SimpleToken::Identifier);
        assert_eq!(lexer.tokens[1].0, SimpleToken::GT);
        assert_eq!(lexer.tokens[2].0, SimpleToken::IntLiteral);
    }
    #[test]
    fn test_3() {
        let mut lexer = SimpleLexer::new("int age = 40".to_string());
        lexer.execute();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 4);
    }
    #[test]
    fn test_4() {
        let mut lexer = SimpleLexer::new("2+3*5".to_string());
        lexer.execute();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 5);
    }
}
