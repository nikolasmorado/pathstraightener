#[derive(Debug)]
pub enum Token {
    TagOpen(String),
    TagClose,
    TagSelfClose,
    TagEnd(String),
    Attribute(String),
    AssignTo,
    QuoteOpen,
    QuoteClose,
    Text(String),
}

pub fn tokenize(s: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::<Token>::new();

    let mut chars = s.chars().peekable();

    let mut in_quote = false;
    let mut in_tag = false;
    while let Some(c) = chars.next() {
        match c {
            '<' => {
                if chars.peek() == Some(&'/') {
                    chars.next().unwrap();
                    let mut tag = String::new();
                    while let Some(b) = chars.peek() {
                        if b == &'>' {
                            break;
                        }
                        tag.push(chars.next().unwrap());
                    }
                    tokens.push(Token::TagEnd(tag));
                } else if chars.peek() == Some(&'!') {
                    chars.next().unwrap();
                    while let Some(b) = chars.peek() {
                        if b == &'>' {
                            chars.next().unwrap();
                            break;
                        }
                        chars.next().unwrap();
                    }
                } else {
                    let mut tag = String::new();
                    while let Some(b) = chars.peek() {
                        if b.is_whitespace() || b == &'>' {
                            break;
                        }
                        tag.push(chars.next().unwrap());
                    }
                    tokens.push(Token::TagOpen(tag));
                    in_tag = true;
                }
            }
            '=' => {
                tokens.push(Token::AssignTo);
            }
            '>' => {
                in_tag = false;
                tokens.push(Token::TagClose);
            }
            '"' => {
                if in_quote {
                    tokens.push(Token::QuoteClose);
                    in_quote = false;
                } else {
                    tokens.push(Token::QuoteOpen);
                    in_quote = true;
                }
            }
            '/' => {
                if in_tag {
                    if let Some(b) = chars.peek() {
                        if b == &'>' {
                            chars.next().unwrap();
                            tokens.push(Token::TagSelfClose);
                        }
                    }
                }
            }
            c if c.is_whitespace() && !in_quote => {}
            c => {
                let mut text = c.to_string();

                let mut hit_equals = false;
                let mut hit_close = false;
                let mut quote_found = true;
                while let Some(&b) = chars.peek() {
                    if b == '<' && !in_tag {
                        break;
                    } else if b == '>' && in_tag {
                        hit_close = true;
                        break;
                    } else if !in_quote && (b == '<' || b == '>') {
                        break;
                    } else if b == '"' && in_tag && !quote_found {
                        quote_found = true;
                    } else if b == '"' && in_tag && quote_found {
                        break;
                    } else if b == '=' && in_tag {
                        hit_equals = true;
                        break;
                    }
                    text.push(chars.next().unwrap())
                }

                if hit_equals || hit_close {
                    tokens.push(Token::Attribute(text))
                } else {
                    tokens.push(Token::Text(text))
                }
            }
        }
    }

    tokens
}
