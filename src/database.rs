// src/database.rs
use mongodb::{Client, Collection};
use crate::models::User;

const DATABASE_NAME: &str = "rust_server";
const COLLECTION_NAME: &str = "users";

pub async fn get_collection() -> Collection<User> {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("Failed to connect to MongoDB");
    let database = client.database(DATABASE_NAME);
    database.collection(COLLECTION_NAME)
}