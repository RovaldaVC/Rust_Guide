//first read start with_.txt file.

fn main() {
    let _x: i32 = 5;
    let _y: &i32 = &_x;

    println!("Value of _x : {}", _x);
    println!("Value of _y : {}", _y);


    let mut _b: i32 = 5;
    let _c: &mut i32 = &mut _b;
    *_c += 1;

    println!("Value of _b : {}", _b);
    //when you change the value of _c, _b changes.
    // you can create only one mutable reference to a variable at a time.
    // but for immutable references, you can have multiple references to the same variable.

    let mut account = BackAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    account.check_balance();
    account.withdraw(45.5);
    account.check_balance();
}

struct BackAccount{
    owner: String,
    balance: f64,
}

impl BackAccount{
    fn withdraw(&mut self, amount: f64){
        println!("withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("account owned by {} has balance of {}", self.owner, self.balance);
    }
}

