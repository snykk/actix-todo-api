use serde::{Deserialize, Serialize};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use uuid::Uuid;
use crate::schema::todos;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Selectable, Deserialize, Debug)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,

    #[serde(skip_deserializing)]
    pub user_id: Uuid,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = todos)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
