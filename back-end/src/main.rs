mod models;
use actix_web::{HttpRequest, HttpResponse, Responder};
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::{from_fn, Next};
use axum::response::Response;
use axum::routing::post;
use axum::Error;
use axum::{response::Html, routing::get, Json, Router};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use models::{Badge, Claims, FormatCourt, Metier, User, UserLogin};
use quick_xml::{events::Event,reader::Reader};
use serde_json::{json, to_string};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, Read, Seek, Write};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    // build our application with a route
    let app = Router::new()
        .route("/api", get(handler).layer(from_fn(validate_token)))
        .route("/api/register", post(register_user))
        .route("/api/login", post(login_user))
        .route("/api/jobs", get(jobs_handler))
        .layer(cors);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn validate_token(
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let auth_header = req.headers().get("Authorization");

    if let Some(auth_header) = auth_header {
        let token = auth_header.to_str().unwrap().replace("Bearer ", "");
        let validation = Validation::default();
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &validation,
        ) {
            Ok(_) => return Ok(next.run(req).await),
            Err(_) => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Invalid token"})),
                ))
            }
        }
    }
    Err((
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": "Missing token"})),
    ))
}

async fn register_user(Json(payload): Json<User>) -> Json<serde_json::Value> {
    match add_user(payload) {
        Ok(_) => Json(json!({"message": "User registered successfully"})),
        Err(e) => Json(json!({"error": format!("{}", e)})),
    }
}

async fn login_user(Json(payload): Json<UserLogin>) -> Json<serde_json::Value> {
    println!("{:?}", payload);
    match verify_user(&payload.username, &payload.password) {
        Ok(true) => {
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
        }
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

fn verify_user(username: &str, password: &str) -> Result<bool, Box<dyn std::error::Error>> {
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

async fn jobs_handler() -> Json<Vec<Metier>> {
    let mut xml_reader = Reader::from_file("Onisep_Ideo_Fiches_Metiers_09122024.xml").expect("Cannot read the file");
    xml_reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut metiers = Vec::new();
    let mut current_metier = None;
    let mut current_format_court = None;
    let mut current_text = String::new();

    loop {
        match xml_reader.read_event_into(&mut buf) {
            
            Err(e) => panic!(
                "Error at position {:?}",e
            ),
            Ok(Event::Eof) => break,
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"metier" => {
                        current_metier = Some(Metier {
                            nom_metier: String::new(),
                            acces_metier: String::new(),
                            competences: String::new(),
                            condition_travail: String::new(),
                            formats_courts: None,
                        });
                    }
                    b"format_court" => {
                        current_format_court = Some(FormatCourt {
                            r#type: String::new(),
                            libelle: String::new(),
                            descriptif: String::new(),
                        });
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                match e.name().as_ref() {
                    b"metier" => {
                        if let Some(metier) = current_metier.take() {
                            metiers.push(metier);
                        }
                    }
                    b"format_court" => {
                        if let Some(mut metier) = current_metier.as_mut() {
                            if let Some(format_court) = current_format_court.take() {
                                if metier.formats_courts.is_none() {
                                    metier.formats_courts = Some(Vec::new());
                                }
                                metier.formats_courts.as_mut().unwrap().push(format_court);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                if let Some(ref mut metier) = current_metier {
                    match xml_reader.element_name() {
                        b"nom_metier" => metier.nom_metier = e.unescape().unwrap().to_string(),
                        b"acces_metier" => metier.acces_metier = e.unescape().unwrap().to_string(),
                        b"competences" => metier.competences = e.unescape().unwrap().to_string(),
                        b"condition_travail" => metier.condition_travail = e.unescape().unwrap().to_string(),
                        _ => {}
                    }
                }
                if let Some(ref mut format_court) = current_format_court {
                    match xml_reader. {
                        b"type" => format_court.r#type = e.unescape().unwrap().to_string(),
                        b"libelle" => format_court.libelle = e.unescape().unwrap().to_string(),
                        b"descriptif" => format_court.descriptif = e.unescape().unwrap().to_string(),
                        _ => {}
                    }
                }
            }

            _ => {}
        }
        buf.clear();
    }
    for metier in &metiers {
        println!("{:?}", metier);
    }
    Json(metiers)
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
