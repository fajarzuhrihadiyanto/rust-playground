use mongodb::{options::ClientOptions, Client, Collection, bson::Document};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub async fn get_collection<T: Serialize + Send + Sync>(collection_name: String) -> Collection<T> {
    let mut client_options = ClientOptions::parse("mongodb://quantum:oiZZ8pRQPaGKkidAxnv0@192.168.166.6:27017/?replicaSet=rs0").await.unwrap();
    client_options.app_name = Some("RustMongoCRUD".to_string());
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("rust_playground");
    db.collection(collection_name.as_str())
}

pub async fn insert<T: Serialize + Send + Sync>(collection_name: String, data: T) {
    let collection: Collection<T> = get_collection(collection_name).await;

    let result = collection.insert_one(data).await.unwrap();
    println!("Result: {:?}", result);
}

pub async fn find_all<T: Serialize + for<'de> Deserialize<'de> + Debug + Send + Sync>(collection_name: String) {
    let collection: Collection<T> = get_collection(collection_name).await;

    let result = collection.find(Document::new()).await;
    result.iter().for_each(|result| {
        println!("Result: {:?}", result.deserialize_current().unwrap());
    });
}