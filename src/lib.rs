pub mod db;
pub mod middlewares;
pub mod model;
pub mod provider;
pub mod routes;
use deadpool_diesel::postgres::Pool;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}
