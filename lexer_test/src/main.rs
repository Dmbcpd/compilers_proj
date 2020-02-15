extern crate lrlex;
extern crate lrpar;

use std::io::{self, BufRead, Write};

use lrlex::lrlex_mod;
use lrpar::Lexer;

// Using `lrlex_mod!` brings the lexer for `lexler.l` into scope. By default the module name will be
// `lexer_l` (i.e. the file name, minus any extensions, with a suffix of `_l`).
lrlex_mod!("lexer.l");

fn main() {
    // Get the `LexerDef` for the `Tiny` language.
    let lexerdef = lexer_l::lexerdef();
    let stdin = io::stdin();
    loop {
        print!(">>> ");
        io::stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                // Now we create a lexer with the `lexer` method with which we can lex an input.
                // Note that each lexer can only lex one input in its lifetime.
                let lexer = lexerdef.lexer(l);
                println!("{:?}", lexer.iter().collect::<Vec<_>>());
            }
            _ => break
        }
    }
}