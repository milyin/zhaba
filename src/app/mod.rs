pub use self::db::models::User;
pub use self::error::{ ModelError, ModelResult };
pub use self::model::{Model};

mod db;
mod error;
mod model;

