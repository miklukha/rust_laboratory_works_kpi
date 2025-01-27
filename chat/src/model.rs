use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CredsRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Response {
    pub message: String,
    pub token: Option<String>,
    pub id: Option<String>,
    pub email: Option<String>,
}
