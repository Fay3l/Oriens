use crate::{
    add_experience, create_jwt, jobs_search,
    models::{
        AppState, Claims, ForgotPasswordRequest, Job, JobsPagination, MetiersPossibles, OAuthCallback, ResetPasswordRequest, SearchQuery, Section, User, UserLogin
    },
    questionnaire_result,
};
use axum::{
    extract::{Query, Request, State},
    http::StatusCode,
    middleware::{from_fn, Next},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use base64::Engine;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, Validation};
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use rand::RngCore;
use serde_json::json;
use sha2::{Digest, Sha256};

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/api", get(handler).layer(from_fn(validate_token)))
        .route("/api/register", post(register_user))
        .route("/api/login", post(login_user))
        // .route("/api/jobs", get(jobs_handler))
        .route("/api/jobs", get(jobs_pagination_handler))
        .route(
            "/api/jobs/search",
            get(jobssearch_handler).layer(from_fn(validate_token)),
        )
        .route(
            "/api/survey/result",
            post(survey_handler).layer(from_fn(validate_token)),
        )
        .route("/api/auth/google", get(start_google_auth))
        .route("/api/auth/google/callback", get(handle_google_callback))
        .route("/api/forgot-password", post(forgot_password_handler))
        .route("/api/reset-password", post(reset_password_handler))
}

pub type Result<T> = std::result::Result<T, AppError>;

// Make our own error that wraps `anyhow::Error`.
#[derive(Debug)]
pub struct AppError(pub anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

async fn validate_token(req: Request, next: Next) -> Result<Response> {
    let auth_header = req.headers().get("Authorization");

    if let Some(auth_header) = auth_header {
        let token = auth_header.to_str().unwrap().replace("Bearer ", "");
        let validation = Validation::default();
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &validation,
        ) {
            Ok(_) => return Ok(next.run(req).await),
            Err(_) => {
                return Err(AppError(anyhow::anyhow!(json!({
                    "status": StatusCode::UNAUTHORIZED.as_u16(),
                    "error": "Invalid token"
                })
                .to_string())))
            }
        }
    }
    Err(AppError(anyhow::anyhow!(json!({
        "status": StatusCode::UNAUTHORIZED.as_u16(),
        "error": "Missing token"
    })
    .to_string())))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<UserLogin>,
) -> Json<serde_json::Value> {
    println!("{:?}", payload);
    match state
        .db
        .verify_user(&payload.lastname, &payload.email, &payload.password)
        .await
    {
        Ok(user_id) => {
            if user_id.id != uuid::Uuid::nil() {
                match state.db.load_user(user_id.id).await {
                    Ok(user) => {
                        let mut u = User {
                            username: user.username,
                            firstname: user.firstname,
                            lastname: user.lastname,
                            address: user.address.unwrap(),
                            email: user.email,
                            city: user.city.unwrap(),
                            postalcode: user.postalcode.unwrap() as u32,
                            number_phone: user.number_phone.unwrap(),
                            age: user.age as u8,
                            password: payload.password.clone(),
                            role: user.role,
                            experience: user.experience as u32,
                            badges: user
                                .badges
                                .as_array()
                                .unwrap_or(&vec![])
                                .iter()
                                .filter_map(|badge| badge.as_str().map(String::from))
                                .collect(),
                        };
                        add_experience(&mut u, 10);
                        let token = create_jwt(user_id.id).unwrap();
                        return Json(json!({"token": token, "id":user_id.id}));
                    }
                    Err(e) => {
                        return Json(
                            json!({"message": format!("{:?}",e)  , "code":StatusCode::INTERNAL_SERVER_ERROR.as_u16()}),
                        )
                    }
                }
            } else {
                return Json(json!({"error": "Invalid credentials"}));
            }
        }
        Err(e) => {
            return Json(
                json!({"message": format!("{:?}",e)  , "code":StatusCode::INTERNAL_SERVER_ERROR.as_u16()}),
            )
        }
    }

    // match verify_user(&payload.username, &payload.password) {
    //     Ok(true) => {
    //         let mut user = match load_user(&payload.username) {
    //             Ok(user) => user,
    //             Err(e) => return Json(json!({"message": format!("{:?}",e.0)  , "code":StatusCode::INTERNAL_SERVER_ERROR.as_u16()})),
    //         };
    //         println!("Add experience");
    //         // Add experience
    //         add_experience(&mut user, 10);
    //         println!("Upload user");
    //         // Save the updated user
    //         if let Err(e) = save_user(&user) {
    //             return Json(json!({"message": format!("{:?}",e.0)  , "code":StatusCode::INTERNAL_SERVER_ERROR.as_u16()}));
    //         }
    //         println!("Create Token ");
    //         match create_jwt(&payload.username) {
    //             Ok(token) => Json(json!({"token": token})),
    //             Err(e) => Json(json!({"message": format!("{:?}",e.0)  , "code":StatusCode::INTERNAL_SERVER_ERROR.as_u16()})),
    //         }
    //     }
    //     Ok(false) => Json(json!({"error": "Invalid credentials"})),
    //     Err(e) => Json(json!({"message": format!("{:?}",e.0)  , "code":StatusCode::INTERNAL_SERVER_ERROR.as_u16()})),
    // }
}

