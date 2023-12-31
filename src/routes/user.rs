use crate::db::schema::users::{id, table};
use crate::model::user::UserGet;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use diesel::*;

pub fn user_route_handler() -> Router<AppState> {
    Router::new().route("/", get(get_users_list))
}

async fn get_users_list(State(state): State<AppState>) -> impl IntoResponse {
    let connection = state.pool.get().await.unwrap();

    let data = connection
        .interact(|conn| {
            let users = table
                .limit(20)
                .order(id)
                .offset(0)
                .select(UserGet::as_returning())
                .load(conn)
                .unwrap();
            users
        })
        .await
        .unwrap();

    Json(data)
}
