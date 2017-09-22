use rocket::request::Form;
use rocket::request::State;
use rocket_contrib::Json;
use rocket::http::{Cookie, Cookies};
use maud::Markup;
use kit::form::to_form;
use app::{Model, ModelResult, AuthInfo};
use serde_json;

#[derive(FromForm, Default, Serialize)]
pub struct Login {
    name: String,
    password: String,
    duration: u32,
}

#[get("/login")]
pub fn get() -> Markup {
    to_form(&Login::default())
}

#[post("/login", data = "<data>")]
pub fn post(
    model: State<Model>,
    data: Form<Login>,
    mut cookies: Cookies,
) -> Json<ModelResult<AuthInfo>> {
    let form = data.get();
    Json((|| {
         let token = model.login(&form.name, &form.password, "", form.duration)?;
         cookies.add_private(Cookie::new("token", serde_json::to_string(&token)?));
         model.authorize(token, "")
     })())
}
