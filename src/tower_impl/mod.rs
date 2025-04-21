use std::{rc::Rc, time::Duration};

use tokio::time::sleep;
use tower::{service_fn, BoxError, Service, ServiceExt};

#[derive(Debug, Clone, Copy)]
struct Request<'a> {
    inner: &'a String,
}

impl<'a> Request<'a> {
    fn new(x: &'a String) -> Self {
        Request { inner: &x }
    }
}
struct Response<'a>(&'a str);
impl<'a> Response<'a> {
    fn new(body: &'a str) -> Self {
        Self(body)
    }
    fn into_body(self) -> &'a str {
        self.0
    }
}

fn static_check<T>(x: T) -> T
where
    T: Send + 'static,
{
    x
}

async fn handle<'a>(request: Request<'a>) -> Result<Response<'a>, BoxError> {
    // let x = Rc::new(5);
    let response = Response::new(request.inner);
    let _ = sleep(Duration::from_millis(100)).await;
    Ok(response)
}

async fn runner<'a>() -> Result<(), BoxError> {
    let mut service = service_fn(handle);
    let x = async move {
        let inner = String::from("hello");
        let req = Request::new(&inner);
        let response = service.ready().await.unwrap().call(req).await.unwrap();

        let _ = sleep(Duration::from_micros(100)).await;

        assert_eq!("Hello, World!", response.into_body());
    };
    let _ = tokio::spawn(x).await;

    Ok(())
}
