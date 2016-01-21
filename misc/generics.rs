// generic data
struct Pair<T> {
    first: T,
    last: T,
}

// generic function
fn last<T>(pair: Pair<T>) -> T {
    pair.last
}

// Option
fn anyfunc(none: bool) -> Option<&'static str> {
    if none {
        return None;
    }
    Some("ALLES OKAY")
}

fn main() {
    // T = u64
    let pair1 = Pair {
        first: 1u64,
        last: 2,
    };
    // T = &str
    let pair2 = Pair {
        first: "Gerrit",
        last: "Schrader",
    };

    println!("{} -- {}", last(pair1), last(pair2));

    // generic enum
    enum MyOption<T> {
        Some(T),
        None(T),
    }
    let x1: MyOption<isize> = MyOption::Some(9);
    let x2: MyOption<&str> = MyOption::Some("Blabla");
    let x3: MyOption<&str> = MyOption::None("File not found");

    match x3 {
        MyOption::Some(val) => println!("{}", val),
        MyOption::None(err) => println!("Error: {}", err),
    }

    // std::Option
    let mut o: Option<&str> = anyfunc(false);
    match o {
        Some(text) => println!("{}", text),
        None => println!("Error"),
    }

    match anyfunc(true) {
        Some(text) => println!("{}", text),
        None => println!("NONE"),
    }

    let n = 42;
    // the 'ref' keyword creates a reference fo use in the pattern
    match n {
        ref r => println!("r={}, *r={}", r, *r),
    }

}
