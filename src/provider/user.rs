use crate::db::schema::users::*;
use crate::model::user::{UserGet, UserNew, UserUpdate};
use diesel::result::Error;
use diesel::*;

pub struct UserProvider;

impl UserProvider {
    pub fn get_list(
        connection: &mut PgConnection,
        take: i64,
        skip: i64,
    ) -> Result<Vec<UserGet>, Error> {
        table
            .limit(take)
            .order(id)
            .offset(skip)
            .select(UserGet::as_returning())
            .load(connection)
    }

    pub fn get_by_id(connection: &mut PgConnection, user_id: i32) -> Result<UserGet, Error> {
        table
            .find(user_id)
            .select(UserGet::as_select())
            .first(connection)
    }

    pub fn create(connection: &mut PgConnection, new_user: &UserNew) -> Result<UserGet, Error> {
        diesel::insert_into(table)
            .values(new_user)
            .returning(UserGet::as_returning())
            .get_result(connection)
    }

    pub fn update(
        connection: &mut PgConnection,
        user_id: i32,
        update_data: &UserUpdate,
    ) -> Result<UserGet, Error> {
        diesel::update(table)
            .filter(id.eq(user_id))
            .set(update_data)
            .returning(UserGet::as_select())
            .get_result(connection)
    }

    pub fn delete(connection: &mut PgConnection, user_id: i32) -> Result<UserGet, Error> {
        diesel::delete(table)
            .filter(id.eq(user_id))
            .returning(UserGet::as_select())
            .get_result(connection)
    }
}
