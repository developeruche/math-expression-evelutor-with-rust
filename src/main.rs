use std::vec;

// ======================
// STACK RELATED FUNCTION
// ======================

fn new_stack(maxsize: usize) -> Vec<String> {
    let vec = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    let popped_val = stack.pop();
    popped_val
}

fn push(stack: &mut Vec<String>, item: String, maxsize: usize) {
    if stack.len() == maxsize {
    } else {
        stack.push(item);
    }
}

fn size(stack: &Vec<String>) -> usize {
    stack.len()
}

// =======================

fn operate(op1: String, op2: String, operator: String) -> f32 {
    let op1 = op1.parse::<f32>().unwrap();
    let op2 = op2.parse::<f32>().unwrap();

    let result = match operator.as_str() {
        "+" => op1 + op2,
        "-" => op1 - op2,
        "*" => op1 * op2,
        "/" => op1 / op2,
        "^" => op1.powf(op2),
        _ => 0.0,
    };

    result
}

fn priority(x: &String) -> u8 {
    if(x == "+") | (x == "-") {
        1
    } else if (x == "*") | (x == "/") {
        2
    } else if "^" == x {
        3
    } else {
        0
    }
}


fn get_symbols(input_expression: String) -> Vec<String> {
    let mut tokenized_input = Vec::new();
    let input_chars: Vec<char> = input_expression.chars().collect();
    let mut temp: Vec<char> = Vec::new();

    for i in input_chars {
        if i != '+' && i != '-' && i != '/' && i != '*' && i != '^' && i != '(' && i != ')' {
            temp.push(i);
            continue;
        } else {
            if temp.len() == 0 {
                tokenized_input.push(i.to_string());
            } else {
                tokenized_input.push(temp.into_iter().collect());
                tokenized_input.push(i.to_string());
                temp = vec![];
            }
        }
    }

    if temp.len() != 0 {
        tokenized_input.push(temp.into_iter().collect());
    }

    println!("{:?}", tokenized_input);
    tokenized_input
}

fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    let size_expression = input.len();
    let mut stack = new_stack(size_expression);
    let mut postfix = Vec::new();

    for i in input {
        match i.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
                if size(&stack) == 0 {
                    push(&mut stack, i, size_expression);
                } else {
                    if priority(&i) > priority(stack.last().unwrap()) {
                        push(&mut stack, i, size_expression);
                    } else {
                        while priority(&i) <= priority(stack.last().unwrap()){
                            postfix.push(pop(&mut stack).unwrap());
                            if stack.last() == None {
                                break;
                            }
                        }

                        push(&mut stack, i, size_expression);
                    }
                }
            }
            "(" => push(&mut stack, i, size_expression),
            ")" => {
                while stack.last().unwrap() != "(" {
                    postfix.push(pop(&mut stack).unwrap());
                    println!("COUNT");
                }

                pop(&mut stack).unwrap();
            }
            _ => postfix.push(i),
        }
    }

    while size(&stack) != 0 {
        postfix.push(pop(&mut stack).unwrap());
    }

    println!("{:?}", postfix);

    postfix
}


fn postfix_evaluator(postfix: Vec<String>) -> f32 {
    let size_expression = postfix.len();
    let mut result_stack: Vec<String> = new_stack(size_expression);

    for i in postfix {
        match i.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
                let oper = i;
                let(op2, op1) = (pop(&mut result_stack).unwrap(), pop(&mut result_stack).unwrap());
                let result = operate(op1, op2, oper);

                push(&mut result_stack, result.to_string(), size_expression);
            }
            _ => {
                push(&mut result_stack, i.to_string(), size_expression);
            }
        }
    }

    pop(&mut result_stack).unwrap().parse::<f32>().unwrap()
}

fn main() {
    let input_expression = String::from("(3+5)(6+9)");
    
    println!("Incoming expression {:?}", input_expression);
    let input_expr_tokenized = get_symbols(input_expression);

    let postfix = infix_to_postfix(input_expr_tokenized);
    let result = postfix_evaluator(postfix);

    println!("RESULT: {}", result);
}
