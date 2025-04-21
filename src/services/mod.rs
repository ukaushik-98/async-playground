// use std::{future::Future, process::Output, task::Poll};

// struct SimpleFuture;

// impl Future for SimpleFuture {
//     type Output = ();

//     fn poll(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         todo!()
//     }
// }

// struct InnerService {
//     inner: SimpleFuture,
// }

// impl Future for InnerService {
//     type Output = SimpleFuture;

//     fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
//         Poll::Ready(self.inner)
//     }
// }

// struct OuterService<'a> {
//     inner: InnerService<'a>,
// }

// impl<'a> Future for OuterService<'a> {
//     type Output = &'a mut SimpleFuture;

//     fn poll(
//         mut self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         todo!()
//     }
// }

// async fn runner() {
//     let mut simple = SimpleFuture;
//     // let inner = InnerService { inner: &mut simple };
//     let outer = OuterService { inner: inner };
//     let x = outer.await;
// }

async fn baz() {}

async fn run() {
    let x = baz();
    x.await;
}
