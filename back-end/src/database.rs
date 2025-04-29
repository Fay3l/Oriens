use actix_web::guard::Get;
use sqlx::postgres::{PgPool, PgPoolOptions};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use crate::{models::{GetUser, GetUserId, User}, verify_user};

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


    pub async fn create_user(&self, user: &User) -> Result<Uuid, sqlx::Error> {
        match self.exist_user(&user.email).await {
            Ok(true) => {
                println!("User already exists");
                Ok(Uuid::nil())
            },
            Ok(false)=>{
                let id  = Uuid::now_v7();
                let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
                println!("User does not exist, creating user");
                sqlx::query!(
                    r#"
                    INSERT INTO users (username, firstname, lastname, address, email, city, postalcode, number_phone, age, password, experience, badges,id,role)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12,$13,$14)
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
                    hashed_password,
                    user.experience as i32,
                    serde_json::to_value(&user.badges).unwrap(),
                    id,
                    user.role 
                )
                .execute(&self.db)
                .await?;
                Ok(id)
            }
            Err(e)=>{
                Err(e)
            }
        }
        
        
        
    }

    pub async fn verify_user(&self, lastname: &str, email:&str, password: &str) -> Result<GetUserId, sqlx::Error> {
        let user = sqlx::query!(
            r#"
            SELECT id,password FROM users WHERE lastname = $1 OR email = $2
            "#,
            lastname,
            email
        )
        .fetch_optional(&self.db)
        .await?;

        if let Some(user) = user {
            println!("User found{:?}", user);
            
            if verify(password, &user.password).unwrap() {
                println!("User connected");
                return Ok(GetUserId{id: user.id});
            }
        }
        Ok(GetUserId{id: Uuid::nil()})
    }

    pub async fn exist_user(&self, email: &str) -> Result<bool, sqlx::Error> {
        let user = sqlx::query!(
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.db)
        .await?; 
        Ok(user.is_some())
    }

    pub async fn load_users(&self) -> Result<Vec<GetUser>, sqlx::Error> {
        let users = sqlx::query_as!(
            GetUser,
            r#"
            SELECT * FROM users
            "#
        )
        .fetch_all(&self.db)
        .await?;
        Ok(users)
    }
    pub async fn load_user(&self, id: Uuid) -> anyhow::Result<GetUser> {
        let user = sqlx::query_as!(
            GetUser,
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(user)
    }
    pub async fn save_user(&self, user: &GetUser) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users SET firstname = $1, lastname = $2, address = $3, email = $4, city = $5, postalcode = $6, number_phone = $7, age = $8, password = $9, experience = $10, badges = $11 WHERE username = $12
            "#,
            user.firstname,
            user.lastname,
            user.address,
            user.email,
            user.city,
            user.postalcode ,
            user.number_phone,
            user.age as i32,
            user.password,
            user.experience as i32,
            serde_json::to_value(&user.badges).unwrap(),
            user.username
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn get_user_id(&self, email: &str) -> Result<GetUserId, sqlx::Error> {
        let user = sqlx::query!(
            r#"
            SELECT id FROM users WHERE email=$1
            "#,
            email,
        )
        .fetch_one(&self.db)
        .await?;
        Ok(GetUserId{id: user.id})
    }
}