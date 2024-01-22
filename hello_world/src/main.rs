///associates greetings module with this crate
//mod greetings;
//use greetings::*;
extern crate hello_world_lib;
mod how_you_hold_data_for_operations;
use how_you_hold_data_for_operations::primitives::*;
use how_you_hold_data_for_operations::derived::*;

///Optionally load each member of greetings
/*use greetings::default_greeting;
use greetings::spanish;
use greetings::french;*/
///Alternatively, use * to load them all
//use greetings::*;
///Alternatively, load them in one line

// use greetings::{spanish, french, english};

//use greetings::default_greeting;
//use greetings::default_greeting2;
//Or use greetings::*;

fn main() {
    //let mut greeting :&str = "Hello there";
    //greeting = "Hi";
    println!("tireni is sgort!");
    //*println!("{}", english::default_greeting());
    compound::main();
    scalar::run();
    user_defined::run2();
    user_defined::run_circle();
    user_defined::run_triangle();
    functions::run();
    functions::run2();
    functions::run3();
    //functions::run4();
    //println!("{}", default_greeting2())
    //println!("My first greeting is '{}' and my second greeting is '{}'", 
    //default_greeting(), 
    //default_greeting2());

    /*println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", hello_world_lib::greeting_from_lib());*/
}

/*core: primitives - scalar (codes), types, compound types
alloc
macro*/
