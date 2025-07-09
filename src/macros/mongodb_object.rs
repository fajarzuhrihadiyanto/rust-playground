/// This macro generates a struct with the given fields and default values
#[macro_export]
macro_rules! generate_mongodb_object {
    ($name:ident {$($field:ident: $type:ty),*}) => {
        use mongodb::bson::oid::ObjectId;
        use mongodb::bson::DateTime;
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct $name {
            pub _id: ObjectId,
            pub created_at: DateTime,
            pub updated_at: DateTime,
            $(pub $field: $type),*
        }

        impl $name {
            pub fn new($($field: $type),*) -> Self {
                Self {
                    _id: ObjectId::new(),
                    created_at: DateTime::now(),
                    updated_at: DateTime::now(),
                    $($field: $field),*
                }
            }
        }
    }
}