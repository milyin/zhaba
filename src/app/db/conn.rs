use r2d2;
use r2d2_diesel::ConnectionManager;
use diesel::sqlite::SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Initializes a database pool.
pub fn init_pool(database_url: &str) -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("db pool")
}
