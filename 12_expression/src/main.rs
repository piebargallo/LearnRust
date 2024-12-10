// Expressions evaluate to a result value. 
// Let´s look at some examples.

fn main() {
    let y = {
        let x = 3;
        // Expressions do not include ending semicolons.
        x + 1
    };
    println!("Hello, the value of y is: {y}");
}
