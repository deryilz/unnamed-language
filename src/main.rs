pub mod compiler;

use compiler::lexer::Lexer;

fn main() {
    let string = "
        // my comment is here btw
        id = [T](a: T) => T: (a)
        say_hi = () => print(3)
    ";
    let mut lexer = Lexer::new(string);

    for _ in 1..50 {
        println!("{:?}", lexer.next());
    }
}
