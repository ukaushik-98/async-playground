use std::rc::Rc;

use tokio::task;

async fn foo() {
    let _ = task::spawn(async move {
        let x = vec!["hello", "world"];
        // it's fine to have a !Send type within a closure as long as it's not pushed into a spawn
        let mut rx = Rc::new(x);
        let rx = Rc::get_mut(&mut rx);
        for v in rx.into_iter() {
            println!("{:?}", v)
        }
    })
    .await;
}
