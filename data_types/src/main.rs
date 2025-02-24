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
    
    

    
    

    

}
