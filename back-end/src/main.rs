mod models;
use axum::http;
use axum::http::Method;
use axum::routing::post;
use axum::Json;
use axum::{response::Html, routing::get, Router};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono;
use jsonwebtoken::decode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::{encode, EncodingKey, Header};
use models::{Claims, User, UserLogin};
use serde_json::json;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([http::header::CONTENT_TYPE]);
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .layer(cors);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn register_user(Json(payload): Json<User>) -> Json<serde_json::Value> {
    match add_user(payload) {
        Ok(_) => Json(json!({"message": "User registered successfully"})),
        Err(e) => Json(json!({"error": format!("{}", e)})),
    }
}

async fn login_user(Json(payload): Json<UserLogin>) -> Json<serde_json::Value> {
    println!("{:?}",payload);
    match verify_user(&payload.username,&payload.password) {
        Ok(true) => match create_jwt(&payload.username) {
            Ok(token) => Json(json!({"token": token})),
            Err(e) => Json(json!({"error true": format!("{}", e)})),
        },
        Ok(false) => Json(json!({"error": "Invalid credentials"})),
        Err(e) => Json(json!({"error false": format!("{}", e)})),
    }
}

fn add_user(user: User) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("data.json")?;

    let hashed_password = hash(&user.password, DEFAULT_COST)?;
    let user_with_hashed_password = User {
        password: hashed_password,
        ..user
    };

    let user_json = serde_json::to_string(&user_with_hashed_password)?;
    writeln!(file, "{}", user_json)?;

    Ok(())
}

fn verify_user(username: &str,password:&str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut file = File::open("data.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{:?}",contents);
    let users:Vec<User>= serde_json::from_str(&contents)?;
    println!("{:#?}", users);
    for item in users {
        if item.username == username && verify(password, &item.password)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn create_jwt(username: &str) -> Result<String, Box<dyn std::error::Error>> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;

    Ok(token)
}

fn validate_jwt(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &validation,
    )?;

    Ok(token_data.claims)
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
