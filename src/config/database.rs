use diesel::prelude::*;
use dotenvy::dotenv;

use crate::config::env::get_database_url;

pub fn get_database_connection() -> PgConnection {
    dotenv().ok();

    let database_url = get_database_url();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
