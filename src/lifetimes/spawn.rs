async fn foo<'a, T>(x: T)
where
    T: 'a,
{
}

async fn foo2<'a, T>(x: T)
where
    T: Send + 'static,
{
    tokio::spawn(foo(x)).await;
}
