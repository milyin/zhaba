#![feature(plugin, proc_macro, custom_derive, never_type)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate maud;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate time;

use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use std::env;
use dotenv::dotenv;
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use app::Model;

pub mod api;
pub mod kit;
pub mod app;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(
    pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>,
);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
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

// An alias to the type for a pool of Diesel SQLite connections.
type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Initializes a database pool.
fn init_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("db pool")
}


fn main() {
    rocket::ignite()
        .manage(init_pool())
        .manage(Model::new())
        .mount(
            "/",
            routes![
                api::update::login::get,
                api::update::login::post,
                api::update::register::get,
                api::update::register::post,
                api::view::users::get
            ],
        )
        .launch();
}
