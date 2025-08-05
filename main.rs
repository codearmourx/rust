use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let x : &str = args[1];
    let y : &str = args[2];
    let result :i32;

    if x > y {
        result = x - y; 


    } else if x < y {
        result = y - x;
    } else {
        result = 0;
    }   
    println!("{} is the absolute value", result);
}

