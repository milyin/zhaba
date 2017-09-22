use rocket::request::Form;
use rocket::request::State;
use rocket_contrib::Json;
use maud::Markup;
use kit::form::to_form;
use app::{Model, ModelResult, AuthToken};

#[derive(FromForm, Default, Serialize)]
pub struct Login {
    name: String,
    password: String,
}

#[get("/login")]
pub fn get() -> Markup {
    to_form(&Login::default())
}

#[post("/login", data = "<data>")]
pub fn post(model: State<Model>, data: Form<Login>) -> Json<ModelResult<AuthToken>> {
    let form = data.get();
    Json(model.login(&form.name, &form.password, "", 60) )
}
