use axum::{Json, Extension};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use firebase_auth_sdk::FireAuth;
use crate::model::{CredsRequest, Response};

pub async fn sign_in(service: Extension<FireAuth>, Json(creds_request) : Json<CredsRequest>)
-> impl IntoResponse {
  match service.sign_in_email(creds_request.email.as_str(), creds_request.password.as_str(), true).await {
     Ok(response) => {
            println!("{:?}",response);
            let response_body = Response {
                message: String::from("Successfully logged in"),
                email: Some(response.email.clone()),
                id: Some(response.local_id.clone()),
                token: Some(response.id_token.clone()),
            };
            (
                StatusCode::OK,
                Json(response_body),
            )
        }
         Err(ex) => {
            eprintln!("{:?}", ex);
            let response_body = Response {
                message: String::from("Invalid credentials"),
                token: None,
                id: None,
                email: None
            };
            (
                StatusCode::UNAUTHORIZED,
                Json(response_body),
            )
        }
  }
}

pub async fn sign_up(service: Extension<FireAuth>, Json(creds_request) : Json<CredsRequest>) -> Result<Json<Response>, StatusCode> {
  match service.sign_up_email(creds_request.email.as_str(), creds_request.password.as_str(), false).await {
    Ok(_) => {
        let msg = Response {message: String::from("Successfully registrated, please login...!"), token: None, id: None, email: None};
        Ok(Json(msg))
      
    }
    Err(ex) => {
      eprintln!("{:?}", ex);
      Err(StatusCode::BAD_REQUEST)
    }
  }
}