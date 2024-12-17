use crate::models::auth::{NewUser, User};
use diesel::prelude::*;
use crate::schema::users::dsl::*;
use diesel::r2d2::{PooledConnection, ConnectionManager};
use diesel::PgConnection;

pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_user(conn: &mut DBConnection, new_user: NewUser) -> User {
    diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_select())
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn find_user_by_username(conn: &mut DBConnection, uname: &str) -> Option<User> {
    users
        .filter(username.eq(uname))
        .select(User::as_select())
        .first(conn)
        .ok()
}
