//to learn match first read match.txt
//use wrap (alt+z) for reading.

//read OptionT explained.txt
fn divideOption(numerator: f64, denominator: f64)->Option<f64> {
    if denominator == 0.0 {
        None
    }
    else{
        Some(numerator/denominator)
    }
} //divide function with option<f64>

//read Result T,E or OptionT.txt
fn divideResult(numerator: f64, denominator: f64)-> Result<f64, String> {
    if denominator == 0.0{
        Err("Cannot divide by zero".to_string())
    }
    else{
        Ok(numerator/denominator)
    }
}// in result, <Ok, Err> so if Ok -> f64 and Err -> String.

fn main() {
    let result = divideOption(10.0, 0.0);
    match result{
        Some(value) => println!("Result is {}", value),
        None => println!("Cannot divide by zero"),
    }


    let result = divideResult(100.23, 73.98);
    match result{
        Ok(result) => println!("Result is {}", result),
        Err(error) => println!("Error: {}", error),
    }// here match actually means if the returned value of result is Some(value) then execute the first block of code and print "Result is {}" with value, otherwise execute the second block of code and print "Cannot divide by zero".
}

// -> for value return and => for match block.