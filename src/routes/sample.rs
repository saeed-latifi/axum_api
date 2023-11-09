use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

pub fn user_route_handler() -> Router {
    Router::new()
        .route("/sample", get(route_with_query))
        .route("/sample/:name", get(route_with_dynamic_path))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
async fn route_with_query(Query(params): Query<HelloParams>) -> impl IntoResponse {
    match params.name {
        None => Html(format!("hello you!")),
        Some(name) => Html(format!("hello {name} !")),
    }
}

async fn route_with_dynamic_path(name: Path<String>) -> impl IntoResponse {
    let name = name.to_string();
    Html(format!("hello {name} !"))
}
