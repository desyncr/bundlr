pub const KEYWORD_USE       : &str = "use:";
pub const KEYWORD_ON        : &str = "on:";
pub const KEYWORD_FROM      : &str = "from:";
pub const KEYWORD_AS        : &str = "as:";
pub const KEYWORD_DELIMITER : &str = ":";
pub const COMMENT           : &str = "#";
pub const BLOCK_START       : &str = "{";
pub const BLOCK_END         : &str = "}";
pub const BLANK             : &str = "";
pub const FILE_END          : &str = "--";

#[derive(Debug)]
#[derive(PartialEq)]
pub enum LexType {
    Keyword,
    Comment,
    Blank,
    Bundle
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum LexInstruction {
    Use,
    On,
    From,
    As,
    String
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct LexItem {
    pub kind: LexType,
    pub instruction: LexInstruction,
    pub args: String,
    pub line: String,
    pub child: Vec<LexItem>
}

pub struct Source {
    pub name: String,
    pub host: String
}

pub enum BundleType {
    Theme,
    Plugin
}

pub enum LexModifier {
    Use,
    On,
    From,
    As
}

pub struct BundleModifier {
    pub kind: LexModifier,
    pub args: String
}

pub struct Bundle {
    pub source: Source,
    pub name: String,
    pub kind: BundleType,
    pub version: String,
    pub modifiers: Vec<BundleModifier>
}
