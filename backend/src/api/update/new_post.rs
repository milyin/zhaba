use rocket::request::Form;
use rocket::request::State;
use rocket_contrib::Json;
use maud::Markup;
use kit::form::to_form;
use app::{Model, ModelResult, AuthInfo};

#[derive(FromForm, Default, Serialize)]
pub struct NewPost {
    title: String,
    body: String,
}

#[get("/new_post")]
pub fn get() -> Markup {
    to_form(&NewPost::default())
}

#[post("/new_post", data = "<data>")]
pub fn post(auth: AuthInfo, model: State<Model>, data: Form<NewPost>) -> Json<ModelResult<()>> {
    let form = data.get();
    Json(model.new_post(&auth, &form.title, &form.body))
}
