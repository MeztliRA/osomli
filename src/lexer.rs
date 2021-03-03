use crate::tokens::*;

pub fn lex(file: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    
    for line in file.lines() {
        let parts: Vec<&str> = line.split(".").collect();

        if line.is_empty() {
            tokens.push(Token {
                identifier: TokenType::BlankLine,
                text: String::from("\n"),
            });
        } else {
            match parts[0] {
                "H" => {
                    tokens.push(Token {
                        identifier: TokenType::Heading,
                        text: String::from(parts[1]),
                    });
                },
                "P" => {
                    tokens.push(Token {
                        identifier: TokenType::Paragraph,
                        text: String::from(parts[1]),
                    });
                },
                "C" => {
                    tokens.push(Token {
                        identifier: TokenType::Code,
                        text: String::from(parts[1]),
                    });
                },
                "UL" => {
                    tokens.push(Token {
                        identifier: TokenType::UnorderedList,
                        text: String::from(parts[1]),
                    });
                },
                "//" => {
                    tokens.push(Token {
                        identifier: TokenType::Comment,
                        text: String::from(parts[1]),
                    });
                },
                _ => panic!("unrecognized identifier."),
            }
        }
    }

    tokens
}