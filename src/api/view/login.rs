use rocket_contrib::Json;
use app::{ModelResult, AuthInfo};

#[get("/login")]
pub fn get(auth: ModelResult<AuthInfo>) -> Json<ModelResult<AuthInfo>> {
    Json(auth)
}
