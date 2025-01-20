use sqlx::postgres::{PgPool, PgPoolOptions};
pub struct DB{
    pub db: PgPool,
}
impl DB {
    pub async fn connect(db_url: &str) -> Result<Self, &'static str> {
        let db = PgPoolOptions::new().connect((db_url)).await.unwrap();
        Ok(Self {db})
    }
}