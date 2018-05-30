use std::io;    
use std::collections::HashMap;    

fn main() { 
    println!("Calculating Fibonacci Sequence...");
    let mut fib_seq: HashMap<u64, u64> = HashMap::new();
    fib_seq.insert(0, 0);
    fib_seq.insert(1, 1); // initializing base cases
    let term_number = get_term_number();
    let answer = recursive_fib(term_number, &mut fib_seq);
    println!("The value for term {} of the fibonacci sequence is: {}", term_number, answer);
}


/// uses recursion and memoization to return the nth term of the fib sequence
fn recursive_fib(n: u64, fib_seq: &mut HashMap<u64, u64>) -> u64 {
    {
        let nth_fib = fib_seq.get(&n);
        match nth_fib {
            Some(num) => return *num,
            _ => (), 
        }
    }
    let nth_fib = recursive_fib(n - 1, fib_seq) + recursive_fib(n - 2, fib_seq);
    fib_seq.insert(n, nth_fib);
    return nth_fib
}

/// returns a line from the user
fn get_line() -> String {
    let mut line = String::new();

    io::stdin().read_line(&mut line)
        .expect("Failed to read line");

    return line
}

/// asks for a number from the user repeatedly until a number is given
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
