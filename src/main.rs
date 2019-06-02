use std::fs::File;
use std::io::{BufRead, BufReader};

mod parser;
use parser::*;

fn main() {
    let file = File::open("./tests/example").unwrap();
    let mut f = BufReader::new(file);

    let mut items: Vec<LexItem>;
    items = parse(&mut f);

    for item in items {
        match item.kind {
            LexType::Bundle => {
            }
            _ => {
            }
        }

        println!("{:?}", item);
    }
}
