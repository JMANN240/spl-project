use std::io;
use std::io::Write;

                // Rust is compiled, and therefore requires and entry point.
                // This entry point is the main function, which is what will
                // execute when the compiled binary is run.
fn main() {
    let n: u32; // Rust is a statically-typed language. The types must be 
                // known at compile time and cannot change. The variable
                // n is of type u32 (unsigned 32-bit integer)
    let mut buf = String::new();
    print!("Find the fibonacci number at index: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).expect("Failed"); 
                // In the above line we see that rust can pass by value or
                // by reference. In this case it is passed by reference
                // because of the & symbol.
    n = buf.trim().parse::<u32>().unwrap();

    let mut current: u32 = 0;
    let mut prev: u32 = 1;
    let mut temp: u32;

    for _ in 0..n {
        temp = current;
        current += prev;
        prev = temp;
    }

    println!("The {}th fibonacci number is {}", n, current);
}
