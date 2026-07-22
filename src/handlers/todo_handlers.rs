use crate::{
    config::database::get_database_connection,
    models::{
        api_response::{error_response, success_response},
        todo::{NewTodo, Todo},
    },
    schema::todos::dsl::*,
};
use axum::{Json, extract::Path, response::IntoResponse};
use diesel::{
    ExpressionMethods, RunQueryDsl, SelectableHelper, associations::HasTable, query_dsl::methods::*,
};

pub async fn get_all_todos() -> impl IntoResponse {
    let connection = &mut get_database_connection();
    let res = todos
        // .filter(published.eq(true))
        .select(Todo::as_select())
        .load(connection);
    match res {
        Err(_) => error_response(None),
        Ok(results) => success_response(None, Some(results)),
    }
}

pub async fn create_todo(Json(payload): Json<NewTodo>) -> impl IntoResponse {
    let connection = &mut get_database_connection();

    let new_todo = NewTodo {
        title: payload.title,
        body: payload.body,
        published: payload.published,
    };

    let res = diesel::insert_into(todos::table())
        .values(new_todo)
        .returning(Todo::as_returning())
        .get_result(connection);

    match res {
        Err(_) => error_response(None),
        Ok(data) => success_response(None, Some(data)),
    }
}

pub async fn publish_todo(Path(todo_id): Path<i32>) -> impl IntoResponse {
    let connection = &mut get_database_connection();

    let existing_todo = todos
        .select(Todo::as_select())
        .filter(id.eq(todo_id))
        .first(connection);

    match existing_todo {
        Err(_) => error_response(Some(format!("No todo with ID {}", todo_id))),
        Ok(exist_todo) => {
            let updated_todo = diesel::update(todos.find(exist_todo.id))
                .set(published.eq(true))
                .returning(Todo::as_select())
                .get_result(connection);
            match updated_todo {
                Err(_) => error_response(Some(format!("No todo with ID {}", todo_id))),
                Ok(updated) => success_response(None, Some(updated)),
            }
        }
    }
}
pub async fn unpublish_todo(Path(todo_id): Path<i32>) -> impl IntoResponse {
    let connection = &mut get_database_connection();

    let existing_todo = todos
        .select(Todo::as_select())
        .filter(id.eq(todo_id))
        .first(connection);

    match existing_todo {
        Err(_) => error_response(Some(format!("No todo with ID {}", todo_id))),
        Ok(exist_todo) => {
            let updated_todo = diesel::update(todos.find(exist_todo.id))
                .set(published.eq(false))
                .returning(Todo::as_select())
                .get_result(connection);
            match updated_todo {
                Err(_) => error_response(Some(format!("No todo with ID {}", todo_id))),
                Ok(updated) => success_response(None, Some(updated)),
            }
        }
    }
}

pub async fn get_todo_by_id(Path(todo_id): Path<i32>) -> impl IntoResponse {
    let connection = &mut get_database_connection();

    let existing_todo = todos
        .select(Todo::as_select())
        .filter(id.eq(todo_id))
        .first(connection);

    match existing_todo {
        Err(_) => error_response(None),
        Ok(data) => success_response(None, Some(data)),
    }
}
