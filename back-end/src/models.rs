use serde::{Deserialize, Serialize};


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
#[derive(Serialize,Clone, Deserialize, Debug)]
pub struct FormatCourt {
    pub r#type: String,
    pub libelle: String,
    pub descriptif: String,
}
#[derive(Serialize,Clone, Deserialize, Debug)]
pub struct Metier {
    pub nom_metier: String,
    pub acces_metier: String,
    pub competences: String,
    pub condition_travail: String,
    pub formats_courts: Option<Vec<FormatCourt>>,
}