struct Node {    
    token: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
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
