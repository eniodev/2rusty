use utils::read_file;
use lexer::Scanner;

pub mod utils;
pub mod lexer;

fn main() {
    let path_to_file = "in.c";

    let mut lexer = Scanner {
        source: &read_file(&path_to_file),
        ..Default::default()
    };

    lexer.parse_tokens();
    lexer.print_tokens();
}
