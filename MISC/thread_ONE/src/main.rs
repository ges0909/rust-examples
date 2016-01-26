// extern crate std;
extern crate time;

use std::thread;
use time::{PreciseTime, Duration};

/**
fn main() {
    let start = PreciseTime::now();
    let handles: Vec<_> = (0..10)
                              .map(|_| {
                                  thread::spawn(|| {
                                      let mut x = 0;
                                      for _ in 0..5_000_000_0 {
                                          x += 1
                                      }
                                      x
                                  })
                              })
                              .collect();

    for h in handles {
        println!("Thread finished with count={}",
                 h.join().map_err(|_| "Could not join a thread!").unwrap());
    }
    let end = PreciseTime::now();
    let duration: time::Duration = start.to(end);
    println!("Duration={0} ms", duration.num_milliseconds());
}
*/
fn main() {

    let start = PreciseTime::now();

    let handles: Vec<thread::JoinHandle<_>> = (0..10)
                                                  .map(|_| {
                                                      thread::spawn(|| {
                                                          let mut x = 0;
                                                          for _ in 0..5_000_000_00 {
                                                              x += 1
                                                          }
                                                      })
                                                  })
                                                  .collect();

    println!("{} threads started -- wait on end", handles.len());

    for h in handles {
        match h.join() { // join() returns Result<T>
            Ok(val) => println!("okay"),
            Err(msg) => println!("failed"),
        }
    }

    let end = PreciseTime::now();
    let duration: time::Duration = start.to(end);

    println!("Duration={0} ms", duration.num_milliseconds());
}
