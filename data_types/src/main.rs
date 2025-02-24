#![allow(unused_variables)]

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


    
    
    
    
    
    
    
    

}
