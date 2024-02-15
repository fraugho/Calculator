use std::io;

enum ResultType {
    Failed,
    Success,
}

fn is_operator(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '^' | '(' | ')')
}

fn map_operator(c: char) -> (char, u32) {
    match c {
    '+' => {
        (c, 2)
    }
    '-' => {
        (c, 1)
    }
    '*' => {
        (c, 4)
    }
    '/' => {
       (c, 3)
    }
    '^' => {
        (c, 5)
    }
    '(' => {
        (c, 6)
    }
    ')' => {
        (c, 7)
    }
    _ => (c, 0),
    }
}

fn group_digits(input: &str) -> Vec<i32> {
    let mut nums = Vec::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() {
            current_number.push(c);
        } else if !current_number.is_empty() {
            if let Ok(num) = current_number.parse::<i32>() {
                nums.push(num);
            }
            current_number.clear();
        }
    }

    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<i32>() {
            nums.push(num);
        }
    }

    nums
}

fn tokenize(input: &str, int_stack: &mut Vec<i32>, char_stack: &mut Vec<(char, u32)>) -> ResultType {
    char_stack.extend(input.chars().filter(|&c| is_operator(c)).map(map_operator));
    int_stack.extend(group_digits(input));

    ResultType::Success
}

fn compute(nums: &mut Vec<i32>, operators: &mut Vec<(char, u32)>) -> Option<i32>{
    while !operators.is_empty() {
        let mut highest_precedence = 0;
        let mut index = 0;
        let operator_len = operators.len();

        // Find the operator with the highest precedence
        for (i, &(_, precedence)) in operators.iter().enumerate() {
            if highest_precedence == 6 {
                break;
            }
            if precedence == 7 {
                return  None;
            }
            if precedence > highest_precedence {
                highest_precedence = precedence;
                index = i;
            }
        }
        
        if highest_precedence == 6 {
            let parenthesis_index = index;
            let mut valid_parentheses = false;
            for i in index..operator_len{
                if operators[i].0 == ')' {
                    operators.remove(i);
                    valid_parentheses = true;
                    break;
                }
                operators[i].1 += 10;
            }
            if !valid_parentheses {
                return None;
            }
            operators.remove(parenthesis_index);
        }

        if index >= nums.len() - 1 {
            println!("Invalid expression");
            return None
        }

        // Apply the operation
        let result = match operators[index].0 {
            '+' => nums[index] + nums[index + 1],
            '-' => nums[index] - nums[index + 1],
            '*' => nums[index] * nums[index + 1],
            '/' => {
                if nums[index + 1] == 0 {
                    println!("Division by zero error");
                    return None
                }
                nums[index] / nums[index + 1]
            }
            '^' => nums[index].pow(nums[index + 1] as u32),
            _ => {
                println!("Unsupported operator");
                return None
            }
        };

        // Update the vectors
        nums.remove(index + 1);
        nums[index] = result;
        operators.remove(index);
    }
    let answer = nums[0];
    nums.clear();
    operators.clear();
    Some(answer)
}

fn main() {
    let mut input =  String::new();
    let mut int_stack: Vec<i32> = vec![];
    let mut op_stack: Vec<(char, u32)> =  vec![];

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    input = input.chars()
        .filter(|&c| !c.is_whitespace())
        .collect();
    
    while let ResultType::Success = tokenize(&input, &mut int_stack, &mut op_stack){
        if let Some(result) = compute(&mut int_stack, &mut op_stack){
            println!("The answer is {}", result);
        }
        
        input.clear();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        input = input.chars()
            .filter(|&c| !c.is_whitespace())
            .collect();
        
        if input == "quit" {
            break;
        }
    }
}