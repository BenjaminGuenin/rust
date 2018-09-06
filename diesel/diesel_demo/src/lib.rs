
pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Post, NewPost};

pub fn establish_connection() -> PgConnection
{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Database error connection {}", database_url))
}

pub fn create_post<'a>(connection: &PgConnection, title: &'a str, body: &'a str) -> Post
{
    use schema::posts;
    let new_post = NewPost
    {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(connection)
        .expect("Error saving new post")
}