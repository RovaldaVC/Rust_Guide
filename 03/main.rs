//read the txt files first start with &txt.
fn main(){
    let tup: (i32, f64, char, String) = (500, 6.4, 'a', "hello world".to_string());
    println!("the tuple is {:?}", tup);
    let (x, y, z, q) = tup;
    println!("The value of y is: {}", y); // prints 6.4

    let my_mix_tuple = ("kratos", 23, true, [1,2,3,4,5]);
    println!("my mix tuple {:?}", my_mix_tuple);

    let array:[i32; 3] = [1, 2, 3];
    println!("{:?}", array);  // prints [1, 2, 3]
    println!("{}", array[1]); //prints 2 

    let fruit:[&str; 3] = ["banana", "apple", "orange"];
    println! ("{:?}", fruit); // prints ["banana", "apple", "orange"]

    let number_slice:&[i32] = &[1,2,3,4,5];
    println!("number slice {:?}", number_slice);

    let animal_slice:&[&str] = &["lion", "elephant", "crocodile"];
    println!("animal slice {:?}", animal_slice);

    let book_slice:&[&String] = &[&"IT".to_string(), &"harry potter".to_string(), &"ZEN".to_string()];
    println!("book slice {:?}", book_slice);

    let mut stone_cold:String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    let string:String = String::from("Hello, World!");
    let slice:&str = &string;
        println!("Slice Value: {}", slice);
    let slice:&str = &string[0..5];
    println!("Slice Value: {}", slice);
} 

//refrences cant be used out of their function.

//rust language's valueables are by default immutable

//we use : to define the type of the tuple and the variables to assign the values to the tuple.
// we use ; to say how many i32 we want. so when we say (i32; 3) we are saying we want an array of 3 i32.
// the whole array or tuple needed to print should use the format {:?}. else you can use {}.
// String is with CAPITAL S 