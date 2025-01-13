use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub name: String,
    pub surname: String,
    pub address: String,
    pub age: u32,
    pub password: String,
}

#[derive(Serialize, Clone,  Deserialize, Debug)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}