use crate::utils::ui::get_text;

use crate::xsdb::{query::Query, tokenizer::Tokenizer};

pub fn main() {
    let input = get_text("Enter your query: ");
    let mut tokenizer = Tokenizer::new(input.as_str());
    let tokens = tokenizer.tokenize().unwrap();
    let query = Query::new(tokens);
    println!("{:?}", query);
}
