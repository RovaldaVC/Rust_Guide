//read const and static.txt first.

fn hello_world(){
    println!("Hello, world!");
} //look for hello_wirld() fn main()

fn tell_height(height:u8){
    println!("My height is {} cm", height);
}//look for tell_height() fn main()

fn human_id(name:&str, age:u8, height:f32){
    println!("My name is {}, I am {} years old and my height is {} cm", name, age, height);
}//look for human_id() fn main()

fn add(a:i32, b:i32) -> i32{
    a + b
}//look for add() fn main()

//BMI = weight(kg)/height(m)^2
fn calculate_bmi(weight_kg:f64, height_m:f64) ->f64{
    weight_kg / (height_m * height_m)
}//look for calculate_bmi() fn main()

const _X = {
  let x = 10;
  x + 20
};

//main function is the entry point of the program. its needed.
fn main() {
   hello_world();
   tell_height(183);
   human_id("Arshia", 18, 183.5);
    let x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Result is: {}", x);

    let y = add(4, 6);
    println!("Result is:{}", y);
    
    println!("Value of function 'add' is: {}", add(4,6));

    let weight = 70.0;
    let height = 1.83;
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is {:.2}", bmi)
    //{:.2} means to display the value with 2 decimal places.
}


// underscore: _
//highten: -

//every function must be used. you cant create a function without using it.

//first your functions and in the end main function this is called hoisting. better for performance.

//expression: anything that returns a value.
//statement: anything that does not return a value.
