use std::sync::Arc;

use runner::{runner, runner2};

pub mod arc_users;
pub mod greet_users;
pub mod ref_users;
pub mod runner;

pub struct User<'a> {
    pub name: &'a str,
}

///✅ use arc to share a ref
pub async fn arc_run() {
    println!("arc runnner....");
    let name1 = "udit";
    let name2 = "parit";
    let names: Vec<&str> = vec![&name1, &name2];
    let names = Arc::new(names);
    let n_clone = names.clone();
    let _ = tokio::spawn(runner(n_clone)).await;
    println!("{:?}", names)
}

/// ❌ this won't work because spawn expectes 'static.
/// This means either the names must be:
///  - Box::leaked
///  - Arc
///  - Owned Types
// pub async fn arc_run_failed() {
//     let name1 = "udit".to_owned();
//     let name2 = "parit".to_owned();
//     let name1_ref = &name1;
//     let name2_ref = &name2;
//     let _ = tokio::spawn(async move {
//         let names: Vec<&String> = vec![name1_ref, name2_ref];
//         runner2(&names).await;
//     })
//     .await;
// }

/// ✅ move context to spawn thread and pass ownership
/// i.e. owned type
pub async fn owned_runner() {
    println!("owned runnner 2....");
    let name1 = "udit".to_owned();
    let name2 = "parit".to_owned();
    let _ = tokio::spawn(async move {
        let names: Vec<&String> = vec![&name1, &name2];
        runner2(&names).await;
    })
    .await;
}
