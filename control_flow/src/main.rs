fn even_or_odd(n: i32) {
    // Equivalent to ternary operator
    let result: &str = if n % 2 == 0 { "Even" } else { "Odd" };
    println!("{}", result);
}

fn main() {

    // --------------------------------------------------------------
   // If else statements
   // --------------------------------------------------------------

    let day: &str = "Monday";

    // Dependent if statements
    if day == "Monday" {
        println!("Oh it sucks!")
    } else if day == "Tuesday" {
        println!("It's okay")
    } else if day == "Friday" {
        println!("Yahh it's Friday!!!")
    } else {
        println!("Double yahhh it's weekend!!!")
    }

    // Independent if statements
    let is_it_raining: bool = true;
    let is_it_cold: bool = true;

    if is_it_raining {
        println!("It's raining")
    }

    if is_it_cold {
        println!("It's cold")
    }

    // Function call
    even_or_odd(2);
    even_or_odd(3);



    // --------------------------------------------------------------
    // Match statement
    // --------------------------------------------------------------
    let evaluation: bool = true;

    match evaluation {
        true => {
            println!("Evaluation is true")
        },
        false => {
            println!("Evaluation is false")
        },
    }

    //Match statement with value
    // every match arms must return the same type
    let value:i32 = match evaluation {
        true => 20,
        false => 10,
    };

    println!("Value: {}", value);


    // Note: match will look for the first arm that matches the value and will not check the rest
    let season: &str = "Autumn";

    match season {
        "Spring" => println!("It's spring"),
        "Summer" => println!("It's summer"),
        "Autumn" => println!("It's autumn"),
        "Winter" => println!("It's winter"),
        _ => println!("Not a valid season"), // _ is a catch all same as else
    };


    // match with pipe
    let number: i32 = 10;

    match number {
        1 | 3 | 5 | 7 | 9 => println!("It's an odd number"),
        2 | 4 | 6 | 8 | 10 => println!("It's an even number"),
        _ => println!("It's not a number"),
    };

    // match with assignment
    let number: i32 = 10;

    match number {
        n if n % 2 == 0 => println!("It's an even number"),
        n if n % 2 != 0 => println!("It's an odd number"),
        _ => unreachable!() // this is line will never be reached
    };

    // match with no assignment
    let number: i32 = 10;

    match number{
        _ if number % 2 == 0 => println!("{number} is an even number"),
        _ if number % 2 != 0 => println!("{number} is an odd number"),
        _ => unreachable!() // this is line will never be reached
   }



   // --------------------------------------------------------------
   // loop
   // --------------------------------------------------------------    
   let mut seconds: i32 = 10;

   loop
   {

    if seconds == 0 {
        break; // break the loop
    }

    println!("{} seconds left", seconds);
    seconds -= 1;

   }

   // loop with continue
   let mut seconds: i32 = 21;

   loop 
   {
    if seconds <= 0 {
        println!("Blast off!");
        break;
    }

    // option 1:
    if seconds % 2 == 0 {
        println!("{} seconds (even number), skipping 3 seconds", seconds);
        seconds -= 3;
        continue; // skip the rest of the loop and go to the next iteration
    }
    
    println!("{} seconds left", seconds);
    seconds -= 1;

    // option 2: same as above but with if else
    // if seconds % 2 == 0 {
    //     println!("{} seconds (even number), skipping 3 seconds", seconds);
    //     seconds -= 3;
    // }
    // else 
    // {
    //     println!("{} seconds left", seconds);
    //     seconds -= 1;
    // }

   }



   // --------------------------------------------------------------
   // While loop
   // --------------------------------------------------------------

   let mut seconds: i32 = 21;

   while seconds > 0 {

    // Do not need this as while loop will break when seconds <= 0
    // if seconds <= 0 {
    //     println!("Blast off!");
    //     break;
    // }

    if seconds % 2 == 0 {
        println!("{} seconds (even number), skipping 3 seconds", seconds);
        seconds -= 3;
        continue; // skip the rest of the loop and go to the next iteration
    }
    
    println!("{} seconds left", seconds);
    seconds -= 1;

   }

   println!("Blast off!");


   // --------------------------------------------------------------
   // Recursion
   // --------------------------------------------------------------

   fn count_down(n: i32) {
    if n <= 0 {
        println!("Blast off!");
    }
    else {
        println!("{} seconds left", n);
        count_down(n - 1);
    }

    //break down the function call with return
    // if n <= 0 {
    //     println!("Blast off!");
    //     return;
    // }
    // println!("{} seconds left", n);
    // count_down(n - 1);
   }

   // Recursion with a function call
   count_down(10);



}