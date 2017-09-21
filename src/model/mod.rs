use std::sync::atomic::{AtomicUsize, Ordering};
use r2d2_diesel::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use std::env;
use dotenv::dotenv;
use r2d2;
use models;
use diesel::prelude::LoadDsl;
use diesel::prelude::LimitDsl;
use diesel::prelude::ExecuteDsl;
use diesel::prelude::FilterDsl;
use diesel::ExpressionMethods;
use std::error::Error;
use std::fmt;
use diesel;
use serde::ser::{Serialize, Serializer};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Initializes a database pool.
fn init_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("db pool")
}

#[derive(Debug)]
pub enum ModelError {
    DieselError(diesel::result::Error),
    DieselConnectionError(diesel::result::ConnectionError),
    ConnectionPoolError(r2d2::GetTimeout),
    UserExists
}

impl<'v> Serialize for ModelError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S:Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<diesel::result::Error> for ModelError {
    fn from(err: diesel::result::Error) -> ModelError {
        ModelError::DieselError(err)
    }
}

impl From<diesel::result::ConnectionError> for ModelError {
    fn from(err: diesel::result::ConnectionError) -> ModelError {
        ModelError::DieselConnectionError(err)
    }
}

impl From<r2d2::GetTimeout> for ModelError {
    fn from(err: r2d2::GetTimeout) -> ModelError {
        ModelError::ConnectionPoolError(err)
    }
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModelError::DieselError(ref err) => err.fmt(f),
            ModelError::DieselConnectionError(ref err) => err.fmt(f),
            ModelError::ConnectionPoolError(ref err) => err.fmt(f),
            ref ownerror => write!(f, "{}", ownerror.description())
        }
    }
}

impl Error for ModelError {
    fn description(&self) -> &str {
        match *self {
            ModelError::DieselError(ref err) => err.description(),
            ModelError::DieselConnectionError(ref err) => err.description(),
            ModelError::ConnectionPoolError(ref err) => err.description(),
            ModelError::UserExists => "user already exists"
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            ModelError::DieselError(ref err) => Some(err),
            ModelError::DieselConnectionError(ref err) => Some(err),
            ModelError::ConnectionPoolError(ref err) => Some(err),
            ModelError::UserExists => None
        }
    }
}

pub type ModelResult<T> = Result<T, ModelError>;

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

    pub fn users(&self) -> ModelResult<Vec<models::User>> {
        let conn = self.pool.get()?;
        use schema::users::dsl::users;
        Ok(users.load::<models::User>(&*conn)?)
    }
    pub fn register(&self, name: &str, email: &str, password: &str ) -> ModelResult<()> {
        let conn = self.pool.get()?;
        use schema::users;
        let user = users::table.filter(users::name.eq(name)).limit(1).load::<models::User>(&*conn)?;
        if user.len() > 0 {
           Err(ModelError::UserExists)
        } else {
            diesel::insert(&models::NewUser {
                name: name,
                email: email,
                password_hash: password
            }).into(users::table).execute(&*conn)?;
            Ok(())
        }
    }
}