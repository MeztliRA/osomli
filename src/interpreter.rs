use crate::tokens::*;
use titlecase::titlecase;
use crossterm::style::Styler;

pub fn interpret(tokens: Vec<Token>) {
    for token in tokens {
        match token.identifier {
            TokenType::BlankLine => println!("{}", token.text),
            TokenType::Paragraph => println!("{}", token.text),
            TokenType::UnorderedList => println!("- {}", token.text),
            TokenType::Heading => {
                let header = &token.text.trim_end();
                let header_length = header.len();
                let mut line = String::new();
                
                for _ in 0..=header_length {
                    line.push_str("_");
                }
                println!("{}", titlecase(header));
                println!("{}", line);
            },
            TokenType::Code => {
                let text_length = token.text.chars().count();
                let mut bound = String::new();

                for _ in 0..=text_length {
                    bound.push_str("_");
                }

                println!("{}", &bound);
                println!("{}", token.text);
                println!("{}", &bound);
            },
            TokenType::Comment => (),
            TokenType::BoldParagraph => {
                println!("{}", token.text.bold());
            },
        }
    }
}