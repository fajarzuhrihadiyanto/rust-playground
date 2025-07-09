mod macros;
mod api;
use api::v1::auth::models::user::User;
mod lib;
use lib::mongodb::{insert, find_all};

#[tokio::main]
async fn main() {
    let crate_name = env!("CARGO_PKG_NAME");
    println!("I am in crate `{}`", crate_name);

    // Using the User struct from the macros module
    let user = User::new(
        "John Doe".to_string(),
        "john.doe@example.com".to_string(),
        "password".to_string(),
        20
    );
    println!("User: {:?}", user);

    // insert("users".into(), user).await;
    find_all::<User>("users".into()).await;
}
