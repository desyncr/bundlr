mod types;
pub use types::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn line(_input: String) -> LexItem {
    let mut _kind;
    let mut ins: LexInstruction = LexInstruction::String;
    let mut _args = String::new();

    let mut input = _input.clone();
    // remove comments
    let to = _input.find(COMMENT);
    if to.is_some() && to.unwrap() > 0 {
        input = _input[..to.unwrap()].trim().to_string();
    }

    if input.starts_with(COMMENT) {
        _kind = LexType::Comment;

    } else if input.is_empty() {
        _kind = LexType::Blank;

    } else if input.starts_with(KEYWORD_USE)
        || input.starts_with(KEYWORD_ON)
        || input.starts_with(KEYWORD_FROM)
        || input.starts_with(KEYWORD_AS) {

        if !input.contains(BLOCK_START) {
            // what about: "use: oh-my-zsh"?
            // panic!("Keyword must open a block:\n\t{}", input);
        }

        _kind = LexType::Keyword;

        if input.starts_with(KEYWORD_USE) {
            ins = LexInstruction::Use;
        }

        if input.starts_with(KEYWORD_ON) {
            ins = LexInstruction::On;
        }

        if input.starts_with(KEYWORD_FROM) {
            ins = LexInstruction::From;
        }

        if input.starts_with(KEYWORD_AS) {
            ins = LexInstruction::As;
        }

        let from = input.find(KEYWORD_DELIMITER).unwrap();
        let to = input.find(BLOCK_START).unwrap();
        _args = input[from+1..to]
            .replace(BLOCK_START, BLANK) // remove any possible block
            .trim() // remove spaces
            .to_string();

    } else {
        _kind = LexType::Bundle;

    }

    LexItem{
        kind: _kind,
        line: input,
        instruction: ins,
        args: _args,
        child: vec![]
    }
}

fn bundle(input: LexItem) -> Bundle {
    let source = Source{name: String::from("github"), host: String::from("https://github.com")};

    Bundle {
        source: source,
        name: String::new(),
        kind: BundleType::Plugin,
        version: String::new(),
        modifiers: vec![]
    }
}

pub fn parse(f: &mut BufReader<File>) -> Vec<LexItem> {
    let mut items: Vec<LexItem> = vec![];
    let mut data = String::new();

    while f.read_line(&mut data).unwrap() > 0
        && !data.starts_with(FILE_END)
    {
        let mut item = line(data.trim().to_string());

        if data.contains(BLOCK_START) {
            item.child = parse(f);
        }

        if data.contains(BLOCK_END) {
            break;
        }

        items.push(item);
        data.clear();
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_comment() {
        let item: LexItem = line(String::from("# vundle/example"));

        assert_eq!(item.kind, LexType::Comment);
        assert_eq!(item.instruction, LexInstruction::String);
        assert_eq!(item.args, String::from(""));
        assert_eq!(item.child, vec![]);
    }

    #[test]
    fn test_parsing_bundle_url() {
        let item: LexItem = line(String::from("vundle/example"));

        assert_eq!(item.kind, LexType::Bundle);
        assert_eq!(item.line, "vundle/example");
        assert_eq!(item.instruction, LexInstruction::String);
        assert_eq!(item.args, String::from(""));
        assert_eq!(item.child, vec![]);
    }

    #[test]
    fn test_parsing_block_modifiers() {
        let item: LexItem = line(String::from("on: darwin {
            vundle/example
            other/example
        }"));

        assert_eq!(item.kind, LexType::Keyword);
        assert_eq!(item.instruction, LexInstruction::On);
        assert_eq!(item.args, String::from("darwin"));
    }

    #[test]
    fn test_parsing_line_modifiers() {
        let item: LexItem = line(String::from("vundle/example on:darwin"));

        assert_eq!(item.kind, LexType::Bundle);
    }

}
