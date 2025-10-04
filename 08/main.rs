fn main() {
    let x = 5;

    let x = x + 1; //read shadowing.txt

    {
        let x = x * 2; 
        println!("The value of x in the inner scope is: {x}");
    }// read the inner scope.txt

    println!("The value of x is: {x}"); //read {x} or {},x.txt
}

/*
thats a block commenting. its like c++ i learnt that in javascript.
*/