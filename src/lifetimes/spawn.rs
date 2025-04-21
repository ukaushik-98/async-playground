use std::{future::Future, rc::Rc, time::Duration};

use tokio::time::sleep;

async fn foo<'a, T>() -> T
where
    T: 'a,
{
    todo!()
}

async fn fooRc() -> Rc<u8> {
    Rc::new(9)
}

fn send_check<T>(x: T)
where
    T: Send,
{
}

async fn foo2<'a, T>(x: T) -> T
where
    T: 'a,
{
    let _ = tokio::spawn(async move {
        let a = foo::<T>();
        let a = a.await;
        // sleep(Duration::from_micros(1)).await;
        let x = a;
    })
    .await;
    x
}

async fn foo3() {
    let z = async move {
        let a = fooRc();
        let x = a.await;
        // sleep(Duration::from_micros(1)).await;
        let y = x.clone();
    };
    let _ = tokio::spawn(z).await;
    // let _ = tokio::spawn(async move {
    //     let r = Rc::new(5);
    //     let a = fooRc();
    //     let x = a.await;
    //     let y = x.clone();
    // })
    // .await;
}

// async fn boo2() {}

// async fn boo<T>() {
//     let f1: Box<dyn Future<Output = _> + Send> = foo::<T>().await;
//     tokio::spawn(async move {
//         let f2 = foo2(f1).await;
//     })
//     .await;
// }
