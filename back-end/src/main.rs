mod api;
mod database;
mod models;
use axum::Router;
use chrono;
use jsonwebtoken::{encode, EncodingKey, Header};
use mistralai_client::v1::chat::{ChatMessage, ChatMessageRole, ChatParams, ResponseFormat};
use mistralai_client::v1::client::Client;
use mistralai_client::v1::constants::Model;
use models::{AppState, Badge, Claims, GoogleAuth, Job, Jobs, MetiersPossibles, Section, User};
use quick_xml::de::from_str;
use std::fs::{self};
use std::sync::{Arc, RwLock};
use tower_http::cors::{Any, CorsLayer};

use crate::models::{Questionnaire, ResponseQuiz};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let file_metiers: Jobs = from_str(
        &fs::read_to_string("Onisep_Ideo_Fiches_Metiers_09122024.xml").expect("Cannot read file"),
    )
    .expect("Cannot deserialize file jobs");
    let google_auth: GoogleAuth =
        serde_json::from_str(&fs::read_to_string("google.json").expect("Cannot read google.json"))
            .expect("Cannot deserialize google.json");
    let url_db = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = database::DB::connect(&url_db)
        .await
        .expect("Cannot connect to database");
    let state = AppState {
        metiers: Arc::new(RwLock::new(file_metiers)),
        db,
        google_auth: Arc::new(RwLock::new(google_auth)),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    // build our application with a route
    let app = Router::new()
        .merge(api::api_routes())
        .layer(cors)
        .with_state(state);

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

pub async fn jobs_search(query: &str) -> api::Result<Vec<Job>> {
    let file = tokio::fs::read_to_string("Onisep_Ideo_Fiches_Metiers_09122024.xml").await?;
    let res: Jobs = from_str(&file)?; 
    let mut items = vec![];
    for job in res.metiers.iter() {
        if job.nom_metier.contains(query) {
            println!("{:?}", job);
            items.push(job.clone());
            if items.len() >= 16{
                break; 
            }
        }
    }
    Ok(items)
}

pub fn create_jwt(id: uuid::Uuid) -> api::Result<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: id.to_owned().to_string(),
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

pub fn mistral_chat(content: String) -> api::Result<mistralai_client::v1::chat::ChatResponse> {
    let api_key = std::env::var("API_MISTRAL_KEY").expect("API_MISTRAL_KEY must be set");
    let client = Client::new(Some(api_key.to_string()), None, None, None).unwrap();
    let model = Model::OpenMistral7b;
    let messages = vec![ChatMessage {
        role: ChatMessageRole::User,
        content: content,
        tool_calls: None,
    }];
    let options = ChatParams {
        temperature: 0.7,
        random_seed: Some(42),
        response_format: Option::from(ResponseFormat::json_object()),
        ..Default::default()
    };
    let result = client.chat(model, messages, Some(options))?;
    Ok(result)
}

pub async fn questionnaire_result(data: Questionnaire) -> api::Result<ResponseQuiz> {
    let object_json = r#"
    {
        {
            "adjectif": "string",
            "description": "string",
            "formations": ["string"],
            "metiers": ["string"],
            "softskills": ["string"]
        }
    }
    "#;
    let content = format!("Pour la personne qui a répondu au questionnaire, trouver l'adjectif sur les compétences trouvés avec la description ensuite les formations suggérées, les métiers compatibles et les softs skills clés qui correspondent aux réponses. Retourner les métiers et les descriptions en objet JSON. Doit retourner {:?}. {:?}",object_json,data);
    let result = tokio::task::spawn_blocking(|| mistral_chat(content)).await??;
    let response: ResponseQuiz = serde_json::from_str(&result.choices[0].message.content)
        .expect("JSON was not well-formatted");

    Ok(response)
}
