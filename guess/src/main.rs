// see: https://doc.rust-lang.org/stable/book/guessing-game.html

// crate `std` is always imported automatically
extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    // '!' refers to a macro
    println!("Guess the number!");

    // rand::thread_rng() gets a copy of the random number generator, which
    // is local to Theparticular thread of execution we’re in
    // see: https://github.com/rust-lang-nursery/rand
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");

        // String = UTF-8 encoded, growing, provided by 'std'
        // new = static method, creates new empty string
        let mut guess = String::new(); // type inference

        // read_line() takes a mutable reference to a string
        // read_line() returns a io::Result, which has has the ok() method
        // ok() method returns a value with the expect() method
        // if expect() method isn't successful it panic!'s
        io::stdin()
            .read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        // ‘shadow’ the previous guess with a new one
        // trim() removes '\n' at the end of input string
        // annotate type wih 'u32' to give Rust a hint what number we want
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // loop
        };

        // '{}' makes type specific string conversion
        println!("You guessed: {}", guess);

        // cmp() method can be called on anything that can be compared, and it takes
        // a reference to the thing you want to compare it to; 'Ordering' is a enum
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // loop
            }
            // _                 => println!("something else")
        }
    }
}