async fn survey_handler(Json(survey): Json<Vec<Section>>) -> Result<Json<MetiersPossibles>> {
    let res = questionnaire_result(survey).await?;
    Ok(Json(res))
}

async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> Result<Json<serde_json::Value>> {
    let _ = state.db.create_user(&payload).await?;
    Ok(Json(json!({"message": "User registered successfully"})))
}
#[axum::debug_handler]
async fn forgot_password_handler(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<serde_json::Value>> {
    // Toujours répondre pareil pour éviter l'énumération d'emails
    let msg = "Si cet email existe, un lien de réinitialisation a été envoyé.";

    // Vérifier si l'utilisateur existe
    let user = state.db.get_user_id(&payload.email).await?;
    if user.id == uuid::Uuid::nil() {
        return Ok(Json(json!({ "message": msg })));
    }

    // Générer un token sécurisé
    let mut token_bytes = [0u8; 64];
    rand::rng().fill_bytes(&mut token_bytes);
    let token = base64::engine::general_purpose::STANDARD.encode(token_bytes);
    // Hasher le token
    let mut hasher = Sha256::new();
    hasher.update(&token);
    let hashed_token = format!("{:x}", hasher.finalize());

    // Expiration dans 1h
    let expiry = Utc::now().timestamp() + 3600;

    // Stocker en base
    state
        .db
        .store_password_reset_token(&user.id, &hashed_token, expiry)
        .await?;
    let reset_link = format!("http://localhost:3000/reset-password?token={}", token);

    // Prépare le message
    let email = Message::builder()
        .from("fayel@oriens.com".parse().unwrap())
        .to(payload.email.parse().unwrap())
        .subject("Réinitialisation de votre mot de passe")
        .body(format!(
            "Bonjour,\n\nPour réinitialiser votre mot de passe, cliquez sur ce lien : {}\n\nCe lien expirera dans 1 heure.",
            reset_link
        ))
        .unwrap();

    // Configure le transport SMTP (exemple avec Gmail, adapte selon ton fournisseur)
    let creds = Credentials::new("fa9ba6bd2ea136".to_string(), "e8f8a64a9bef9b".to_string());
    let mailer = SmtpTransport::starttls_relay("sandbox.smtp.mailtrap.io")
        .unwrap()
        .credentials(creds)
        .build();

    // Envoie l'email (tu peux rendre ça async avec lettre_async si besoin)
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }

    // Envoyer le lien par email (à implémenter)
    // let reset_link = format!("https://example.com/reset-password?token={}", token);
    // send_email(payload.email, reset_link).await?;

    Ok(Json(json!({ "message": msg })))
}

