use crate::middlewares::logger::response_logger;
use crate::routes::{
    auth::auth_route_handler, sample::user_route_handler, statics::statics_route_handler,
};
use axum::middleware;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

mod middlewares;
mod routes;

#[tokio::main]
async fn main() {
    let app_router = axum::Router::new()
        .merge(user_route_handler())
        .merge(auth_route_handler())
        .layer(middleware::map_response(response_logger))
        .layer(CookieManagerLayer::new())
        .fallback_service(statics_route_handler());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app_router).await.unwrap();
}
