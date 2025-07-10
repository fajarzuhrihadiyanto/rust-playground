/// This macro generates a struct with the given fields and default values
#[macro_export]
macro_rules! generate_mongodb_object {
    ($name:ident {$($field:ident: $type:ty),*}) => {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            pub _id: mongodb::bson::oid::ObjectId,
            pub created_at: mongodb::bson::DateTime,
            pub updated_at: mongodb::bson::DateTime,
            $(pub $field: $type),*
        }

        impl $name {
            pub fn new($($field: $type),*) -> Self {
                Self {
                    _id: mongodb::bson::oid::ObjectId::new(),
                    created_at: mongodb::bson::DateTime::now(),
                    updated_at: mongodb::bson::DateTime::now(),
                    $($field: $field),*
                }
            }
        }
    }
}