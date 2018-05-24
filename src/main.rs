use std::io;    

fn main() { 
    println!("Calculating Fibonacci Sequence...");
    let term_number = get_term_number();
    let answer = recursive_fib(term_number);
    println!("The value for term {} of the fibonacci sequence is: {}", term_number, answer);
}

fn recursive_fib(n: u64) -> u64 {
    if n == 0 {
        return 0
    }
    else if n == 1 {
        return 1 
    }
    else {
        return recursive_fib(n - 1) + recursive_fib(n - 2)
    }
}

fn get_line() -> String {
    let mut line = String::new();

    io::stdin().read_line(&mut line)
        .expect("Failed to read line");

    return line
}

fn get_term_number() -> u64 {
     loop {
        println!("Enter term number...");

        let term_number = get_line();

        let term_number: u64 = match term_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return term_number
    }
}
