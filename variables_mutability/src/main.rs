#![allow(unused_variables)] // allow unused variables in the entire file

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#storing-values-with-variables
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#printing-values-with-println-placeholders
// https://doc.rust-lang.org/rust-by-example/hello/print.html
// https://doc.rust-lang.org/std/macro.println.html
// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability
// https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html
// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
// https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html
// https://doc.rust-lang.org/rust-by-example/types/alias.html


// Constants can be declared in any scope, including the global scope
const RATE: f64 = 0.05;


// allow unused variables in the main function
#[allow(unused_variables)]
fn main() {

    /*------------------------------------- Variables -------------------------------------*/
    // We declare some variables with let keyword
    let dogs: i32 = 5;
    let some_float_number: f64 = 0.05;
    let sum_float: f64 = 2.0 + 3.0; 
    let _unused_variable: i32 = 3;   // starts with underscore

    let dogs: i32 = 45; // variable can still be changed
    
    println!("I have {} dogs", dogs);
    println!("I have {} as float number and {} as sum float", some_float_number, sum_float);
    println!("I have {0} dogs and {1} as float number and again {0} dogs.", dogs, some_float_number);

    // cannot assign twice to immutable variable
    // dogs = 46; 



    /*------------------------------------- Mutability -------------------------------------*/
    // mutated variable
    let mut cats: i32 = 10; 
    println!("I have {} cats", cats);
    // cats = 11.0; // mismatch type - we cannot change the type
    cats = 11; // mutated variable
    println!("I now have {} cats", cats);

    // another example with mutability
    let mut mut_value: i32 = 10;
    let unmut_value: i32 = 100;

    mut_value += 1;
    println!("mut_value is {}", mut_value);

    // cannot mutate immutable variable `unmut_value`
    // unmut_value += 1; // error, reassignment of immutable variable



    /*------------------------------------- Shadowing -------------------------------------*/
    let x: i32 = 10;
    let x: i32 = x + 1;
    let x: i32 = x * 2;
    println!("x is {}", x);

    let value: i32 = 10;
    println!("value is {}", value);

    let value: &str = "Hello"; // change type of value
    println!("value is {}", value);

    let value: bool = true; // change type of value
    println!("value is {}", value);



    /*------------------------------------- Scope -------------------------------------*/
    let rice: i32 = 100;
    println!("rice is {}", rice);

    let butter: i32 = 50;

    // inner scope
    {
        let bread: i32 = 200;    
        println!("bread is {}", bread);

        let butter: i32 = 500; // shadowing
        println!("butter is {}", butter);
    }

    // value is still 50 because it is not shadowed outside the inner scope
    println!("butter is {}", butter); // 50

    // we cannot access bread because it is not defined in this scope
    // println!("bread is {}", bread); // error, bread is not defined in this scope



    /*------------------------------------- Constants -------------------------------------*/
    const POINTS: i32 = 6; // Constant should be uppercase and data type should be specified
    println!("Points are {}", POINTS);

    // cannot modify a constant
    // POINTS = 10; // error, cannot assign to constant

    // Constants can be declared in any scope, including the global scope
    println!("Rate is {}", RATE);



    /*------------------------------------- Type Aliases -------------------------------------*/
    type Point = (i32, i32); // Types must have UpperCamelCase
    let origin: Point = (0, 0);
    println!("Origin is {:?}", origin);

    type UserId = i64;
    let user_id: UserId = 123;
    println!("User ID is {}", user_id);



    /*------------------------------------- Compiler Directives -------------------------------------*/
    #[allow(unused_variables)]
    let unused_variable: i32 = 3; // no warning

    // have to be added for each unused variable
    #[allow(unused_variables)]
    let another_unused_variable: i32 = 4;

    // no warning because it is allowed in the main function
    let yet_another_unused_variable: i32 = 5; 
}
