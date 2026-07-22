use axum::{
    Router,
    routing::{get, patch},
};
use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};

use crate::{
    config::database::get_database_connection,
    handlers::todo_handlers::{
        create_todo, get_all_todos, get_todo_by_id, publish_todo, unpublish_todo,
    },
    models::todo::NewTodo,
    schema::todos::dsl::*,
};

pub async fn todo_seed() {
    let range = 10_000;
    let connection = &mut get_database_connection();

    let res = todos.count().get_result::<i64>(connection);

    match res {
        Err(err) => {
            panic!("Todo count error:\n{}", err)
        }
        Ok(count) => {
            println!("todo number: {}", count);
            if count < 50 {
                println!("Need to seed todos\nSeeding todos...");
                for i in 1..range + 1 {
                    let new_todo = NewTodo {
                        title: format!("Todo title {}", i),
                        body: format!("Todo body {}", i),
                        published: if i % 2 == 0 { Some(true) } else { Some(false) },
                    };

                    let res = diesel::insert_into(todos::table())
                        .values(new_todo)
                        .execute(connection);

                    match res {
                        Err(err) => {
                            println!("Error while seeding todo\n{}", err)
                        }
                        Ok(_) => {
                            println!("{} of {} todos inserted", i, range);
                            continue;
                        }
                    }
                }
                let res = todos.count().get_result::<i64>(connection);
                match res {
                    Err(err) => {
                        println!("Todo seeder Error\n{}", err)
                    }
                    Ok(n) => {
                        println!("Succes inserting {} of todos", n)
                    }
                }
            } else {
                println!("Seeding todos in unnecessary\nNo todos inserted")
            }
        }
    }
}

fn create_dynamic_todo_router() -> Router {
    Router::new()
        .route("/", get(get_todo_by_id))
        .route("/publish", patch(publish_todo))
        .route("/unpublish", patch(unpublish_todo))
}

pub fn create_todo_router() -> Router {
    let dynamic_router = create_dynamic_todo_router();
    Router::new()
        .route("/", get(get_all_todos).post(create_todo))
        .nest("/{todo_id}", dynamic_router)
}
