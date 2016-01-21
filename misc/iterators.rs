fn main() {

    let mut range = -3..3;

    // loop 1
    loop {
        let n = range.next();
        // let n: Option<isize> = range.next();
        match n {
            Some(_) => println!("{:?}", n),
            None => {
                println!("{:?}", n);
                break;
            }
        }
    }

    // loop 2
    range = -2..2;

    loop {
        let n: Option<isize>;
        match {
            n = range.next();
            n
        } {
            Some(_) => println!("{:?}", n),
            None => {
                println!("{:?}", n);
                break;
            }
        }
    }

    //
    range = 0..10;
    let numbers: Vec<isize> = range.filter(|n| n % 2 == 0)
                                   .map(|n| n * n)
                                   .take(3)
                                   .collect();
    for n in numbers {
        println!("{}", n)
    }

    range = 0..10;
    let sum = range.filter(|n| n % 2 == 0)
                   .map(|n| n * n)
                   .take(3)
                   .fold(0, |accu, n| accu + n);

    println!("sum={}", sum);
}
