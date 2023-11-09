use crate::routes::{
    auth::auth_route_handler, sample::user_route_handler, statics::statics_route_handler,
};

mod routes;

#[tokio::main]
async fn main() {
    let app_router = axum::Router::new()
        .merge(user_route_handler())
        .merge(auth_route_handler())
        .fallback_service(statics_route_handler());

    let server_address = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("server is run on {}", server_address);

    axum::Server::bind(&server_address)
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
