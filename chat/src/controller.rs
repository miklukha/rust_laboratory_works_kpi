use axum::Json;
use crate::model::{LoginInfo, LoginResponse};

pub async fn login_handler(Json(login_info) : Json<LoginInfo>)
-> Result<Json<LoginResponse>, StatusCode> {

}

pub async fn get_info_handler() -> Result<Json<String>, StatusCode> {

}