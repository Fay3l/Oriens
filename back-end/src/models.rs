use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use crate::database;

#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct GoogleAuth {
    pub client_id: String,
    pub project_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_secret:String,
    pub redirect_uris:Vec<String>,
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}
#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct User {
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub address: String,
    pub email: String,
    pub city:String,
    pub postalcode:u32,
    pub number_phone:String,
    pub age: u8,
    pub password: String,
    pub experience: u32,
    pub role: String,
    pub badges: Vec<String>,
}

#[derive(Serialize,Deserialize,Clone, Debug)]
pub struct GetUser {

    pub id: uuid::Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub address: Option<String>,
    pub email: String,
    pub city:Option<String>,
    pub postalcode:Option<i32>,
    pub number_phone:Option<String>,
    pub age: i32,
    pub password: String,
    pub experience: i32,
    pub role:String,
    pub badges: serde_json::Value,
}

#[derive(Serialize,Deserialize,Clone, Debug)]
pub struct GetUserId {
    pub id: uuid::Uuid,
}

#[derive(Serialize, Clone,  Deserialize, Debug)]
pub struct UserLogin {
    pub email: String,
    pub lastname:String,
    pub firstname:String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Badge {
    pub name: String,
    pub required_experience: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
#[derive(Debug,Clone, Deserialize, Serialize)]
pub struct Job {
    pub identifiant: String,
    pub nom_metier: String,
    pub acces_metier: String,
    pub competences: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Jobs {
    #[serde(rename = "metier")]
    pub metiers: Vec<Job>,
}

#[derive(Deserialize,Debug,Serialize)]
pub struct SearchQuery {
    pub search: String,
}

#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct Question{
    pub question: String,
    pub response: String,
    pub options: Vec<String>,
}

#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct JobsPagination{
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct Section{
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Debug,Clone)]
pub struct AppState{
    pub metiers : Arc<RwLock<Jobs>>,
    pub db: database::DB,
    pub google_auth: Arc<RwLock<GoogleAuth>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Questionnaire {
    pub sections: Vec<Section>,
}

#[derive(Deserialize, Debug,Serialize,Clone)]
pub struct OAuthCallback {
    pub code: String,
    pub scope: String,
    pub authuser: String,
    pub prompt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metier {
    pub nom_metier: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetiersPossibles {
    pub metiers_possibles: Vec<Metier>,
}

