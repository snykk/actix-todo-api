use crate::models::todo::{Todo, NewTodo, UpdateTodo};
use crate::schema::todos::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{PooledConnection, ConnectionManager};
use diesel::PgConnection;
use uuid::Uuid;

pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_todos(conn: &mut DBConnection, user_id_filter: Uuid) -> Vec<Todo> {
    todos
        .filter(user_id.eq(user_id_filter))
        .load::<Todo>(conn)
        .unwrap_or_default()
}

pub fn create_todo(conn: &mut DBConnection, new_todo: NewTodo) -> Todo {
    diesel::insert_into(todos)
        .values(&new_todo)
        .get_result::<Todo>(conn)
        .expect("Error saving new todo")
}

pub fn update_todo(
    conn: &mut DBConnection,
    todo_id: Uuid,
    user_id_filter: Uuid,
    changes: UpdateTodo,
) -> Option<Todo> {
    diesel::update(todos.filter(id.eq(todo_id).and(user_id.eq(user_id_filter))))
        .set(&changes)
        .get_result::<Todo>(conn)
        .ok()
}

pub fn delete_todo(conn: &mut DBConnection, todo_id: Uuid, user_id_filter: Uuid) -> bool {
    let deleted_count = diesel::delete(todos.filter(id.eq(todo_id).and(user_id.eq(user_id_filter))))
        .execute(conn)
        .unwrap_or(0);

    deleted_count > 0
}
