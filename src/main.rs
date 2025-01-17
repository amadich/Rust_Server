// src/main.rs
mod database;
mod models;

use warp::Filter;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Utc, Duration};
use mongodb::Collection;
use std::convert::Infallible;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    email: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    token: String,
}

// JWT secret key (replace with a secure key in production)
const JWT_SECRET: &[u8] = b"your_secret_key";

// Register a new user
async fn register_user(
    user: RegisterRequest,
    collection: Collection<models::User>,
) -> Result<impl warp::Reply, Infallible> {
    // Hash the password
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();

    // Create a new user
    let new_user = models::User {
        email: user.email.clone(),
        password: hashed_password,
    };

    // Insert the user into MongoDB
    collection.insert_one(new_user, None).await.unwrap();

    // Generate a JWT
    let expiration = Utc::now() + Duration::hours(24);
    let claims = Claims {
        email: user.email,
        exp: expiration.timestamp() as usize,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap();

    // Return the token
    Ok(warp::reply::json(&RegisterResponse { token }))
}

#[tokio::main]
async fn main() {
    // Connect to MongoDB
    let collection = database::get_collection().await;

    // Define the register route
    let register = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |user: RegisterRequest| {
            let collection = collection.clone();
            async move { register_user(user, collection).await }
        });

    // Start the server
    warp::serve(register).run(([127, 0, 0, 1], 3030)).await;
}