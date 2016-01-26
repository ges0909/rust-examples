fn main() {
    // conversion
    let string = "123";
    // trim() removes leading and trailing white spaces
    // parse() returns Result<T, E>
    // unwrap() destructs value of type T from Result<T, E>
    let number: usize = string.trim().parse().unwrap();
    println!("number = {0}", number);
}
