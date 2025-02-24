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

    

}
