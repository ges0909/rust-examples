fn myfunc(n: isize) -> Result<isize, String> {
    if n % 2 == 0 {
        return Result::Ok(n);					// returns Ok
    }
    Result::Err(format!("odd number {}", n))	// returns Err
}

fn main() {
    // explicit match
    let r = match myfunc(2) {
        Ok(num) => num,							// r = num
        Err(msg) => panic!(msg),				// write message and exit
    };
    println!("{}", r);

    // match with unwrap() returning Ok
    let r = myfunc(4);
    println!("{}", r.unwrap()); 				// unwrap() is doing the match

    // match with unwrap() result in panic!
    let r = myfunc(3);
    println!("{}", r.unwrap()); 				// unwrap() and panic
}
