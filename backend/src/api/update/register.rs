use rocket::request::Form;
use rocket::request::State;
use rocket_contrib::Json;
use maud::Markup;
use kit::form::to_form;
use app::{Model, ModelResult};

#[derive(FromForm, Default, Serialize)]
pub struct Register {
    name: String,
    email: String,
    password: String,
}

#[get("/register")]
pub fn get() -> Markup {
    to_form(&Register::default())
}

#[post("/register", data = "<data>")]
pub fn post(model: State<Model>, data: Form<Register>) -> Json<ModelResult<()>> {
    let form = data.get();
    Json(model.register(&form.name, &form.email, &form.password))
}
