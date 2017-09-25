use rocket::request::Form;
use rocket::request::State;
use rocket_contrib::Json;
use maud::Markup;
use kit::form::to_form;
use app::{Model, ModelResult, AuthInfo};

#[derive(FromForm, Default, Serialize, Deserialize)]
pub struct EditPost {
    post_id: i32,
    title: String,
    body: String,
}

#[get("/edit_post")]
pub fn get() -> Markup {
    to_form(&EditPost::default())
}

#[post("/edit_post", data = "<data>")]
pub fn post(auth: AuthInfo, model: State<Model>, data: Form<EditPost>) -> Json<ModelResult<()>> {
    let form = data.get();
    Json(model.edit_post(&auth, form.post_id, &form.title, &form.body))
}

#[post("/edit_post", data = "<data>")]
pub fn post_json(auth: AuthInfo, model: State<Model>, data: Json<EditPost>) -> Json<ModelResult<()>> {
    let form = data.into_inner();
    Json(model.edit_post(&auth, form.post_id, &form.title, &form.body))
}
