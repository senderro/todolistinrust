use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = crate::schema::todo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub done: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::todo)]
pub struct NewTodo {
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub done: bool,
    pub due_date: Option<NaiveDateTime>,
}
