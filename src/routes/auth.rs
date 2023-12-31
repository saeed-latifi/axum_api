use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, Cookies};

use crate::AppState;

pub fn auth_route_handler() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "auth".into_response() }))
        .route(
            "/login",
            post(login).get(|| async { "login".into_response() }),
        )
}

async fn login(cookies: Cookies, payload: Json<LoginPayload>) -> impl IntoResponse {
    println!("login process ...");
    // TODO from db!
    // sample
    if payload.username != "saeed" || payload.password != "1234" {
        return (StatusCode::FORBIDDEN, "forbidden").into_response();
    }

    cookies.add(Cookie::new("x-cookie", "1234"));
    Json(json!({"status":"ok"})).into_response()
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
