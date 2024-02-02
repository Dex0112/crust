struct Node {    token: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug, PartialEq)]
enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Number(f64),
    Unrecognized,
    //I am currently unsure of how to detect/handle this
    //Unrecognized,
    //Assignemnt
    //Sin, Cos, Tan, etc
}

fn main() {
    let expression = "32.320+10+5-10";

    let tokens: Vec<Token> = tokenize(expression);

    println!("{}", expression);
}

fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut current_token: String = String::new();

    for c in expression.chars() {
        if !(c.is_numeric() || c == '.') {
            //try parse number
            //return a result maybe??? tokenize enum results??

            let token: Token = Token::Number(current_token.parse::<f64>().unwrap());

            tokens.push(token);
        }

        match c {
            '+' => tokens.push(Token::Add),
            //This will cause problems with negative numbers
            '-' => tokens.push(Token::Subtract),
            _ => current_token.push(c),
        }
    }

    return tokens;
}

//Generate tree
//1. Find the operator
//2. Check precedence
//3. Check next operators precedence
//4a. Equal precedence = set next number to right side expression
//4b. Higher precedence = add to stack and repeat step 3
//4c. Lower precedence = Roll back to previous operator
//4d. EOL = add to stack

fn generate_parsing_tree(tokens: Vec<Token>) -> Option<Box<Node>> {
    todo!("Impl");
}

fn evaluate(node: Option<Box<Node>>) -> f64 {
    todo!("Impl");
}

#[cfg(test)]
mod test {
    use std::num::NonZeroI8;

    use super::{Node, Token};

    #[test]
    fn tokenize_all_operators() {
        let tokens: Vec<Token> = vec![
            Token::Number(34.0),
            Token::Divide,
            Token::Number(5.0),
            Token::Add,
            Token::Number(12.0),
            Token::Multiply,
            Token::Number(3.0),
            Token::Divide,
            Token::Number(2.0),
            Token::Subtract,
            Token::Number(6.0),
            Token::Add,
            Token::Number(33.0),
            Token::Divide,
            Token::Number(3.0),
            Token::Add,
            Token::Number(13.0),
        ];

        let expression: &str = "34/5+12*3/2-6+33/3+13";

        assert_eq!(tokens, super::tokenize(expression));
    }

    #[test]
    fn tokenize_add() {
        let tokens: Vec<Token> = vec![
            Token::Number(33.0),
            Token::Add,
            Token::Number(3.0),
            Token::Add,
            Token::Number(354.0),
            Token::Add,
            Token::Number(5.0),
            Token::Add,
            Token::Number(9.0),
        ];

        let expression: &str = &"33+3+354+5+9";

        assert_eq!(tokens, super::tokenize(expression));
    }

    #[test]
    fn tokenize_subtract() {
        let tokens: Vec<Token> = vec![
            Token::Number(795.0),
            Token::Subtract,
            Token::Number(3.0),
            Token::Subtract,
            Token::Number(17.0),
            Token::Subtract,
            Token::Number(42.0),
        ];

        let expression: &str = "795-3-17-42";

        assert_eq!(tokens, super::tokenize(expression));
    }

    #[test]
    fn tokenize_multiply() {
        let tokens: Vec<Token> = vec![
            Token::Number(15.0),
            Token::Multiply,
            Token::Number(14.0),
            Token::Multiply,
            Token::Number(27.0),
            Token::Multiply,
            Token::Number(25.0),
        ];

        let expression: &str = "15*14*27*25";

        assert_eq!(tokens, super::tokenize(expression));
    }

    #[test]
    fn tokenize_divide() {
        let tokens: Vec<Token> = vec![
            Token::Number(1459.0),
            Token::Divide,
            Token::Number(4.0),
            Token::Divide,
            Token::Number(7.0),
        ];

        let expression: &str = "1459/4/7";

        assert_eq!(tokens, super::tokenize(expression));
    }

