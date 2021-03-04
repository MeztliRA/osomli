pub enum TokenType {    
    Heading,
    UnorderedList,
    Paragraph,
    Code,
    BlankLine,
    Comment,
    BoldParagraph,
}

pub struct Token {
    pub identifier: TokenType,
    pub text: String,
}