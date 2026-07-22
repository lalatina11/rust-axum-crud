use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::todos;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = todos)] // todos come from => use crate::schema::todos
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = todos)] // todos come from => use crate::schema::todos
pub struct NewTodo {
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
}
