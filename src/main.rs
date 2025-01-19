use std::sync::Arc;

use async_playground::lifetimes::{runner, runner2};

async fn arc_run() {
    let name1 = "udit";
    let name2 = "parit";
    let names: Vec<&str> = vec![&name1, &name2];
    let names = Arc::new(names);
    let n_clone = names.clone();
    let _ = tokio::spawn(runner(n_clone)).await;
    println!("{:?}", names)
}

async fn arc_run_2() {
    let name1 = "udit";
    let name2 = "parit";
    let _ = tokio::spawn(async move {
        let names: Vec<&str> = vec![&name1, &name2];
        runner2(&names).await;
    })
    .await;
}

#[tokio::main]
async fn main() {
    // arc_run().await;
    arc_run_2().await;
}
