mod models;
use actix_web::{HttpRequest, HttpResponse, Responder};
use axum::extract::Request;
use axum::http::{self, StatusCode};
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
use models::{Badge, Claims, User, UserLogin};
use serde_json::json;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufReader, Read, Seek, Write};
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
        .route("/api", get(handler))
        .route("/api/register", post(register_user))
        .route("/api/login", post(login_user))
        .layer(cors);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn protected_handler(req: HttpRequest) -> impl Responder {
    let token = req.headers().get("Authorization").and_then(|value| value.to_str().ok()).unwrap_or("");

    match validate_jwt(token) {
        Ok(claims) => {
            // Utiliser les claims pour autoriser ou refuser l'accès
            HttpResponse::Ok().body(format!("Welcome, user with ID: {}", claims.sub))
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid token"),
    }
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
        Ok(true) =>{
            let mut user = match load_user(&payload.username) {
                Ok(user) => user,
                Err(e) => return Json(json!({"error": format!("{}", e)})),
            };
            println!("Add experience");
            // Add experience
            add_experience(&mut user, 10);
            println!("Upload user");
            // Save the updated user
            if let Err(e) = save_user(&user) {
                return Json(json!({"error": format!("{}", e)}));
            }
            println!("Create Token ");
            match create_jwt(&payload.username) {
                Ok(token) => Json(json!({"token": token})),
                Err(e) => Json(json!({"error true": format!("{}", e)})),
            }
        } ,
        Ok(false) => Json(json!({"error": "Invalid credentials"})),
        Err(e) => Json(json!({"error false": format!("{}", e)})),
    }
}

fn add_user(user: User) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("data.json")?;
    
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut users: Vec<User> = if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents)?
    };

    let hashed_password = hash(&user.password, DEFAULT_COST)?;
    let user_with_hashed_password = User {
        password: hashed_password,
        ..user
    };

    users.push(user_with_hashed_password);

    // Sérialiser la liste mise à jour en JSON
    let users_json = serde_json::to_string_pretty(&users)?;

    // Réécrire le fichier avec la liste mise à jour
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data.json")?;
    file.write_all(users_json.as_bytes())?;

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

fn add_experience(user: &mut User, experience: u32) {
    user.experience += experience;
    check_badges(user);
}

fn check_badges(user: &mut User) {
    let badges = vec![
        Badge { name: "Novice".to_string(), required_experience: 10 },
        Badge { name: "Intermediate".to_string(), required_experience: 50 },
        Badge { name: "Expert".to_string(), required_experience: 100 },
    ];

    for badge in badges {
        if user.experience >= badge.required_experience && !user.badges.contains(&badge.name) {
            user.badges.push(badge.name);
        }
    }
}

fn save_user(user: &User) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("data.json")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut users: Vec<User> = if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents)?
    };

    // Trouver l'index de l'utilisateur à mettre à jour
    if let Some(index) = users.iter().position(|u| u.username == user.username) {
        users[index] = user.clone();
    } else {
        users.push(user.clone());
    }

    let users_json = serde_json::to_string_pretty(&users)?;
    file.set_len(0)?; // Truncate the file
    file.seek(io::SeekFrom::Start(0))?; // Go to the start of the file
    file.write_all(users_json.as_bytes())?;

    Ok(())
}

fn load_user(username: &str) -> Result<User, Box<dyn std::error::Error>> {
    let mut file = File::open("data.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let users: Vec<User> = serde_json::from_str(&contents)?;
    for user in users {
        if user.username == username {
            return Ok(user);
        }
    }

    Err("User not found".into())
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
