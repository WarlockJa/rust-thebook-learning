use std::io;

fn main() {
    let x: f64 = -5.0 / 3.0;

    // let x = x + 1;

    {
        let x: f64 = x * 2.0;
        println!("the value of x in the inner scope is {x}");
    }

    println!("the value of x is {x}");

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("{x} {y} {z}");

    // array in Rust has fixed length
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // array short-hand
    let a = [3; 5];

    for n in 0..a.len() {
        let str = a[n];
        println!("{str}")
    }

    let mut index = String::new();
    println!("Enter arr index");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Entered index is not a number");

    // if index is a number not in 0..4 this line will throw an error
    let element = arr[index];

    println!("The value of the element is {element}")
}
