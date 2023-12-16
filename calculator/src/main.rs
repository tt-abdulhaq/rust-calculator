use std::env;


fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() != 4{
        println!("Usage: {} <number> <operation> <number>", args[0]);
        return;
    }

    let num1:f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Invalid number");
            return;
        }
    };
    let num2:f64 = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Invalid number");
            return;
        }
    };

    let result= match args[2].as_str() {
        "+" => num1+num2,
        "-" => num1+num2,
        "*" => num1+num2,
        "/" => {
            if num2 != 0.0 {
                num1/num2
            }else {
                println!("ZeroDevisionException");
                return;
            }
        } _ =>{
            println!("Please enter one of +, -, *, /");
            return;
        }
        
    };
    println!("Result: {}", result);

}
