use sqlx::postgres::{PgPool, PgPoolOptions};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use crate::models::{GetUser, GetUserId, PasswordResetToken, User};

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

    pub async fn store_password_reset_token(
        &self,
        user_id: &Uuid,
        hashed_token: &str,
        expiry: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO password_reset_tokens (user_id, token, token_expiry)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, token) DO UPDATE SET token_expiry = $3
            "#,
            user_id,
            hashed_token,
            expiry
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

     pub async fn get_password_reset_token(&self, hashed_token: &str) -> Result<Option<PasswordResetToken>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT user_id, token, token_expiry
            FROM password_reset_tokens
            WHERE token = $1
            "#,
            hashed_token
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(row.map(|r| PasswordResetToken {
            user_id: r.user_id,
            token: r.token,
            token_expiry: r.token_expiry,
        }))
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

    pub async fn delete_all_password_reset_tokens(&self, user_id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM password_reset_tokens WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub async fn update_user_password(&self, user_id: Uuid, new_hashed_password: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users SET password = $1 WHERE id = $2
            "#,
            new_hashed_password,
            user_id
        )
        .execute(&self.db)
        .await?;
        Ok(())
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
            SELECT id,firstname,lastname,postalcode,city,address,email,number_phone,age,role,experience,badges,username FROM users
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
            SELECT id,firstname,lastname,postalcode,city,address,email,number_phone,age,role,experience,badges,username FROM users WHERE id = $1
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
            UPDATE users SET firstname = $1, lastname = $2, address = $3, email = $4, city = $5, postalcode = $6, number_phone = $7, age = $8, experience = $9, badges = $10 WHERE username = $11
            "#,
            user.firstname,
            user.lastname,
            user.address,
            user.email,
            user.city,
            user.postalcode ,
            user.number_phone,
            user.age as i32,
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
    
    // pub async fn get_user_by_id(&self, id: Uuid) -> Result<GetUser, sqlx::Error> {
    //     let user = sqlx::query_as!(
    //         GetUser,
    //         r#"
    //         SELECT id,firstname,lastname,postalcode,city,address,email,number_phone,age,role,experience,badges,username FROM users WHERE id = $1
    //         "#,
    //         id
    //     )
    //     .fetch_one(&self.db)
    //     .await?;
    //     Ok(user)
    // }
}