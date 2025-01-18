use std::{sync::Arc, time::Duration};

use tokio::time::sleep;

struct User<'a> {
    name: &'a str,
}

async fn coerce_to_user<'a>(name: &'a str) -> User<'a> {
    User { name }
}

async fn create_users(names: Arc<Vec<&str>>) -> Vec<User> {
    let mut users = Vec::new();
    for name in names.iter() {
        users.push(coerce_to_user(name).await);
    }
    users
}

async fn greet_users<'a>(users: Vec<User<'a>>) {
    for user in users.iter() {
        let _ = sleep(Duration::from_millis(100)).await;
        println!("Hello, {}!", user.name);
    }
}

async fn runner(names: Arc<Vec<&str>>) {
    println!("Started Runner...");
    let users = create_users(names).await;
    greet_users(users).await;
}

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
