const SOURCE: &str = r#"
fn main() {
    a := 1 + 2
    return a
}
"#;

use x_compiler::{
    fmt::{Format, Formatter},
    Parser,
};

fn main() {
    let mut parser = Parser::new(SOURCE);
    let file = parser.parse().unwrap();
    println!("{}", file.fmt(&mut Formatter::default()));
}
