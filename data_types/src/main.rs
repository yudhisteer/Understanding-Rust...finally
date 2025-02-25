#![allow(unused_variables)]

use std::ops::{Range, RangeInclusive};

fn main() {

    /*------------------------------------- Integers -------------------------------------*/
    let int1: i8 = 127;
    let int2: i8 = -128;
    //let int3: i8 = 128; // overflow: i8: [-128, 127]
    //let int4: u8 = 256; // overflow: u8: [0, 255]

    let big_number: u64 = 1_000_000_000_000_000_000; // use underscores to make it easier to read
    println!("{}", big_number); // print as 1000000000000000000

    let value: usize = 1000; // usize is the size of the pointer in the computer
    println!("{}", value);
    println!("Size of usize: {} bytes or {} bits", std::mem::size_of::<usize>(), std::mem::size_of::<usize>() * 8); // 8 bytes on my computer

    let value: isize = 1000; // isize is the size of the pointer in the computer
    println!("{}", value);
    println!("Size of isize: {} bytes or {} bits", std::mem::size_of::<isize>(), std::mem::size_of::<isize>() * 8); // 8 bytes on my computer


    /* ------------------------------------- Floating Point Numbers ------------------------------------- */
    let float1: f32 = 2.53232332323232; //print as 2.5323234
    let float2: f64 = 2.53232332323232323232; //print as 2.5323233232323235

    println!("{}", float1);
    println!("{}", float2);
    
    // print with precision
    println!("{:.2} || {:.3} || {:.4}", 123.4567, 234.45678, 456.78912);


    /* ------------------------------------- Casting with As ------------------------------------- */
    let int1: i8 = 127;
    let float1: f32 = 2.53232332323232;
    println!("{}", float1); // print as 2.5323234

    let int2 = float1 as i64;
    let float2: f64 = float1 as f64;

    println!("{}", int2); // print as 2
    println!("{}", float2); // print as 2.5323233604431152


    /* ------------------------------------- Boolean ------------------------------------- */
    let bool1: bool = true;
    let bool2: bool = false;

    println!("{}", bool1);
    println!("{}", bool2);

    // ! operator
    let bool3: bool = !true;
    println!("{}", bool3); // print as false


    /* ------------------------------------- Array ------------------------------------- */
    // In Rust, we can't directly create a heterogeneous array like [1, 0.9, true, "string"] 
    //because arrays and vectors require all elements to be of the same type.
    let array1: [i32; 5] = [1, 2, 3, 4, 5];
    let names: [&str; 3] = ["John", "Jane", "Doe"];
    let empty_array: [i32; 0] = []; //type annotation is required for empty array

    println!("{:?}", array1);
    println!("{}", array1[0]); // print as 1
    println!("{}", array1[1]); // print as 2
    println!("{}", array1[2]); // print as 3
    println!("{}", array1[3]); // print as 4
    println!("{}", array1[4]); // print as 5
    
    println!("{:?}", names);
    println!("{:?}", empty_array);

    // mutable array
    let mut array2: [i32; 5] = [1, 2, 3, 4, 5];
    array2[0] = 10;
    (&mut array2)[1] = 20; // mutable reference to the array
    println!("{:?}", array2); // print as [10, 20, 3, 4, 5]

    // immutable array
    let array3: [i32; 5] = [3; 5]; // [3, 3, 3, 3, 3]
    println!("{:?}", array3);
    //array3[0] = 10; // error: cannot mutate immutable variable



    /* ------------------------------------- Display Trait ------------------------------------- */
    // Display is for user-facing output formatting, meant to be clean and readable
    let int1: i32 = 1;
    let float1: f64 = 2.5;
    let bool1: bool = true;
    let str1: &str = "Hello";

    // Display output - clean, user-friendly format
    println!("{}", int1);
    println!("{}", float1);
    println!("{}", bool1);
    println!("{}", str1);

    // Debug output - more detailed, development-oriented format
    println!("{:?}", int1);
    println!("{:?}", float1);
    println!("{:?}", bool1);
    println!("{:?}", str1);

    // Pretty debug output with {:#?} for more complex structures
    println!("{:#?}", int1);
    println!("{:#?}", float1);
    println!("{:#?}", bool1);
    println!("{:#?}", str1);



    /* ------------------------------------- Debug Trait ------------------------------------- */
    // Debug is for debugging/development purposes, showing more detailed internal representation
    let array1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array1);
    println!("{array1:?}"); // same as {:?}

    let tuple1: (i32, f64, bool, &str) = (1, 2.5, true, "Hello");
    println!("{:?}", tuple1); // print as (1, 2.5, true, "Hello")
    println!("{:#?}", tuple1); // pretty print
    /*
    print as:
    (
        1,
        2.5,
        true,
        "Hello",
    )
    */

    // #[derive(Debug)] is an attribute that automatically implements the Debug trait
    // for a struct or enum, allowing it to be printed with {:?} format specifier.
    // This saves us from having to manually implement Debug for our custom types.



    /* ------------------------------------- dbg! Macro ------------------------------------- */
    let array1: [i32; 5] = [1, 2, 3, 4, 5];
    dbg!(array1);

    /*
    print as:
    [src\main.rs:142:5] array1 = [
    1,
    2,
    3,
    4,
    5,
    ]
     */


    /* ------------------------------------- Tuple ------------------------------------- */
    let tuple: (i32, f64, bool, &str) = (1, 2.5, true, "Hello");
    println!("{:?}", tuple1); // print as (1, 2.5, true, "Hello")

    // accessing elements
    let int_value = tuple.0;
    let float_value = tuple.1;
    let bool_value = tuple.2;
    let str_value = tuple.3;

    println!("{}", int_value); // print as 1
    println!("{}", float_value); // print as 2.5
    println!("{}", bool_value); // print as true
    println!("{}", str_value); // print as Hello

    // destructuring
    let (int_value, float_value, bool_value, str_value) = tuple;
    println!("{}", int_value); // print as 1
    println!("{}", float_value); // print as 2.5
    println!("{}", bool_value); // print as true
    println!("{}", str_value); // print as Hello



    /* ------------------------------------- Range ------------------------------------- */
    let range1: Range<i32> = 1..5;
    println!("{:#?}", range1); // print as 1..5

    let range2: RangeInclusive<i32> = 1..=5;
    println!("{:#?}", range2); // print as 1..=5

    let letters: RangeInclusive<char> = 'a'..='e';
    println!("{:#?}", letters); // print as a..=e

    for i in range1 {
        println!("{}", i); // print as 1, 2, 3, 4
    }

    for i in range2 {
        println!("{}", i); // print as 1, 2, 3, 4, 5
    }

    for i in letters {
        println!("{}", i); // print as a, b, c, d, e
    }
    
}
