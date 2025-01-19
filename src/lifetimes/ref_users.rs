use super::User;

async fn coerce_to_user<'a>(name: &'a str) -> User<'a> {
    User { name }
}

pub async fn create_users_2<'a>(names: &Vec<&'a str>) -> Vec<User<'a>> {
    let mut users: Vec<User<'a>> = Vec::new();
    for name in names.iter() {
        users.push(coerce_to_user(name).await);
    }
    users
}
