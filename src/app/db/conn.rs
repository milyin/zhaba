use r2d2;
use r2d2_diesel::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use std::env;
use dotenv::dotenv;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Initializes a database pool.
pub fn init_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("db pool")
}
