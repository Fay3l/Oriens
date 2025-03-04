use crate::{
    add_experience, add_user, create_jwt, jobs, load_user,
    models::{
        AppState, Claims, Job, MetiersPossibles, Question, Questionnaire, SearchQuery, Section,
        User, UserLogin,
    },
    save_user, questionnaire_result, verify_user,
};
use axum::{
    extract::{Query, Request},
    http::StatusCode,
    middleware::{from_fn, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use mistralai_client::v1::client::Client;
use serde_json::json;

pub fn api_routes() -> Router {
    Router::new()
        .route("/api", get(handler).layer(from_fn(validate_token)))
        .route("/api/register", post(register_user))
        .route("/api/login", post(login_user))
        .route("/api/jobs/search", get(jobssearch_handler))
        .route("/api/survey/result", post(survey_handler))
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
        let _decode = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &validation,
        )?;
    }
    Ok(next.run(req).await)
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn login_user(Json(payload): Json<UserLogin>) -> Json<serde_json::Value> {
    println!("{:?}", payload);
    match verify_user(&payload.username, &payload.password) {
        Ok(true) => {
            let mut user = match load_user(&payload.username) {
                Ok(user) => user,
                Err(e) => return Json(json!({})),
            };
            println!("Add experience");
            // Add experience
            add_experience(&mut user, 10);
            println!("Upload user");
            // Save the updated user
            if let Err(e) = save_user(&user) {
                return Json(json!({}));
            }
            println!("Create Token ");
            match create_jwt(&payload.username) {
                Ok(token) => Json(json!({"token": token})),
                Err(_) => Json(json!({})),
            }
        }
        Ok(false) => Json(json!({"error": "Invalid credentials"})),
        Err(_) => Json(json!({})),
    }
}

async fn survey_handler(
    Json(survey): Json<Vec<Section>>,
) -> Result<Json<MetiersPossibles>> {
    let res = questionnaire_result(survey).await?;
    Ok(Json(res))
}

async fn register_user(Json(payload): Json<User>) -> Result<Json<serde_json::Value>> {
    let _ = add_user(payload)?;
    Ok(Json(json!({"message": "User registered successfully"})))
}

async fn jobssearch_handler(Query(search): Query<SearchQuery>) -> Result<Json<Vec<Job>>> {
    let res = jobs(&search.search).await?;
    Ok(Json(res))
}


