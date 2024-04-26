/*
-Variables
    -Definintion
    -mutability
    -scope
    -Shadowing
-Constants

*/

use std::array;

fn main() {
    // Definition
    let x: i16 = 10;
    println!("x is {x}");

    // Mutability
    let mut y: i32 = 5;
    y = 10;

    // Scope
    {
        let z: i32 = 50;
    }
    // let s = z; not in this scope doesnt work

    // Shadowing
    let t: i32 = 10;
    let t: i32 = t + 10;
    println!("t is {t}");

    let u: i32 = 3;
    let u: f64 = 3.0;

    let v: i32 = 30;
    {
        let v: i32 = 40;
        println!("inner v is {v}")
    }
    println!("v is {v}");

    // Constants
    const MAX_VALUE: u32 = 100;

    // unsigned ints 
    let unsigned_int: u8 = 5;

    // signed ints 
    let signed_int: i8 = 5;

    // floating point nums 
    let floating_num: f32 = 5.0; //f64 default

    // Characters 
    let chat: char = 'a';

    // Boolean 
    let b: bool = true;

    // Type Aliasing
    type Age = u8;
    let pete_age: Age = 42;

    // Type Conversion
    let a:i32 = 10;
    let b: f64 = a as f64;


    // &str and String
    let fixed_str: &str = "Fixed length string";
    let mut flexible_str: String = String::from("This String will grow");

    // Arrays
    let mut array_1: [i32; 5] = [4, 3, 5, 6, 7];

    println!("{:?}", array_1);
    let array_2 = [0; 10];
    
    // Vectors
    let vec_1: Vec<i32> = vec![4, 5, 6, 8, 9];
    let num = vec_1[3];

    // Tuples
    let my_info = ("Salary", 40000, "Age", 40);
    let salary_value = my_info.1;
    let (salary, salary_value, age, age_value) = my_info;

    let unit = ();


}
