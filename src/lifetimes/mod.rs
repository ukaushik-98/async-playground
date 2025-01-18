use std::sync::Arc;

use arc_users::create_users;
use greet_users::greet_users_runner;

pub mod arc_users;
pub mod greet_users;

pub async fn runner(names: Arc<Vec<&str>>) {
    println!("Started Runner...");
    let users = create_users(names).await;
    greet_users_runner(users).await;
}
