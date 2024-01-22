use std::mem;

    // Tuples can be used as function arguements and as return values.
    fn reverse (pair: (i32, bool)) -> (bool, i32) {
        // 'let' can be used to bind the memebrs of a tuple to variables.
        let (int_param, bool_param) = pair;

        (bool_param, int_param)
    }

        //The following struct is for the activity.
        #[derive(Debug)]
        struct Matrix(f32, f32, f32, f32);

// This function borrows a slice.
fn analyze_slice (slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

pub fn main() {

     //A tuple with a bunch of different types.
     let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

     //Values can be extracted from the tuple using tuple indexing.
     println!("Long tuple's first value: {}", long_tuple.0);
     println!("Long tuple's second value: {}", long_tuple.1);

    let tuple_example = ("Rust", "fun", 100);
    println!("Tuple contents = {:?}", tuple_example);
    println!("Tuple's first value: {}", long_tuple.0);
    println!("Tuple's second value: {}", long_tuple.1);
    println!("Tuple's third value: {}", long_tuple.2);

    //Tuples can be members.
    let tuple_of_tuples: ((u8, u16, u32), (u64, i8), _) = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    //Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    //But long Tuples (more than 12 elements) cannot be printed.
    //Let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);
    //TODO: Uncomment the above 2 lines to see the compiler error

    let pair: (i32, bool) = (1, true);
    println!("Fair is {:?}", reverse(pair));

    //To create one element tuples, the comma is required to tell them apart from a literal surrounded by parenthesis.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    //Tuples can be deconstructed to create bindings.
    let tuple: (i32, &str, f64, bool) = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix: Matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    //Fixed-size array (type signatureis superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    //All elements can be initiated to the name value.
    let ys: [i32; 500] = [0; 500];

    //Indexing starts at 0 just like arrays.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // 'len' returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    //Arrays are stack aallocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    //Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    //Slices can point to a section of an array.
    //They are of the form [staring_index..ending_index],
    // 'starting_index' is the first position in the slice.
    // 'ending_index' is one more than the last position in the slice.
    println!("Borrow a section of the array as the slice.");
    analyze_slice(&ys[1 .. 4]);

    //Example of empty slice '&[];
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using '.get', which returtns an 'Option'.
    //This can be matched as shown below, or used with '.except()' if you would like the program to exit with a nice
    //message instead of happily continue.
    
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i){
            Some(xval) => println! ("{}: {}", i, xval),
            None => println! ("Slow down! {} is too far!", i),
        }  

    /*pub fn multiplier (arr: &[f64]) -> f64 {
        let i: usize = 0;
        let product: f64 = 1;
        while i < arr.len() {
            product = product*a+1;
            i+=1;
        }
        return product;
    }*/

    //Arrays
    let arr1:[i32;4] = [10, 20, 30, 40];
    println!("Contents of the array are {:?}", arr1);

    // Out of bound indexing on array causes compile time  error.
    //println! ("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    // println! ("{}", xs[..][5]); 
}
}