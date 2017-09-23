use rocket::request::Form;
use rocket::request::State;
use rocket_contrib::Json;
use maud::Markup;
use kit::form::to_form;
use app::{Model, ModelResult, AuthInfo};

#[derive(FromForm, Default, Serialize)]
pub struct Post {
    pub title: String,
    pub body: String,
}

#[get("/post")]
pub fn get() -> Markup {
    to_form(&Post::default())
}

#[post("/post", data = "<data>")]
pub fn post(auth: AuthInfo, model: State<Model>, data: Form<Post>) -> Json<ModelResult<()>> {
    let form = data.get();
    Json(model.post(&auth, &form.title, &form.body))
}
