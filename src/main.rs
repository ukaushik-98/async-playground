use std::sync::Arc;

use async_playground::lifetimes::runner;

#[tokio::main]
async fn main() {
    let name1 = "udit";
    let name2 = "parit";
    let names: Vec<&str> = vec![&name1, &name2];
    let names = Arc::new(names);
    let n_clone = names.clone();
    let _ = tokio::spawn(runner(n_clone)).await;
    println!("{:?}", names)
}
