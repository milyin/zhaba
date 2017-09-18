use rocket::request::Form;
use maud::Markup;
use kit::form::to_form;

#[derive(FromForm, Default, Serialize)]
pub struct Login {
    name: String,
    password: String,
}

#[get("/login")]
pub fn get() -> Markup {
    to_form(&Login::default())
}

#[post("/login", data = "<_data>")]
pub fn post(_data: Form<Login>) -> String {
    "NOT IMPLEMENTED".to_owned()
}
