use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

fn main() {
    let (sx, rx): (Sender<i32>, Receiver<i32>) = channel();
    thread::spawn(move || { sx.send(10).unwrap() });
    let res = rx.recv().unwrap();
    println!("{:?}", res);
}
