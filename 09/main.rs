#![allow(warnings)] //read #![].txt
//read elif.txt first.

fn main() {
    let age: u8 = 18;
    if age >= 18 {
        println!("You can drive a car!");
    } 
    else{
        println!("You cant drive a car!");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("The number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("The number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("the number is divisible by 2");
    }
    else{
        println!("The number is not divisible by 2, 3 or 4");
    }


    let condition = true;
    let number = if condition { 5 } else { 6 };//read statement.txt
    println!("The value of number is: {number}");
}
/* 
    for those who dont know, % means when we divide a number by another number, it gives us the remainder.
    if the remainder is 0, then the number is divisible by the divisor.
*/
