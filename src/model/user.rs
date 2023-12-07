use crate::db::schema::users;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = users,check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_active: bool,
}

#[derive(Queryable, Selectable, Debug, PartialEq, Serialize)]
#[diesel(table_name = users,check_for_backend(diesel::pg::Pg))]
pub struct UserGet {
    pub id: i32,
    pub username: String,
    // pub password: String,
    // pub is_active: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = users,check_for_backend(diesel::pg::Pg))]
pub struct UserUpdate<'a> {
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
}

#[derive(Insertable)]
#[diesel(table_name = users,check_for_backend(diesel::pg::Pg))]
pub struct UserNew<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
