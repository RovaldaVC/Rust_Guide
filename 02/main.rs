//read the files first, start with integer.

//primitive data types in Rust
//string,integer,float,boolean,char

fn main(){
    let x:i32 = -42;
    let y:u64 = 100;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

    let pi:f64 = 3.14;
    println!("value of pi: {}", pi);

    let is_snowing: bool = true;
    println!("is it snowing? {}", is_snowing);

    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);
}

// chars will only be with ''
// strings will be with ""