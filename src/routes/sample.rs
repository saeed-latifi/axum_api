use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

use crate::AppState;

pub fn sample_route_handler() -> Router<AppState> {
    Router::new()
        .route("/", get(route_with_query))
        .route("/:name", get(route_with_dynamic_path))
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

async fn route_with_dynamic_path(cookies: Cookies, name: Path<String>) -> impl IntoResponse {
    let name = name.to_string();
    let mut cookie: Cookie<'_> = Cookie::build(("name-v", name.clone())).into();

    if name == "no" {
        cookie.set_max_age(Duration::hours(0));
    }

    cookies.add(cookie);
    Html(format!("hello {name} !"))
}