async fn reset_password_handler(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<serde_json::Value>> {
    // Hasher le token reçu
    let mut hasher = Sha256::new();
    hasher.update(&payload.token);
    let hashed_token = format!("{:x}", hasher.finalize());

    // Chercher le token en base
    if let Some(row) = state.db.get_password_reset_token(&hashed_token).await? {
        if row.token_expiry < Utc::now().timestamp() {
            return Ok(Json(json!({"error": "Token expiré"})));
        }

        // Supprimer tous les tokens de ce user
        state.db.delete_all_password_reset_tokens(&row.user_id).await?;

        // Mettre à jour le mot de passe
        let hashed_pw = hash(&payload.new_password, DEFAULT_COST)?;
        state.db.update_user_password(row.user_id, &hashed_pw).await?;

        return Ok(Json(json!({"message": "Mot de passe réinitialisé"})));
    }

    Ok(Json(json!({"error": "Token invalide"})))
}

async fn jobssearch_handler(Query(search): Query<SearchQuery>) -> Result<Json<Vec<Job>>> {
    let res = jobs_search(&search.search).await?;
    Ok(Json(res))
}

async fn jobs_handler(State(state): State<AppState>) -> Result<Json<Vec<Job>>> {
    let res = state.metiers.read().unwrap().metiers.clone();
    Ok(Json(res))
}

async fn jobs_pagination_handler(
    State(state): State<AppState>,
    Query(params): Query<JobsPagination>,
) -> Result<Json<Vec<Job>>> {
    let res = state.metiers.read().unwrap().metiers.clone();
    let page = params.page;
    let per_page = params.per_page;
    let start = (page - 1) * per_page;
    let end = start + per_page;

    let paginated_jobs = if start < res.len().try_into().unwrap() {
        res[start as usize..std::cmp::min(end as usize, res.len())].to_vec()
    } else {
        Vec::new()
    };

    Ok(Json(paginated_jobs))
}

async fn start_google_auth(State(state): State<AppState>) -> Redirect {
    let google_auth = state.google_auth.read().unwrap().clone();
    let redirect_uri = &google_auth.redirect_uris[0];
    let auth_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope=email profile",
        google_auth.auth_uri, google_auth.client_id, redirect_uri
    );
    Redirect::temporary(&auth_url)
}

async fn handle_google_callback(
    Query(params): Query<OAuthCallback>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>> {
    let google_auth = state.google_auth.read().unwrap().clone();

    // Échanger le code d'autorisation contre un token d'accès
    let client = reqwest::Client::new();
    let response = client
        .post(&google_auth.token_uri)
        .form(&[
            ("code", params.code.as_str()),
            ("client_id", google_auth.client_id.as_str()),
            ("client_secret", google_auth.client_secret.as_str()),
            ("redirect_uri", google_auth.redirect_uris[0].as_str()),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
        .expect("Failed to send request");

    let token_response: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse token response");

    // Récupérer les informations de l'utilisateur depuis Google
    let access_token = token_response["access_token"]
        .as_str()
        .expect("Missing access token");
    let user_info_response = client
        .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
        .bearer_auth(access_token)
        .send()
        .await
        .expect("Failed to fetch user info");

    let user_info: serde_json::Value = user_info_response
        .json()
        .await
        .expect("Failed to parse user info");

    // Extract information of user
    let email = user_info["email"].as_str().unwrap_or_default();
    let firstname = user_info["given_name"].as_str().unwrap_or_default();
    let lastname = user_info["family_name"].as_str().unwrap_or_default();

    // Verify if the user already exists in the database
    if state.db.exist_user(email).await? {
        let user_id = state.db.get_user_id(email).await?;
        let token = create_jwt(user_id.id)?;
        return Ok(Json(json!({ "token": token, "id": user_id.id })));
    }
    // Create a new user
    let new_user = User {
        username: format!("{}{}", firstname, lastname),
        firstname: firstname.to_string(),
        lastname: lastname.to_string(),
        address: "Unknown".to_string(),
        email: email.to_string(),
        city: "Unknown".to_string(),
        postalcode: 0,
        number_phone: 0.to_string(),
        role: "google_user".to_string(),
        age: 0,
        password: hash("default_password", DEFAULT_COST).unwrap(),
        experience: 0,
        badges: Vec::new(),
    };

    // add New user to database
    let id = state.db.create_user(&new_user).await?;

    // Retourner un token JWT
    let token = create_jwt(id)?;
    Ok(Json(json!({ "token": token, "id":id })))
}
