// use std::collections::HashMap;

#[derive(Debug)]
enum Token {
    TagOpen(String),
    TagClose,
    TagEnd(String),
    Attibute(String),
    AssignTo,
    QuoteOpen,
    QuoteClose,
    Text(String),
}

fn main() {
    // let a = String::from(r#"<a><p></p></a>"#);
    let a = String::from(r#"<div class="tester"> "womp womp" the big cow said </div>"#);
    tokenize(a);
}

fn tokenize(s: String) {
    // let mut char_map: HashMap<char, usize> = HashMap::new();
    //
    // for (i, c) in s.char_indices() {
    //     if !char_map.contains_key(&c) {
    //         char_map.insert(c, i);
    //     }
    // }

    // for (i, c) in s.char_indices() {
    //     if c == '"' {
    //         if let Some(q_pos) = s[i+1..].find('"') {
    //             for j in i..=i+1+q_pos {
    //                 println!("{}", s.chars().nth(j).unwrap());
    //             }
    //             break;
    //         }
    //     }
    // }

    let mut tokens: Vec<Token> = Vec::<Token>::new();

    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '<' => {
                let mut tag = String::new(); 
                while let Some(b) = chars.peek() {
                    if b.is_whitespace() {
                        break;
                    }
                    tag.push(chars.next().unwrap());
                }
                tokens.push(Token::TagOpen(tag));

            }
            '>' => {
                tokens.push(Token::TagClose);
            }

            _ => {}
        }
    }

    for i in tokens {
        println!("{:?}", i);
    }
}
