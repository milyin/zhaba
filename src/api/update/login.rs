use rocket::request::Form;
use rocket::request::State;
use maud::Markup;
use kit::form::to_form;
use app::Model;

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
pub fn post( model: State<Model>, _data: Form<Login>) -> String {
    model.inc();
    model.get().to_string()
}
