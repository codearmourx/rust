use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let _x = &args[1];
    let _y = &args[2];
    let _op = &args[3];
    let _result = 0;

    println!("{} {} {}", _x, _op, _y);
    let _int_x: f64 = _x.parse().unwrap();
    let _int_y: f64 = _y.parse().unwrap();

    match _op.as_str() {
        "+" => println!("{}", _int_x + _int_y),
        "-" => println!("{}", _int_x - _int_y),
        "*" => println!("{}", _int_x * _int_y),
        "/" => {
            if _int_y != 0.0 {
                println!("{}", _int_x / _int_y);
            } else {
                println!("Division by zero error");
            }
        }
        _ => println!("Invalid operator"),
    }

   

    // println!("The result is: {}", result);
}

    
