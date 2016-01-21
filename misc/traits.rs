//
struct Person {
    first: &'static str,
    last: &'static str,
}

impl Person {
    fn new(_first: &'static str, _last: &'static str) -> Person {
        Person {
            first: _first,
            last: _last,
        }
    }
}

// trait
trait Logable {
    fn log(&self, msg: &str) {
        println!("Default Logable: {}", msg);
    }
}

impl Logable for Person {
    fn log(&self, msg: &str) {
        println!("Logable for Person: {}", msg);
    }
}

fn use_trait_as_type<T: Logable>(l: T) {
    l.log("use a trait as type");
}

//
struct Birth {
    day: u8,
    month: u8,
    year: u16,
}

impl Logable for Birth {}

fn main() {
    // new() -> static method
    let mut ich = Person::new("Gerrit", "Schrader");
    ich.log("I implement the Logable trait");
    //
    use_trait_as_type(ich);
    //
    let geburtstag = Birth {
        day: 9,
        month: 9,
        year: 1996,
    };
    geburtstag.log("I us the default Logable trait")
}
