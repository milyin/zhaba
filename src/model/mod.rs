use std::sync::atomic::{AtomicUsize, Ordering};
use r2d2_diesel::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use std::env;
use dotenv::dotenv;
use r2d2;
use models;
use diesel::prelude::LoadDsl;
use std::error;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type Error = Box<error::Error>;

/// Initializes a database pool.
fn init_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("db pool")
}

pub struct Model {
    pool: Pool,
    counter: AtomicUsize
}

impl Model {

    pub fn new() -> Model {
        Model {
            pool: init_pool(),
            counter: AtomicUsize::new(0)
        }
    }
    pub fn inc(&self) {
        self.counter.fetch_add(1,Ordering::Relaxed);
    }
    pub fn get(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    pub fn users(&self) -> Result<Vec<models::User>, Error> {
        let conn = self.pool.get()?;
        use schema::users::dsl::users;
        let v = users.load::<models::User>(&*conn)?;
        Ok(v)
    }
}