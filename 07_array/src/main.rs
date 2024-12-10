use std::io;

fn main() {
    let months = ["January", "February", "March", "April", "June",
                  "Jully", "August", "September", "October",
                   "November", "December"];

    // Accesing array elements
    let first = months[0];
    println!("The first element is: {first}");

    // Invalid array element access
    println!("Enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = months[index];

    println!("The value of the element at index {index} is : {element}");
}
