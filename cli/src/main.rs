const SOURCE: &str = r#"
fn main() {
    a := 1 + 2
    print(a)
}
"#;

use x_compiler::{Lexer, Token};

fn main() {
    let mut lexer = Lexer::new(SOURCE);

    while lexer.next().unwrap() != Token::EOF {
        println!("{}", lexer.token())
    }
}