    #[test]
    fn evaluate_all_operators() {
        let head = Some(Box::new(Node {
            left: Some(Box::new(Node {
                left: Some(Box::new(Node {
                    left: Some(Box::new(Node {
                        left: Some(Box::new(Node {
                            left: Some(Box::new(Node {
                                left: None,
                                right: None,
                                token: Token::Number(34.0),
                            })),
                            right: Some(Box::new(Node {
                                left: None,
                                right: None,
                                token: Token::Number(5.0),
                            })),
                            token: Token::Divide,
                        })),
                        right: Some(Box::new(Node {
                            left: Some(Box::new(Node {
                                left: Some(Box::new(Node {
                                    left: None,
                                    right: None,
                                    token: Token::Number(12.0),
                                })),
                                right: Some(Box::new(Node {
                                    left: None,
                                    right: None,
                                    token: Token::Number(3.0),
                                })),
                                token: Token::Multiply,
                            })),
                            right: Some(Box::new(Node {
                                left: None,
                                right: None,
                                token: Token::Number(2.0),
                            })),
                            token: Token::Divide,
                        })),
                        token: Token::Add,
                    })),
                    right: Some(Box::new(Node {
                        left: None,
                        right: None,
                        token: Token::Number(6.0),
                    })),
                    token: Token::Subtract,
                })),
                right: Some(Box::new(Node {
                    left: Some(Box::new(Node {
                        left: None,
                        right: None,
                        token: Token::Number(33.0),
                    })),
                    right: Some(Box::new(Node {
                        left: None,
                        right: None,
                        token: Token::Number(3.0),
                    })),
                    token: Token::Divide,
                })),
                token: Token::Add,
            })),
            right: Some(Box::new(Node {
                left: None,
                right: None,
                token: Token::Number(13.0),
            })),
            token: Token::Add,
        }));

        let result: f64 = 40.8;

        assert_eq!(result, super::evaluate(head));
    }

    #[test]
    fn evaluate_addition() {
        //15+21+247

        let head = Some(Box::new(Node {
            left: Some(Box::new(Node {
                left: Some(Box::new(Node {
                    left: Some(Box::new(Node {
                        left: None,
                        right: None,
                        token: Token::Number(15.0),
                    })),
                    right: Some(Box::new(Node {
                        left: None,
                        right: None,
                        token: Token::Number(21.0),
                    })),
                    token: Token::Add,
                })),
                right: None,
                token: Token::Unrecognized,
            })),
            right: Some(Box::new(Node {
                left: None,
                right: None,
                token: Token::Number(247.0),
            })),
            token: Token::Add,
        }));

        let result: f64 = 283.0;

        assert_eq!(result, super::evaluate(head));
    }

    #[test]
    fn evaluate_subtraction() {
        // 287-73-8

        let head = Some(Box::new(Node {
            left: Some(Box::new(Node {
                left: Some(Box::new(Node {
                    left: None,
                    right: None,
                    token: Token::Number(287.0),
                })),
                right: Some(Box::new(Node {
                    left: None,
                    right: None,
                    token: Token::Number(73.0),
                })),
                token: Token::Subtract,
            })),
            right: Some(Box::new(Node {
                left: None,
                right: None,
                token: Token::Number(8.0),
            })),
            token: Token::Subtract,
        }));

        let result: f64 = 206.0;

        assert_eq!(result, super::evaluate(head));
    }

    #[test]
    fn evaluate_multiplication() {
        //47*52*100

        let head = Some(Box::new(Node {
            left: Some(Box::new(Node {
                left: Some(Box::new(Node {
                    left: None,
                    right: None,
                    token: Token::Number(47.0),
                })),
                right: Some(Box::new(Node {
                    left: None,
                    right: None,
                    token: Token::Number(52.0),
                })),
                token: Token::Multiply,
            })),
            right: Some(Box::new(Node {
                left: None,
                right: None,
                token: Token::Number(100.0),
            })),
            token: Token::Multiply,
        }));

        let result: f64 = 244400.0;

        assert_eq!(result, super::evaluate(head));
    }

    #[test]
    fn evaluate_division() {
        let head = Some(Box::new(Node {
            left: Some(Box::new(Node {
                left: Some(Box::new(Node {
                    left: None,
                    right: None,
                    token: Token::Number(47.0),
                })),
                right: Some(Box::new(Node {
                    left: None,
                    right: None,
                    token: Token::Number(2.0),
                })),
                token: Token::Divide,
            })),
            right: Some(Box::new(Node {
                left: None,
                right: None,
                token: Token::Number(3.0),
            })),
            token: Token::Divide,
        }));


        let result: f64 = 58.75; 

        assert_eq!(result, super::evaluate(head));
    }
}
