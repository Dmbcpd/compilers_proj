mod lexer_l {
use lrlex::{LexerDef, Rule};

#[allow(dead_code)]
pub fn lexerdef() -> LexerDef<u8> {
    let rules = vec![
Rule::new(Some(0), Some("FLOAT".to_string()), "([0-9]+\\.[0-9]*)|([0-9]*\\.[0-9]+)".to_string()).unwrap(),
Rule::new(Some(1), Some("INT".to_string()), "[0-9]+".to_string()).unwrap(),
Rule::new(Some(2), Some("PLUS".to_string()), "\\+".to_string()).unwrap(),
Rule::new(Some(3), Some("MUL".to_string()), "\\*".to_string()).unwrap(),
Rule::new(Some(4), Some("MINUS".to_string()), "-".to_string()).unwrap(),
Rule::new(Some(5), Some("DIV".to_string()), "/".to_string()).unwrap(),
Rule::new(Some(6), Some("LBRACK".to_string()), "\\(".to_string()).unwrap(),
Rule::new(Some(7), None, "[\\t ]+".to_string()).unwrap(),
];
    LexerDef::new(rules)
}
}