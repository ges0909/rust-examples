use std::env::args;

fn main() {
    let mut n = 0;
    for arg in args() {
        n += 1;
        println!("{}. {}", n, arg)
    }
    // there is also a 'getopts' in rust library
}
