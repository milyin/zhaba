use rocket_contrib::Json;
use rocket::request::State;
use models::User;
use model::{Model, ModelResult};

#[get("/users")]
fn get(model:State<Model>) -> Json<ModelResult<Vec<User>>> {
    Json(model.users())
}
