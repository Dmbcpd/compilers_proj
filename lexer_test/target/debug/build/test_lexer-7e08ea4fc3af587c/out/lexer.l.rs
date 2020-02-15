mod lexer_l {
use lrlex::{LexerDef, Rule};

#[allow(dead_code)]
pub fn lexerdef() -> LexerDef<u8> {
    let rules = vec![
Rule::new(Some(0), Some("IF".to_string()), "if".to_string()).unwrap(),
Rule::new(Some(1), Some("ELSE".to_string()), "else".to_string()).unwrap(),
Rule::new(Some(2), Some("RETURN".to_string()), "return".to_string()).unwrap(),
Rule::new(Some(3), Some("LENGTH".to_string()), "length".to_string()).unwrap(),
Rule::new(Some(4), Some("WHILE".to_string()), "while".to_string()).unwrap(),
Rule::new(Some(5), Some("CHAR".to_string()), "char".to_string()).unwrap(),
Rule::new(Some(6), Some("WRITE".to_string()), "write".to_string()).unwrap(),
Rule::new(Some(7), Some("READ".to_string()), "read".to_string()).unwrap(),
Rule::new(Some(8), Some("NAME".to_string()), "[A-Za-z][A-Za-z0-9_]*".to_string()).unwrap(),
Rule::new(Some(9), Some("NUMBER".to_string()), "'[0-9]+'".to_string()).unwrap(),
Rule::new(Some(10), Some("QCHAR".to_string()), "'.'".to_string()).unwrap(),
Rule::new(Some(11), Some("COMMENT".to_string()), "\\\\\\\\.*".to_string()).unwrap(),
Rule::new(Some(12), Some("INT".to_string()), "[-]?[1-9][0-9]*|0".to_string()).unwrap(),
Rule::new(Some(13), Some("LPAR".to_string()), "\\(".to_string()).unwrap(),
Rule::new(Some(14), Some("RPAR".to_string()), "\\)".to_string()).unwrap(),
Rule::new(Some(15), Some("LBRACE".to_string()), "\\[".to_string()).unwrap(),
Rule::new(Some(16), Some("RBRACE".to_string()), "\\]".to_string()).unwrap(),
Rule::new(Some(17), Some("LBRACK".to_string()), "\\{".to_string()).unwrap(),
Rule::new(Some(18), Some("RBRACK".to_string()), "\\}".to_string()).unwrap(),
Rule::new(Some(19), Some("NEQUAL".to_string()), "!=".to_string()).unwrap(),
Rule::new(Some(20), Some("ASSIGN".to_string()), "=".to_string()).unwrap(),
Rule::new(Some(21), Some("SEMICOLON".to_string()), ";".to_string()).unwrap(),
Rule::new(Some(22), Some("COMMA".to_string()), ",".to_string()).unwrap(),
Rule::new(Some(23), Some("PLUS".to_string()), "\\+".to_string()).unwrap(),
Rule::new(Some(24), Some("MINUS".to_string()), "-".to_string()).unwrap(),
Rule::new(Some(25), Some("TIMES".to_string()), "\\*".to_string()).unwrap(),
Rule::new(Some(26), Some("DIVIDE".to_string()), "/".to_string()).unwrap(),
Rule::new(Some(27), Some("EQUAL".to_string()), "==".to_string()).unwrap(),
Rule::new(Some(28), Some("GREATER".to_string()), ">".to_string()).unwrap(),
Rule::new(Some(29), Some("LESS".to_string()), "<".to_string()).unwrap(),
Rule::new(Some(30), Some("NOT".to_string()), "!".to_string()).unwrap(),
];
    LexerDef::new(rules)
}
}