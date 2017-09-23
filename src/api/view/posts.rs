use rocket_contrib::Json;
use rocket::request::State;
use app::Model;
use app::ModelResult;
use app::Post;

#[get("/posts")]
fn get(model: State<Model>) -> Json<ModelResult<Vec<Post>>> {
    Json(model.posts())
}
