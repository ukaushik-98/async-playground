use std::sync::Arc;

use runner::{runner, runner2, runner3};
use tokio::task;

pub mod arc_users;
pub mod greet_users;
pub mod ref_users;
pub mod runner;
pub mod spawn;
pub struct User<'a> {
    pub name: &'a str,
}

/// ❌ passing refs won't work because spawn expectes F:'static.
/// This means either the names must be:
///  - Live for static
///     - Either we have a &str which will have 'static.
///     - Box::leak() which pushes the object into the heap and then "safely" leaks it.
///  - Owned Type, which is what move is for.
///  - Arc to push the object into a box and have a shared ref.
pub async fn ref_runner() {
    println!("ref runner...");
    let name1 = "user1";
    let name2 = "user2";
    let name1_ref: &str = &name1;
    let name2_ref: &str = &name2;
    let _ = tokio::spawn(async move {
        let names: Vec<&str> = vec![name1_ref, name2_ref];
        runner3(&names).await;
    })
    .await;
}

/// ✅ use arc to share a ref
pub async fn arc_runner() {
    println!("arc runnner...");
    let name1 = "user1";
    let name2 = "user2";
    let names: Vec<&str> = vec![&name1, &name2];
    let names = Arc::new(names);
    let n_clone = names.clone();
    let _ = tokio::spawn(runner(n_clone)).await;
    println!("{:?}", names)
}

/// ✅ move context to spawn thread and pass ownership
/// i.e. owned type
pub async fn owned_runner() {
    println!("owned runnner...");
    let name1 = "user1".to_owned();
    let name2 = "user2".to_owned();
    let _ = tokio::spawn(async move {
        let names: Vec<&String> = vec![&name1, &name2];
        garbage(&names).await;
        println!("names: {:?}", names);
    })
    .await;
}

async fn sum_random_large_comp() {
    let t = task::spawn(async {
        let x = vec!["hello"];
        let y = &x;
    })
    .await;
}

async fn garbage<'a>(names: &'a Vec<&String>) {
    let hello = String::from("hello");
    let y: &String = &hello;
    let x = vec![&y];
    sum_random_large_comp().await;
    // reference can be kept across await points with a 'a bound because the data is captured by the future
    // however, once a spawn is required, that requires spawn to OWN the data.
    println!("x: {:?}", x);
}
