extern crate futures;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::executor::block_on;

async fn print_hello(mut tx: Sender<i32>) {
    let numbers = &[2];
    for n in numbers {
        tx.try_send(*n).unwrap();
    }
}

async fn print_world(mut rx: Receiver<i32>) {
    let res = rx.try_next().unwrap();
    match res {
        Some(x) => println!("{}", x),
        None => println!("err"),
    }
}

async fn async_main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel(1);
    let f = print_hello(tx);
    let g = print_world(rx);

    (f.await, g.await);
}

fn main() {
    block_on(async_main())
}
