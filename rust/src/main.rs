use std::io;
use std::io::Write;

fn main() {
    let n: u32;
    let mut buf = String::new();
    print!("Find the fibonacci number at index: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).expect("Failed");
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
