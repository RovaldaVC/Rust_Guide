fn main() {
    let s1 : String = String::from("RUST");
    let lenght = calculate_lenght(&s1);
    println!("The Lenght of '{}' is {}", s1, lenght);
    let s2 = s1; //that means we transfered the ownership from s1 to s2. after transfering the ownership, s1 is no longer valid.
}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}// read usize.txt