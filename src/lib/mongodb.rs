use mongodb::{options::ClientOptions, Client, Collection, bson::Document, Database, ClientSession};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// This function is used to get the client of the mongodb
pub async fn get_client() -> Client {
    let mut client_options = ClientOptions::parse("mongodb://quantum:oiZZ8pRQPaGKkidAxnv0@192.168.166.6:27017/?replicaSet=rs0").await.unwrap();
    client_options.app_name = Some("RustMongoCRUD".to_string());
    Client::with_options(client_options).unwrap()
}

/// This function is used to get the database of the mongodb
pub async fn get_db(client: Option<Client>) -> Database {
    let c = match client {
        Some(c) => c,
        None => get_client().await
    };
    c.database("rust_playground")
}

/// This function is used to get the session of the mongodb
pub async fn get_session(client: Option<Client>) -> ClientSession {
    let c = match client {
        Some(c) => c,
        None => get_client().await
    };
    c.start_session().await.unwrap()
}

/// This function is used to insert a new object into the mongodb
pub async fn insert<T: Serialize + Send + Sync>(collection_name: String, data: T) {
    let client = get_client().await;
    let db = get_db(Some(client)).await;
    let collection: Collection<T> = db.collection(collection_name.as_str());

    let result = collection.insert_one(data).await.unwrap();
    println!("Result: {:?}", result);
}

/// This function is used to find all the objects in the mongodb
pub async fn find_all<T: Serialize + for<'de> Deserialize<'de> + Debug + Send + Sync>(collection_name: String) {
    
    let client = get_client().await;
    let db = get_db(Some(client)).await;
    let collection: Collection<T> = db.collection(collection_name.as_str());

    let result = collection.find(Document::new()).await;
    result.iter().for_each(|result| {
        println!("Result: {:?}", result.deserialize_current().unwrap());
    });
}