use std::sync::Arc;

use crate::lifetimes::{
    arc_users::create_users,
    greet_users::greet_users_runner,
    ref_users::{create_users_2, create_users_3},
};

pub async fn runner(names: Arc<Vec<&str>>) {
    println!("Started Runner...");
    let users = create_users(names).await;
    greet_users_runner(users).await;
}

pub async fn runner2(names: &Vec<&String>) {
    println!("Started Runner...");
    let users = create_users_2(names).await;
    greet_users_runner(users).await;
}

// do this better - coerce String into ref and use a trait bound
pub async fn runner3(names: &Vec<&str>) {
    println!("Started Runner...");
    let users = create_users_3(&names).await;
    greet_users_runner(users).await;
}
