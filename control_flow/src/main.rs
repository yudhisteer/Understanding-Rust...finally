fn even_or_odd(n: i32) {
    // Equivalent to ternary operator
    let result: &str = if n % 2 == 0 { "Even" } else { "Odd" };
    println!("{}", result);
}

fn main() {
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

    //Match statement
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

}