use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::models::User;

#[derive(Debug,Clone)]
pub struct DB{
    pub db: PgPool,
}
impl DB {
    pub async fn connect(db_url: &str) -> Result<Self, sqlx::Error> {
        let db = PgPoolOptions::new().connect(db_url).await?;
        println!("Connected to database");
        sqlx::migrate!("./migrations").run(&db).await?;
        println!("Migrations applied");
        Ok(Self {db})
    }
    pub async fn create_user(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO users (username, firstname, lastname, address, email, city, postalcode, number_phone, age, password, experience, badges)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (email) DO NOTHING
            "#,
            user.username,
            user.firstname,
            user.lastname,
            user.address,
            user.email,
            user.city,
            user.postalcode as i32,
            user.number_phone,
            user.age as i32,
            user.password,
            user.experience as i32,
            serde_json::to_value(&user.badges).unwrap()
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }
}