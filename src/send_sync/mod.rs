use std::{rc::Rc, time::Duration};

use tokio::task;

async fn foo() {
    let _ = task::spawn(async move {
        let x = vec!["hello", "world"];
        // it's fine to have a !Send type within a closure as long as it's not held across await boundaries in a send closure
        let mut rx = Rc::new(x);
        // uncommenting this makes the closure !Send
        // tokio::time::sleep(Duration::from_secs(1)).await;
        let rx = Rc::get_mut(&mut rx);
        for v in rx.into_iter() {
            println!("{:?}", v)
        }
    })
    .await;
}

async fn foo2() {
    let x = vec!["hello", "world"];
    // it's fine to have a !Send type within a closure as long as it's not pushed into a spawn
    let mut rx = Rc::new(x);
    tokio::time::sleep(Duration::from_secs(1)).await;
    let rx = Rc::get_mut(&mut rx);
    for v in rx.into_iter() {
        println!("{:?}", v)
    }
}

fn foo_send<T: Send>(x: T) {}

async fn run() {
    // future cannot be sent between threads safely
    // within `impl Future<Output = ()>`, the trait `Send` is not implemented for `Rc<Vec<&str>>
    // tokio::spawn(foo2());
}
