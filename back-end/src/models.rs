use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::database::DB;


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
pub struct Metier {
    pub identifiant: String,
    pub nom_metier: String,
    pub acces_metier: String
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Metiers {
    #[serde(rename = "metier")]
    pub metiers: Vec<Metier>,
}

#[derive(Deserialize,Debug,Serialize)]
pub struct SearchQuery {
    pub search: String,
}

#[derive(Clone)]
pub struct AppState{
    pub metiers : Arc<RwLock<Metiers>>,
    // pub db: DB
}