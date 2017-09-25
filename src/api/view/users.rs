use rocket_contrib::Json;
use rocket::request::State;
use app::Model;
use app::ModelResult;
use app::UserInfo;

#[get("/users")]
fn get(model: State<Model>) -> Json<ModelResult<Vec<UserInfo>>> {
    Json(model.users())
}
