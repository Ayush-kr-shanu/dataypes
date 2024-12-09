use std::io;
fn main() {
    // there is 2 types of dataTypes... one is scalar and another is compound
    // scalar data types are:
    // - integers (i8, u8, i16, u16, i32, u32, i64, u64, isize, usize)
    // - floats (f32, f64)
    // - characters (char)
    // - booleans (bool)

    // let's declare some variables
    // let x: i32 = 5; // number
    // let y: f64 = 3.14; // float

    // let sum = 5 + 10;

    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1

    // // remainder
    // let remainder = 43 % 5;

    // // printing the values
    // println!("x = {}", x);
    // println!("y = {}", y);
    // println!("sum = {}", sum);
    // println!("difference = {}", difference);
    // println!("product = {}", product);
    // println!("quotient = {}", quotient);
    // println!("truncated = {}", truncated);
    // println!("remainder = {}", remainder);

    // let z: char = 'a'; // string
    // let is_true: bool = true; // boolean
    // let is_false: bool = false; // boolean

    // // printing the values
    // println!("z = {}", z);
    // println!("is_true = {}", is_true);
    // println!("is_false = {}", is_false);

    // // compound type
    // // compound data types are:
    // // - tuples
    // // - arrays
    // // - structs
    // // - enums

    // // tuples
    // let tup = (1, "ayush", 3.5);
    // println!("Print name = {}", tup.1);

    // // arrays
    // let arr = [1, 2, 3];
    // println!("Print second element = {}", arr[1]);
    // let arr2 = [3; 8];
    // println!("Print all elements = {:?}", arr2);

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}