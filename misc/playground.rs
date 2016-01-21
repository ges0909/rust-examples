fn main() {

    struct UserDefined<'a> {
        value: &'a u64, // error: missing lifetime sepcifier
    }

    // --
    struct Person {
        name: &'static str, // error: missing lifetime sepcifier
        age: u64,
    }

    // {
    let p = Person {
        name: "Dieter",
        age: 60,
    };
    println!("name={}, age={}", p.name, p.age);
    // }
    // println!("name={}, age={}", p.name, p.age); // error: unresolved name `p`

    let p2 = p;
    println!("name={}, age={}", p.name, p.age);
    // p2.age = 50; // error: cannot assign to immutable field `p.age`

    let p3 = &mut p;
    p3.age = 50;
}
