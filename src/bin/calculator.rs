use std::{fs::OpenOptions, io::{self, Write}};


use chrono::Utc;
use thiserror::Error;


// define our custom error so as map errors from the std lib to what we want 

// 
#[derive(Debug, Error)]
enum CalErrors {
    #[error("Could not read input")]
    ReadError,
    #[error("Not a floating number")]
    NumError,
    #[error("Invalid Operator")]
    InvalidOp,
    #[error("Zero division is not allowed")]
    ZeroDivision,
    #[error("error storing logs")]
    StoreError
}

fn main() -> Result<(), CalErrors> {
    cal()
}

fn cal() -> Result<(), CalErrors> {

    // state define the state of the calculator (run or stop)
    let mut state = String::new();
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();


    // closure to store operation which avaluates lazily to avoid code repetition
    let operation = |op: &str, parse_num: f32, parse_num2: f32| -> Result<f32, CalErrors> {
        match op.trim() {
            "+" => Ok(parse_num + parse_num2),
            "-" => Ok(parse_num - parse_num2),
            "*" => Ok(parse_num * parse_num2),
            "/" => {
                if parse_num2 == 0.0 {
                    Err(CalErrors::ZeroDivision)
                } else {
                    Ok(parse_num / parse_num2)
                }
            }
            _ => Err(CalErrors::InvalidOp),
        }
    };

    // RUN the calculator
    loop {
        println!("Enter num as a floating number (e.g., 2.0 for 2):");
        
        io::stdin().read_line(&mut num1).map_err(|_| CalErrors::ReadError)?;
        let parse_num = num1.trim().parse::<f32>().map_err(|_| CalErrors::NumError)?;

        println!("Enter operator (+, -, *, /):");
        
        io::stdin().read_line(&mut operator).map_err(|_| CalErrors::ReadError)?;

        println!("Enter num:");
     
        io::stdin().read_line(&mut num2).map_err(|_| CalErrors::ReadError)?;
        let parse_num2 = num2.trim().parse::<f32>().map_err(|_| CalErrors::NumError)?;

        let result = operation(&operator, parse_num, parse_num2)?;
        println!("Result: {}", result);

        println!("Press 'c' to continue, 'n' for a new session or 'q' to quit :");
       
        io::stdin().read_line(&mut state).map_err(|_| CalErrors::ReadError)?;

        // stop and stores logs
    
        if state.trim() == "q" {
            let log = format!("performed {num1}-{operator}-{num2} at the {:?}", Utc::now());
            OpenOptions::new()
            .append(true)
            .create(true)
            .open("calcultion-logs")
            .map_err(|_| CalErrors::StoreError)?
            .write_all(log.as_bytes()).ok();

            break;
        }
        if state.trim() == "n" {
            num1.clear();
            num2.clear();
            operator.clear();
        }
        if state.trim() == "c" {
        println!("Enter next operator (+, -, *, /):");
        operator.clear();
        io::stdin().read_line(&mut operator).map_err(|_| CalErrors::ReadError)?;

        println!("Enter next number:");
        num1.clear();
        io::stdin().read_line(&mut num1).map_err(|_| CalErrors::ReadError)?;
        let parse_num = num1.trim().parse::<f32>().map_err(|_| CalErrors::NumError)?;

        let new_result = operation(&operator, result, parse_num)?;
        println!("New Result: {}", new_result);
    }
}

    Ok(())
}
