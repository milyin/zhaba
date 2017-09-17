use rocket::request::Form;
use rocket_contrib::Json;
use maud::Markup;
use kit::form::to_form;

#[derive(FromForm, Default, Serialize)]
pub struct Login {
    name: String,
    password: String,
}

#[get("/login")]
pub fn get() -> Markup {
    to_form(&Login {
        name: "a".to_owned(),
        password: "b".to_owned(),
    })
    //    to_form(&Login::default())
}

#[post("/login", data = "<data>")]
pub fn post(data: Form<Login>) -> String {
    "NOT IMPLEMENTED".to_owned()
}
