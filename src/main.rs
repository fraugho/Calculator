use std::io;

enum ResultType {
    Failed,
    Success,
}

fn is_operator(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '^')
}

fn map_operator(c: char) -> (char, u32) {
    match c {
    '+' => {
        (c, 2)
    },
    '-' => {
        (c, 1)
    }
    '*' => {
        (c, 4)
    }
    '/' => {
       (c, 3)
    },
    '^' => {
        (c, 5)
    },
    _ => (c, 0),
    }
}


fn tokenize(input: &str, int_stack: &mut Vec<i32>, char_stack: &mut Vec<(char, u32)>) -> ResultType{
    let mut num = String::new();
    for i in input.chars(){
        if i.is_ascii_digit(){
            num.push(i);
        } else if is_operator(i) {
            if let Ok(value) = num.parse::<i32>() {
                int_stack.push(value);
            } else {
                println!("Invalid number");
                return ResultType::Failed
            }
            char_stack.push(map_operator(i));
            num.clear();
        } else if i.is_alphabetic() {
            println!("Invalid Input");
            return ResultType::Failed
        }
    }
    if !num.is_empty(){
        if let Ok(value) = num.parse::<i32>(){
            int_stack.push(value);
        } else{
            println!("Failed");
            return ResultType::Failed
        }
    }
    if int_stack.len() > char_stack.len() + 1 {
        println!("Too many numbers");
        return ResultType::Failed
    }
    if char_stack.len() > int_stack.len() - 1 {
        println!("Too many Operators");
        return ResultType::Failed
    }
    ResultType::Success
}

fn compute(nums: &mut Vec<i32>, operators: &mut Vec<(char, u32)>) -> Option<i32>{
    while !operators.is_empty() {
        let mut highest_precedence = 0;
        let mut index = 0;

        // Find the operator with the highest precedence
        for (i, &(_, precedence)) in operators.iter().enumerate() {
            if precedence > highest_precedence {
                highest_precedence = precedence;
                index = i;
            }
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

    Some(nums[0])
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