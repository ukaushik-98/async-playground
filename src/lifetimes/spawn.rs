async fn foo<'a, T>(x: T)
where
    T: 'a,
{
}

async fn foo2<'a, T>(x: T)
where
    // fails to compile because 'a it not 'static
    // T: Send + 'a,
    T: Send + 'static,
{
    tokio::spawn(foo(x)).await;
}
