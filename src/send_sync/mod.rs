use std::{rc::Rc, time::Duration};

pub mod implicit_type;

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

fn create_vec<'a, T>() -> &'a T {
    todo!()
}

async fn foo2<'a, T>()
where
    T: 'a,
{
    let x = create_vec::<'a, T>();
    // it's fine to have a !Send type within a closure as long as it's not pushed into a spawn
    tokio::time::sleep(Duration::from_secs(1)).await;
}

fn foo_send<'a, T: Send + 'a>(x: T) {}

async fn run<T>() {
    // foo_send(foo2::<T>());
    let _ = tokio::spawn(async move {
        let x = foo2::<T>().await;
    })
    .await;
    // future cannot be sent between threads safely
    // within `impl Future<Output = ()>`, the trait `Send` is not implemented for `Rc<Vec<&str>>
    // tokio::spawn(foo2());
}
