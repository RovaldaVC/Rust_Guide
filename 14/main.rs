//here we will learn collection types.
// to learn the difference between tup, array and collection types read : tup, array or collection type.txt

fn main() {
    //vec
    let _v : Vec<i32> = Vec::new();
    let _theVec: Vec<i32> = vec![1, 2, 3];

    let mut _the_numbers_vec:Vec<i32> = Vec::new();
    _the_numbers_vec.push(5);
    _the_numbers_vec.push(6);
    _the_numbers_vec.push(7);
    _the_numbers_vec.push(8);
    _the_numbers_vec.push(9);
    println!("{:?}", _v);

    let _V2: Vec<i32> = vec![1, 2, 3, 4, 5];
    let third : &i32 = &_V2[2];
    //println!("the third element of the vector is {}", third);

    let third2 : Option<&i32> = _V2.get(2);
    match third2{
        Some(value)=> println!("the third element of the vector is {}", value),
        None=> println!("There is no third element in the vector"),
    }

    //String
    let s:String = "hello".to_string();
    let s:String = String::from("hello");
    let mut s:String = String::from("hello");
    s.push_str(", world!");
    s.push('!');
    println!("the value of s is {}", s);


    let s1:String = String::from("hello");
    let s2:String = String::from(", world!");
    let s3 = s1 + &s2;
    println!("the value of s3 is {}", s3);
    //s1 doesnt exist anymore because of the ownership transfer.
    //s2 exist because it was refrenced to s3.
    //emphasizing point : if s2 was mut you could only transfer it once! with the &mut part.    
    
    //hashmap
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // insert will get (key, value) like python dict.
}
