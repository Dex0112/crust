use crate::Lexer;
use crate::Token;

struct Node {
    token: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

//I don't really know how to implement this but I want to try to get a working iplementation by
//myself
//I could try to make the implementation as just a function but then work in some pesky variables
//into fields like stacks

struct Ast {
    head: Option<Box<Node>>,
}

impl Ast {
    fn new(expression: String) {
        println!("New Abstract Syntax Tree");

        //Generate tree

        let mut stack: Vec<Token> = Vec::new();

        //Parenthesis could work by adding like a base_precedence and it would add to
        //current_precedence
        let mut current_precedence: Option<i32> = None;

        let mut lexer = Lexer::new(expression);

        loop {
            let token = lexer.next_token();

            if token == Token::Eol {
                break;
            }

            let precedence: Option<i32> = match token {
                Token::Add => Some(1),
                Token::Subtract => Some(1),
                Token::Multiply => Some(2),
                Token::Divide => Some(2),
                _ => None,
            };

            if precedence.is_some() {
                if precedence > current_precedence {
                    current_precedence = precedence;
                }
            }

            stack.push(token);
        }
    }
}

// Generate tree
// 1. Find the operator
// 2. Check precedence
// 3. Check next operators precedence
// 4a. Equal precedence = set next number to right side expression
// 4b. Higher precedence = add to stack and repeat step 3
// 4c. Lower precedence = Roll back to previous operator
// 4d. EOL = add to stack
// Stacks
