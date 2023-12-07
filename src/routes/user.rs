use crate::provider::user::UserProvider;
use crate::{db::schema::users::*, model::user::UserGet};
use diesel::*;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

pub fn user_route_handler() -> Router {
    Router::new().route("/user", get(get_users_list))
}

async fn get_users_list(pool: &deadpool_diesel::postgres::Pool) -> impl IntoResponse {
    let connection = pool.get().await.unwrap();

    let res = connection
        .interact(move |conn| {
            table
                .limit(20)
                .order(id)
                .offset(0)
                .select(UserGet::as_returning())
                .load(conn)
        })
        .await
        .unwrap()
        .unwrap();

    // let users_list = UserProvider::get_list(connection, 20, 0);

    Html(format!("hello you {} !", res[0].username))
}
