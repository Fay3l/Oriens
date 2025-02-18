use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Debug,Clone)]
pub struct DB{
    pub db: PgPool,
}
impl DB {
    pub async fn connect(db_url: &str) -> Result<Self, sqlx::Error> {
        let db = PgPoolOptions::new().connect(db_url).await?;
        println!("Connected to database");
        Ok(Self {db})
    }
}