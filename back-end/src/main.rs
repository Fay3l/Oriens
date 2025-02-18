mod models;
mod api;
mod database;
use api::AppError;
use axum::{ Json, Router};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono;
use jsonwebtoken::{encode, EncodingKey, Header};
use mistralai_client::v1::chat::{ChatMessage, ChatMessageRole, ChatParams, ResponseFormat};
use mistralai_client::v1::client::Client;
use mistralai_client::v1::constants::Model;
use models::{AppState, Badge, Claims, Job, Jobs, MetiersPossibles, Question, Questionnaire, Section, User};
use quick_xml::de::from_str;
use std::fs::{ self, File, OpenOptions};
use std::io::{self, BufReader, Read, Seek, Write};
use std::sync::{Arc, RwLock};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let file_metiers: Jobs = from_str(
        &fs::read_to_string("Onisep_Ideo_Fiches_Metiers_09122024.xml").expect("Cannot read file")
    ).expect("Cannot deserialize file jobs");
    let url_db = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = database::DB::connect(&url_db)
        .await
        .expect("Cannot connect to database");
    let state = AppState {
        metiers: Arc::new(RwLock::new(file_metiers)),
        db
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    // build our application with a route
    let app =
        Router::new()
        .merge(api::api_routes())
        .layer(cors);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

pub fn add_user(user: User) -> api::Result<()> {
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



pub async fn jobs(query: &str) -> api::Result<Vec<Job>> {
    let file = tokio::fs::read_to_string("Onisep_Ideo_Fiches_Metiers_09122024.xml").await?;
    let res: Jobs = from_str(&file)?;
    let mut items = vec![];
    for metier in res.metiers {
        if metier.nom_metier.contains(query) {
            println!("{:?}", metier);
            items.push(metier);
        }
    }
    Ok(items)
}

pub fn verify_user(username: &str, password: &str) -> api::Result<bool> {
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

pub fn create_jwt(username: &str) -> api::Result<String> {
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

pub fn add_experience(user: &mut User, experience: u32) {
    user.experience += experience;
    check_badges(user);
}

pub fn check_badges(user: &mut User) {
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

pub fn save_user(user: &User) -> api::Result<()> {
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

pub fn load_user(username: &str) -> api::Result<Json<User>> {
    let mut file = File::open("data.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let users: Vec<User> = serde_json::from_str(&contents)?;

    for user in users {
        if user.username == username {
            return Ok(Json(user));
        }
    }

    Err(AppError(anyhow::anyhow!("User not found")))
}

pub async fn survey_result( data: Vec<Section>) -> api::Result<MetiersPossibles> {
    let api_key = std::env::var("API_MISTRAL_KEY").expect("API_MISTRAL_KEY must be set");
    
    let client = Client::new(Some(api_key.to_string()), None, None, None).unwrap();
    let model = Model::OpenMistral7b;
    println!("{:?}",api_key);
    let messages = vec![ChatMessage {
        role: ChatMessageRole::User,
        content:format!("Pour la personne qui a répondu au questionnaire, trouver 2-3 métiers qui correspondent aux réponses. Retourner les métiers et les descriptions en objet JSON.{:?}",data) ,
        tool_calls: None,

    }];
    let options = ChatParams {
        temperature: 0.7,
        random_seed: Some(42),
        response_format: Option::from(ResponseFormat::json_object()),
        ..Default::default()
    };

    let result = client.chat_async(model, messages, Some(options)).await.unwrap();
    println!("Assistant: {}", result.choices[0].message.content);
    let metiers_possibles: MetiersPossibles = serde_json::from_str(&result.choices[0].message.content).expect("JSON was not well-formatted");
    Ok(metiers_possibles)
}


