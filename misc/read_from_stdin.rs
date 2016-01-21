use std::io::{self, Read};

fn main() {
    // {:?} => Debug formatting
    let hallo = "Hallo";
    println!("{} -- {:?}", hallo, hallo);

    // flushing output
    let mut buf = String::new();
    println!("Press some keys: ");
    let result = io::stdin().read_to_string(&mut buf);
    let result2 = result.ok();
    match result2 {
        Some(num) => println!("number of chars = {}", num),
        None => println!("FEHLER"),
    }
}
