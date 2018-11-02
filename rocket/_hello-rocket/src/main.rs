#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rocket_contrib::Json;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
pub mod models;

#[get("/")]
fn index() -> &'static str {
    "Hey, world!"
}

#[get("/testjson")]
fn testjson() -> Json<String> {
    let str = String::new();
    let string = "Love".to_string();
    Json(string)
}

#[get("/testdata")]
fn user() -> Json<String> {
    let str = String::new();
    Json(str)
}

// What we want in the end
// #[get("/users")]
// fn handler(conn: DbConn) { ... }

// An alias to the type for a pool of Diesel SQLite connections.
type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub PooledConnection<ConnectionManager<SqliteConnection>>);

// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<SqlitePool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Initializes a database pool.
fn init_pool() -> SqlitePool {
    let manager = ConnectionManager::<SqliteConnection>::new("/Users/ethan/Documents/_DEV_/Workspaces/github/NetCore_Angular/DatingApp.API/DatingApp.db");
    Pool::new(manager).expect("db pool")
}

// #[get("/users", format = "application/json")]
// fn get_users(conn: DbConn) -> QueryResult<Json<Vec<int>>> {
//     all_users.order()
//         .load::<User>(&*conn)
//         .map(|users| Json(users))
// }

fn main() {
    rocket::ignite().mount("/", routes![index, testjson]).launch();

     rocket::ignite()
        .manage(init_pool())
        .launch();
}