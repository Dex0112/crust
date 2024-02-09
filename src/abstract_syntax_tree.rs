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
