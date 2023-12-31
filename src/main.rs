use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::get;
use deadpool_diesel::postgres::{Manager, Pool};
use demo::middlewares::logger::response_logger;
use demo::routes::auth::auth_route_handler;
use demo::routes::sample::sample_route_handler;
use demo::routes::statics::statics_route_handler;
use demo::routes::user::user_route_handler;
use demo::AppState;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url: String = std::env::var("DATABASE_URL").expect("env 'DATABASE_URL' not set");
    let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();
    let state: AppState = AppState { pool };

    let app_router = axum::Router::new()
        .route("/", get(greeting))
        .nest("/user", user_route_handler())
        .nest("/auth", auth_route_handler())
        .nest("/sample", sample_route_handler())
        .layer(middleware::map_response(response_logger))
        .layer(CookieManagerLayer::new())
        .with_state(state)
        .fallback_service(statics_route_handler());

    let host = "127.0.0.1:3000".to_string();
    let listener = TcpListener::bind(host.clone()).await.unwrap();
    println!(" app is running on http://{}", host);
    axum::serve(listener, app_router).await.unwrap();
}

async fn greeting() -> impl IntoResponse {
    "hi! this is a main page. you also can visit /sample page with query or name params. and more in way "
}
