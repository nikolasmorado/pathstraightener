#[derive(Debug)]
pub enum Token {
    TagOpen(String),
    TagClose,
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
                } else {
                    let mut tag = String::new();
                    while let Some(b) = chars.peek() {
                        if b.is_whitespace() || b == &'>' {
                            break;
                        }
                        tag.push(chars.next().unwrap());
                    }
                    tokens.push(Token::TagOpen(tag));
                }
            }
            '=' => {
                tokens.push(Token::AssignTo);
            }
            '>' => {
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
            c if c.is_whitespace() && !in_quote => {}
            c => {
                let mut text = c.to_string();

                let mut hit_equals = false;
                while let Some(&b) = chars.peek() {
                    if b == '"' || (!in_quote && (b == '<' || b == '>')) {
                        break;
                    } else if b == '=' {
                        hit_equals = true;
                        break;
                    }
                    text.push(chars.next().unwrap())
                }
                if hit_equals {
                    tokens.push(Token::Attribute(text))
                } else {
                    tokens.push(Token::Text(text))
                }
            }
        }
    }

    tokens
}
