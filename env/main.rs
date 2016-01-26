use std::env;

fn main() {
    let vars = env::vars();
    for (key, value) in vars {
        println!("{}: {}", key, value);
    }
}
