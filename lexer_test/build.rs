extern crate lrlex;
use lrlex::LexerBuilder;

fn main() {
    LexerBuilder::<u8>::new()
        .process_file_in_src("lexer.l")
        .unwrap();
}