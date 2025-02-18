use std::future::Future;

use super::User;

async fn coerce_to_user<'a>(name: &'a String) -> User<'a> {
    User { name }
}

pub async fn create_users_2<'a>(names: &Vec<&'a String>) -> Vec<User<'a>> {
    let mut users: Vec<User<'a>> = Vec::new();
    for name in names.iter() {
        users.push(coerce_to_user(name).await);
    }
    users
}

pub async fn create_users_3<'a>(names: &Vec<&'a str>) -> Vec<User<'a>> {
    let mut users: Vec<User<'a>> = Vec::new();
    for name in names.iter() {
        users.push(User { name });
    }
    users
}

// pub async fn run<'a>(h: &'a String) {
//     let x = vec![h];
//     let x = Box::pin(async move {
//         create_users_2(&x).await;
//     });
//     tokio::spawn(x).await;
//     // runc(x);
// }

pub async fn runc<T: Future<Output = ()>>(x: T) {}
