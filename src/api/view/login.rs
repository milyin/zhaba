use rocket::request::State;
use rocket_contrib::Json;
use rocket::http::Cookies;
use app::{Model, ModelResult, ModelError, AuthToken, AuthInfo};
use serde_json;

#[get("/login")]
pub fn get(model: State<Model>, mut cookies: Cookies) -> Json<ModelResult<AuthInfo>> {
    Json((|| {
         let cookie = cookies.get_private("token").ok_or(
            ModelError::AuthTokenNotFound,
        )?;
         let token: AuthToken = serde_json::from_str(cookie.value())?;
         model.authorize(token, "")
     })())
}
