pub fn run() {
     //Variable can be type annotated.
     let _logical: bool = true;

     let _a_float: f64 = 1.0; // Regular annotation
     let an_integer: i32 = 5i32; // Suffix annotation

     //Or a default will be used.
     let default_float: f64 = 3.0; // 'f64'
    let default_integer: i32 = 7; // 'i32'

    //A type can also be inferred from context.
    let mut inferred_type: i64 = 12; //Type i64 inferred from another line.
    inferred_type = 429496729i64;
    
    // A mutable variable's value can be changed.
    let mut mutable: i32 = 12; //Mutable 'i32'

    //Error! The type of a variable can't be changed.
    //mutable = true;

    // Variable can be overwritten with shadowing.
    let mutable: bool = true;

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}