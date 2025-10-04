// today we are learning struct in Rust. its something like classes in other languages.also for validation!
fn main() {
    struct User{
        active : bool,
        username: String,
        email: String,
        password: String,
        sign_in_count: u64,
    }

    let mut user1 = User{
        active: true,
        username: String::from("someusername"),
        email: String::from("someemail@gmail.com"),
        password: String::from("somepassword"),
        sign_in_count: 1,
    }; //creating a new user
    user1.email = String::from("anotheremail@gmail.com");//changing the email of the user
    println!("User email is: {}", user1.email);


    //using build_user function to create a new user
    fn build_user(email: String, username: String) -> User{
        User{
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }
    //"-> user" means the value i want to be returned is a struct of type User.
    //Inner User simply means we called the struct User from the outside to fill it out for this part and then return it.
    //when we create the fn with (email and username) it means they will be filled inside the fn for structure automatiucally.

    //using let for creating a new user
    let user2 = User{
        email: String::from("someemail2@gmail.com"),
        ..user1
        //this ..user1 means we want to copy all the values from user1 into user2 except for the email which we will overwrite with the new email.
    }

    // Tuple Structs
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    let black:Color = (0,0,0);
    let white:Color = (255,255,255);

}   