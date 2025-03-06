use std::sync::{Arc, RwLock};

use mistralai_client::v1::client::Client;
use serde::{Deserialize, Serialize};

use crate::database;



#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct User {
    pub username: String,
    pub name: String,
    pub surname: String,
    pub address: String,
    pub age: u32,
    pub password: String,
    pub experience: u32,
    pub badges: Vec<String>,
}

#[derive(Serialize, Clone,  Deserialize, Debug)]
pub struct UserLogin {
    pub username: String,
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
#[derive(Debug, Deserialize, Serialize)]
pub struct Job {
    pub identifiant: String,
    pub nom_metier: String,
    pub acces_metier: String
}
#[derive(Debug, Deserialize, Serialize)]
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
pub struct Section{
    pub title: String,
    pub questions: Vec<Question>,
}
#[derive(Debug,Clone)]
pub struct Survey{
    pub sections: Vec<Section>,
}

#[derive(Debug,Clone)]
pub struct AppState{
    pub metiers : Arc<RwLock<Jobs>>,
    pub db: database::DB
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Questionnaire {
    pub sections: Vec<Section>,
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

