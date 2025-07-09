use crate::generate_mongodb_object;

generate_mongodb_object! {
    User {
        name: String,
        email: String,
        password: String,
        age: u8
    }
}