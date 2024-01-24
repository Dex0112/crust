struct Node<'a> {
    value: Token,
    left: Option<&'a Node<'a>>,
    right: Option<&'a Node<'a>>,
}

enum Token {
    Add,
    Number(i64),
    Unrecognized,
    //Assignemnt
    //Sin, Cos, Tan, etc
}

fn main() {}

fn parse_expression(expression: &mut str) -> Node {
    for c in expression.chars() {
        let token: Token = match c {
            '+' => Token::Add,
            _ => {
                //Is it a number?
                Token::Unrecognized
            },
        };
    }

    return Node {
        value: Token::Add,
        left: None,
        right: None,
    };
}

//fn evaluate 
