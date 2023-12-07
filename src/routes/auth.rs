use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, Cookies};

pub fn auth_route_handler() -> Router {
    Router::new().route("/api/auth/login", post(login))
}

async fn login(cookies: Cookies, payload: Json<LoginPayload>) -> impl IntoResponse {
    println!("login process ...");
    // TODO from db!
    // sample
    if payload.username != "saeed" || payload.password != "1234" {
        return (StatusCode::INTERNAL_SERVER_ERROR, "unhandled client error").into_response();
    }

    cookies.add(Cookie::new("x-cookie", "1234"));
    Json(json!({"status":"ok"})).into_response()
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
