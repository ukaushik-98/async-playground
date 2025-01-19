use std::sync::Arc;

use arc_users::create_users;
use greet_users::greet_users_runner;
use ref_users::create_users_2;

pub mod arc_users;
pub mod greet_users;
pub mod ref_users;

pub struct User<'a> {
    pub name: &'a str,
}

pub async fn runner(names: Arc<Vec<&str>>) {
    println!("Started Runner...");
    let users = create_users(names).await;
    greet_users_runner(users).await;
}

pub async fn runner2(names: &Vec<&str>) {
    println!("Started Runner...");
    let users = create_users_2(names).await;
    greet_users_runner(users).await;
}
