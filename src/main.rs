use std::io::{self, Write};

#[derive(Debug)]
#[derive(PartialEq)]
enum Token {
    Number(f64),
    Operator(Operator),
    Parentheses(Parentheses),
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Parentheses {
    Open,
    Close,
}


fn main() {

    // Welcome message
    println!("Welcome to the calculator app!\nEnter 'q' to exit.");
    println!("");

    // Application main loop
    loop {
        // Take input
        let mut input = String::new();
        print!("(calc) ");
        io::stdout().flush().expect("Flush failed");
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // Check input for exit
        if input == "q\n" {
            break;
        }

        // Pass input to lexer
        let infix_tokens = match lexer(input) {
            Some(token_vector) => token_vector,
            None => continue,
        };

        let result = parser(infix_tokens);
        println!("{result}");
    }
}

// The Lexer job is to analyse the input string and produce an infix token 
// vector if all the input string is valid. The lexer will catch any invalid
// input that cannot be change into a token.
// Completed
fn lexer(input: String) -> Option<Vec<Token>> {
   
    let mut token_vector = Vec::new();
    let mut number_string = String::new();

    // Parsing through the String input
    for character in input.chars() {
        if character.is_ascii_digit() || character == '.' {
            number_string.push(character);
        } else {
            if !number_string.is_empty() {
                match number_string.parse::<f64>() {
                        Ok(num) => token_vector.push(Token::Number(num)),
                        Err(_) => {println!("Invalid number") ;return None},
                }
                number_string.clear();
            }
            match character {
                '+' => token_vector.push(Token::Operator(Operator::Add)),
                '-' => token_vector.push(Token::Operator(Operator::Subtract)),
                '*' => token_vector.push(Token::Operator(Operator::Multiply)),
                '/' => token_vector.push(Token::Operator(Operator::Divide)),
                '(' => token_vector.push(Token::Parentheses(Parentheses::Open)),
                ')' => token_vector.push(Token::Parentheses(Parentheses::Close)),
                ' ' => (),
                '\n' => (),
                _ => {println!("Input contains invalid character"); return None},
                               
            }
        }
        
    }

    if token_vector.len() == 0 {
        return None;
    } else {
        return Some(token_vector);
    }
}


fn parser(infix_tokens: Vec<Token>) -> f64{

    let mut operand_stack: Vec<f64> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();
    
    for token in infix_tokens {
        match token {
            Token::Number(num) => operand_stack.push(num),
            Token::Parentheses(Parentheses::Open) => operator_stack.push(token),
            Token::Parentheses(Parentheses::Close) => {
                while operator_stack[operator_stack.len()-1] != Token::Parentheses(Parentheses::Open) {
                    let _ = process(&mut operator_stack, &mut operand_stack);                    
                }
                operator_stack.pop();
            },
            Token::Operator(_) => {
                if operator_stack.is_empty() {
                    operator_stack.push(token);
                } else {
                    while precedence(&operator_stack[operator_stack.len()-1]) >= precedence(&token) {
                        let _ = process(&mut operator_stack, &mut operand_stack);
                        if operator_stack.is_empty() {
                            break;
                        }
                    }
                    operator_stack.push(token);
                }
            },
        }
    }
    for _ in 0..operator_stack.len() {
        process(&mut operator_stack, &mut operand_stack);
    }
    operand_stack[0]
}

fn precedence(op: &Token) -> u8 {
    match op {
        Token::Operator(Operator::Add) => 1,
        Token::Operator(Operator::Subtract) => 1,
        Token::Operator(Operator::Multiply) => 2,
        Token::Operator(Operator::Divide) => 2,
        _ => 0,
    }
    
}


fn process(operator_stack: &mut Vec<Token>, operand_stack: &mut Vec<f64>) -> bool {
    let y = match operand_stack.pop() {
        Some(num) => num,
        None => return false,
    };
    let x = match operand_stack.pop() {
        Some(num) => num,
        None => return false,
    };
    let operator = match operator_stack.pop() {
        Some(token) => token,
        None => return false,
    };
    match operator {
        Token::Operator(Operator::Add) => operand_stack.push(x+y),
        Token::Operator(Operator::Subtract) => operand_stack.push(x-y),
        Token::Operator(Operator::Multiply) => operand_stack.push(x*y),
        Token::Operator(Operator::Divide) => operand_stack.push(x/y),
        _ => (),
    }
    return true;
        
    
}
