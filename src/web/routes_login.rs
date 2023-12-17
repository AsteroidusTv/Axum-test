use crate::{Error, Result, web};
use axum::{Json, routing::post, Router};
use tower_cookies::{Cookies, Cookie};
use crate::Deserialize;

use serde_json::{Value, json};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
} 

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");
    
    // Faire une vrai db mais pour l'instant tkt on apprend

    if payload.username != "Asteroidus" ||  payload.pwd != "asterlebg" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}