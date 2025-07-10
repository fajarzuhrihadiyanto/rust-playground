/// This macro generates a struct with the given fields and default values
#[macro_export]
macro_rules! generate_mongodb_object {
    ($name:ident {$($field:ident: $type:ty),*}) => {
        /// Generate a struct with a given fields and default fields which are:
        /// - _id: mongodb object id
        /// - created_at: date time of the creation of the object
        /// - updated_at: date time of the last update of the object
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            pub _id: mongodb::bson::oid::ObjectId,
            pub created_at: mongodb::bson::DateTime,
            pub updated_at: mongodb::bson::DateTime,
            $(pub $field: $type),*
        }

        impl $name {
            /// This function is used to create a new object with the given fields,
            /// and fill the new _id, created_at, and updated_at
            pub fn new($($field: $type),*) -> Self {
                Self {
                    _id: mongodb::bson::oid::ObjectId::new(),
                    created_at: mongodb::bson::DateTime::now(),
                    updated_at: mongodb::bson::DateTime::now(),
                    $($field: $field),*
                }
            }

            /// This function is used to update the object with the given fields,
            /// and update the updated_at field
            pub fn update(&mut self, $(#[allow(unused_variables)] $field: Option<$type>),*) {
                self.updated_at = mongodb::bson::DateTime::now();
                $(
                    if let Some($field) = $field {
                        self.$field = $field;
                    }
                )*
            }
        }
    }
}