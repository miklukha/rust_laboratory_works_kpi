use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Mutex;
use lazy_static::lazy_static;
use bcrypt::{hash, verify, DEFAULT_COST};
use warp::reject::Reject;
use chrono;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

static SECRET_KEY: &[u8] = b"your_secret_key";

#[derive(Debug)]
struct TokenCreationError;
impl warp::reject::Reject for TokenCreationError {}

#[derive(Debug)]
struct InvalidCredentialsError;
impl warp::reject::Reject for InvalidCredentialsError {}

#[derive(Debug)]
struct MissingTokenError;
impl warp::reject::Reject for MissingTokenError {}

#[derive(Debug)]
struct InvalidTokenError;
impl warp::reject::Reject for InvalidTokenError {}

#[derive(Debug)]
struct MissingCredentialsError;
impl Reject for MissingCredentialsError {}

#[derive(Debug)]
struct PasswordHashError;
impl Reject for PasswordHashError {}

#[derive(Debug)]
struct UserAlreadyExistsError;
impl Reject for UserAlreadyExistsError {}

lazy_static! {
    static ref USERS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn login_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler)
}

async fn login_handler(body: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    let username = body.get("username").cloned().unwrap_or_default();
    let password = body.get("password").cloned().unwrap_or_default();

    if username.is_empty() || password.is_empty() {
        return Err(warp::reject::custom(MissingCredentialsError));
    }

    let users = USERS.lock().unwrap();

    if let Some(hashed_password) = users.get(&username) {
        if verify(password, hashed_password).map_err(|_| warp::reject::custom(PasswordHashError))? {
            let claims = Claims {
                sub: username.clone(),
                exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
            };
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
                .map_err(|_| warp::reject::custom(TokenCreationError))?;

            let json = warp::reply::json(&serde_json::json!({ "token": token }));
            Ok(warp::reply::with_status(json, warp::http::StatusCode::OK))
        } else {
            Err(warp::reject::custom(InvalidCredentialsError))
        }
    } else {
        Err(warp::reject::custom(InvalidCredentialsError))
    }
}

pub fn with_auth() -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    warp::any()
        .and(
            warp::header::optional::<String>("authorization")
                .or(warp::query::<HashMap<String, String>>().map(|query: HashMap<String, String>| {
                    query.get("token").cloned()
                }))
                .unify()
        )
        .and_then(authorize)
}

async fn authorize(token: Option<String>) -> Result<Claims, warp::Rejection> {
    let token = token.ok_or_else(|| warp::reject::custom(MissingTokenError))?;
    let token = token.replace("Bearer ", "");
    decode::<Claims>(&token, &DecodingKey::from_secret(SECRET_KEY), &Validation::new(Algorithm::HS256))
        .map(|data| data.claims)
        .map_err(|_| warp::reject::custom(InvalidTokenError))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status("Not Found", warp::http::StatusCode::NOT_FOUND))
    } else if let Some(_) = err.find::<InvalidCredentialsError>() {
        Ok(warp::reply::with_status("Invalid username or password", warp::http::StatusCode::UNAUTHORIZED))
    } else if let Some(_) = err.find::<TokenCreationError>() {
        Ok(warp::reply::with_status("Token creation error", warp::http::StatusCode::INTERNAL_SERVER_ERROR))
    } else if let Some(_) = err.find::<MissingTokenError>() {
        Ok(warp::reply::with_status("Missing token", warp::http::StatusCode::BAD_REQUEST))
    } else if let Some(_) = err.find::<InvalidTokenError>() {
        Ok(warp::reply::with_status("Invalid token", warp::http::StatusCode::UNAUTHORIZED))
    } else if let Some(_) = err.find::<MissingCredentialsError>() {
        Ok(warp::reply::with_status("Missing username or password", warp::http::StatusCode::BAD_REQUEST))
    } else if let Some(_) = err.find::<PasswordHashError>() {
        Ok(warp::reply::with_status("Error hashing password", warp::http::StatusCode::INTERNAL_SERVER_ERROR))
    } else if let Some(_) = err.find::<UserAlreadyExistsError>() {
        Ok(warp::reply::with_status("User already exists", warp::http::StatusCode::CONFLICT))
    } else {
        Ok(warp::reply::with_status("Internal Server Error", warp::http::StatusCode::INTERNAL_SERVER_ERROR))
    }
}


pub fn register_filter() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(register_handler)
}

async fn register_handler(body: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    let username = body.get("username").cloned().unwrap_or_default();
    let password = body.get("password").cloned().unwrap_or_default();

    if username.is_empty() || password.is_empty() {
        return Err(warp::reject::custom(MissingCredentialsError));
    }

    let hashed_password = hash(password, DEFAULT_COST).map_err(|_| warp::reject::custom(PasswordHashError))?;

    let mut users = USERS.lock().unwrap();

    if users.contains_key(&username) {
        return Err(warp::reject::custom(UserAlreadyExistsError));
    }

    users.insert(username.clone(), hashed_password);

    let json = warp::reply::json(&serde_json::json!({ "status": "User registered successfully" }));
    Ok(warp::reply::with_status(json, warp::http::StatusCode::OK))
}