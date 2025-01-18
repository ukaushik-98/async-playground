use std::time::Duration;
use tokio::time::sleep;

use super::arc_users::User;

pub async fn greet_users_runner<'a>(users: Vec<User<'a>>) {
    for user in users.iter() {
        let _ = sleep(Duration::from_millis(100)).await;
        println!("Hello, {}!", user.name);
    }
}
