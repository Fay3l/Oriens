mod models;
mod database;
use axum::extract::{Query, Request};
use axum::http::StatusCode;
use axum::middleware::{from_fn, Next};
use axum::response::{IntoResponse, Response};
use axum::{response::Html, routing::{get,post}, Json, Router};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use models::{AppState, Badge, Claims, Metier, Metiers, SearchQuery, User, UserLogin};
use quick_xml::de::from_str;
use serde_json::json;
use std::fs::{ self, File, OpenOptions};
use std::io::{self, BufReader, Read, Seek, Write};
use std::sync::{Arc, RwLock};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    
    let file_metiers: Metiers = from_str(
        &fs::read_to_string("Onisep_Ideo_Fiches_Metiers_09122024.xml").expect("Cannot read file")
    ).expect("Cannot deserialize file jobs");

    let state = AppState {
        metiers: Arc::new(RwLock::new(file_metiers))
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    // build our application with a route
    let app = Router::new()
        .route("/api", get(handler).layer(from_fn(validate_token)))
        .route("/api/register", post(register_user))
        .route("/api/login", post(login_user))
        .route("/api/jobs/search", get(jobssearch_handler))
        .layer(cors);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


type Result<T> = std::result::Result<T, AppError>;

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}


async fn validate_token(
    req: Request,
    next: Next,
) -> Result<Response> {
    let auth_header = req.headers().get("Authorization");
    if let Some(auth_header) = auth_header {
        let token = auth_header.to_str().unwrap().replace("Bearer ", "");
        let validation = Validation::default();
        let _decode=  decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &validation,
        )?;
       
    }
    Ok(next.run(req).await)
}

async fn register_user(Json(payload): Json<User>) -> Result<Json<serde_json::Value>> {
    let _ = add_user(payload)?;
    Ok(Json(json!({"message": "User registered successfully"})))
}

async fn login_user(Json(payload): Json<UserLogin>) -> Json<serde_json::Value> {
    println!("{:?}", payload);
    match verify_user(&payload.username, &payload.password) {
        Ok(true) => {
            let mut user = match load_user(&payload.username) {
                Ok(user) => user,
                Err(e) => return Json(json!({})),
            };
            println!("Add experience");
            // Add experience
            add_experience(&mut user, 10);
            println!("Upload user");
            // Save the updated user
            if let Err(e) = save_user(&user) {
                return Json(json!({}));
            }
            println!("Create Token ");
            match create_jwt(&payload.username) {
                Ok(token) => Json(json!({"token": token})),
                Err(_) => Json(json!({})),
            }
        }
        Ok(false) => Json(json!({"error": "Invalid credentials"})),
        Err(_) => Json(json!({})),
    }
}

fn add_user(user: User) -> Result<()> {
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

async fn jobssearch_handler(
    Query(search): Query<SearchQuery>,
) -> Result<Json<Vec<Metier>>> {
    let res = jobs(&search.search).await?;
    Ok(Json(res))
}

async fn jobs(query: &str) -> Result<Vec<Metier>> {
    let file = tokio::fs::read_to_string("Onisep_Ideo_Fiches_Metiers_09122024.xml").await?;
    let res: Metiers = from_str(&file)?;
    let mut items = vec![];
    for metier in res.metiers {
        if metier.nom_metier.contains(query) {
            println!("{:?}", metier);
            items.push(metier);
        }
    }
    Ok(items)
}

fn verify_user(username: &str, password: &str) -> Result<bool> {
    let mut file = File::open("data.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{:?}", contents);
    let users: Vec<User> = serde_json::from_str(&contents)?;
    println!("{:#?}", users);
    for item in users {
        if item.username == username && verify(password, &item.password)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn create_jwt(username: &str) -> Result<String> {
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
        Badge {
            name: "Novice".to_string(),
            required_experience: 10,
        },
        Badge {
            name: "Intermediate".to_string(),
            required_experience: 50,
        },
        Badge {
            name: "Expert".to_string(),
            required_experience: 100,
        },
    ];

    for badge in badges {
        if user.experience >= badge.required_experience && !user.badges.contains(&badge.name) {
            user.badges.push(badge.name);
        }
    }
}

fn save_user(user: &User) -> Result<()> {
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

fn load_user(username: &str) -> Result<Json<User>> {
    let mut file = File::open("data.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut res_user: User ;
    let users: Vec<User> = serde_json::from_str(&contents)?;
    for user in users {
        if user.username == username {
           res_user = user ;
        }
    }
    Ok(Json(res_user))
    
    

}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
