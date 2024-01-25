struct Node {
    token: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

enum Token {
    Add,
    Subtract,
    Number(f64),
    //I am currently unsure of how to detect/handle this
    //Unrecognized,
    //Assignemnt
    //Sin, Cos, Tan, etc
}

fn main() {
    let result = evaluate(parse_expression(&"10+5+-7"));

    println!("{}", result);
}

fn parse_expression(expression: &str) -> Option<Box<Node>> {
    let is_num = expression.trim().parse::<f64>();

    if is_num.is_ok() {
        return Some(Box::new(Node {
            token: Token::Number(is_num.unwrap()),
            left: None,
            right: None,
        }));
    }

    for (i, c) in expression.char_indices() {
        let token: Token = match c {
            '+' => Token::Add,
            '-' => Token::Subtract,
            _ => continue,
        };

        //left = parse_expression(str left of token)
        let left = parse_expression(&expression[..i].trim());
        //right = parse_expression(str right of token)
        let right = parse_expression(&expression[i + 1..].trim());

        return Some(Box::new(Node { token, left, right }));
    }

    None
}

fn evaluate(node: Option<Box<Node>>) -> f64 {
    match node {
        Some(outer_node) => {
            let inner_node = *outer_node;

            let left_value = evaluate(inner_node.left);
            let right_value = evaluate(inner_node.right);

            match inner_node.token {
                Token::Add => return left_value + right_value,
                Token::Subtract => return left_value - right_value,
                Token::Number(num) => return num,
            }

        },

        None => return 0.0,
    }
}
