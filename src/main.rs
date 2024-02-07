use lexer::{Lexer, Token};

mod lexer;

fn main() {
    //let expression = String::from("32 + 10 + 5 - 0");
    let expression = String::from("32+10+5-0");

    let mut lexer = Lexer::new(expression.clone());

    println!("{}", expression);

    loop {
        let token = lexer.next_token();

        if token == Token::Eol {
            break;
        }

        print!("{}", token);
    }

    println!("\nGood bye");
}
