
// higher-order func's are func's with other func's as arguments
fn high_order<F: Fn() -> &'static str>(func_or_closure: F, n: usize) {
    for i in 0..n {
        let s = func_or_closure();
        println!("{}. {}", i, s)
    }
}

fn high_order2<F: Fn() -> String>(func_or_closure: F, n: usize) {
    for i in 0..n {
        let s = func_or_closure();
        println!("{}. {}", i, s)
    }
}

fn simple_func() -> &'static str {
    "simpleFunc() is called"
}

fn main() {
    // higher-order func
    // hand-over a named func as argument to another func
    high_order(simple_func, 3);

    // hand-over an anonymous func => closure
    // closure with stmt. => curly braces may be omitted
    let closure = || "closure called"; // value of type "&'static str" is allocated on stack
    high_order(closure, 2);

    // closure => "surrounding scope is enclosed"
    // inlined closure
    // value of type "String" is allocated on heap
    let hint = "without binding";
    high_order2(|| {
                    let mut s = "closure called".to_string();
                    s.push_str(" ");
                    s.push_str(hint);
                    s
                },
                1);
}
