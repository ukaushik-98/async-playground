use std::{fmt::Debug, rc::Rc, time::Duration};

use tokio::time::sleep;

async fn create_t<'a, T>() -> &'a T
where
    T: 'a,
{
    todo!()
}

fn static_check<T>(x: T)
where
    T: 'static + Send,
{
}

pub async fn foo<'a, T>()
where
    T: Send + Sync + 'a,
{
    let a = Box::pin(async move {
        let x = create_t::<T>().await;
        sleep(Duration::from_micros(100)).await;
    });
    static_check(a);
    // static_check(create_t::<T>());
    // let _ = tokio::spawn(a).await;
    // let _ = tokio::spawn(create_t::<T>()).await;
}
