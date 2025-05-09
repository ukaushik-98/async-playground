use std::sync::Arc;

use super::User;

async fn coerce_to_user<'a>(name: &'a str) -> User<'a> {
    User { name }
}

pub async fn create_users(names: Arc<Vec<&str>>) -> Vec<User> {
    let mut users = Vec::new();
    for name in names.iter() {
        users.push(coerce_to_user(name).await);
    }
    users
}
