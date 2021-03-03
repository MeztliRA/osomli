pub enum TokenType {    
    Heading,
    UnorderedList,
    Paragraph,
    Code,
    BlankLine,
    Comment,
}

pub struct Token {
    pub identifier: TokenType,
    pub text: String,
}