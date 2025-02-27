// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values

fn main() {

    let n = 5;
    let square_of_n = square(n);
    println!("The square of {} is {}", n, square_of_n);

    let n = 5;
    let plus_one_of_n = plus_one(n);
    println!("The plus one of {} is {}", n, plus_one_of_n);

    // Block in function
    let number = 6;
    
    let calculated_number = {
        let value = 3;
        let number = value * 2;
        number + 1 // no semicolon, so it returns the value
    };
    println!("The number is {}", number); // number = 6
    //println!("The value is {}", value); // not found in this scope
    println!("The calculated number is {}", calculated_number); // calculated_number = 13
}

// Explicit return
fn square(n: i32) -> i32 {
    return n * n; 
}

// Implicit return
fn plus_one(n: i32) -> i32 {
    n + 1 // no semicolon, so it returns the value
}





