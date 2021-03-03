use crate::tokens::*;
use titlecase::titlecase;

pub fn interpret(tokens: Vec<Token>) {
    for token in tokens {
        match token.identifier {
            TokenType::BlankLine => println!("{}", token.text),
            TokenType::Paragraph => println!("{}", token.text),
            TokenType::UnorderedList => println!("- {}", token.text),
            TokenType::Heading => {
                let header = &token.text.trim_end();                
                println!("{}", titlecase(header));
            },
            TokenType::Code => {
                let text_length = token.text.chars().count();
                let mut bound = String::new();

                for _character in 0..=text_length {
                    bound.push_str("_");
                }

                println!("{}", &bound);
                println!("{}", token.text);
                println!("{}", &bound);
            },
            TokenType::Comment => (),
        }
    }
}