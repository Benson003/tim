mod cmd;
mod parser;
mod tokenizer;

use crate::cmd::cmd::load_file_from_args;
use crate::parser::parser::Parser;
use crate::tokenizer::tokenizer::TokenList;

fn main() {
    let source = load_file_from_args().unwrap();
    let mut tokens = TokenList::new();
    tokens.tokenize(source.as_str());
    let mut parser = Parser::new(tokens.tokens);
    println!("{:#?}", parser.parse());
}
