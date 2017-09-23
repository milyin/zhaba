pub use self::db::models::User;
pub use self::error::{ModelError, ModelResult};
pub use self::model::{Model, AuthToken, AuthInfo};
pub use self::settings::COOKIE_TOKEN;

mod db;
mod error;
mod model;
mod settings;
