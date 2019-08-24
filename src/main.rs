extern crate futures;
use futures::executor::block_on;
use std::sync::mpsc::{channel, Receiver, Sender};

async fn print_hello(tx: Sender<i32>) {
    let numbers = &[2, 5, 6, 7, 8];
    for n in numbers {
        tx.send(*n).unwrap();
    }
}

async fn print_world(rx: Receiver<i32>) {
    for i in rx.iter() {
        println!("{}", i);
    }
}

async fn async_main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let f = print_hello(tx);
    let g = print_world(rx);

    (f.await, g.await);
}

fn main() {
    block_on(async_main())
}
