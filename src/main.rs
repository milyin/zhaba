#![feature(plugin, proc_macro, custom_derive, never_type)]
#![plugin(rocket_codegen)]
#![recursion_limit="128"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate maud;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;
#[macro_use]
extern crate lazy_static;

use rocket::response::NamedFile;
use app::Model;
use std::io;
use std::path::{Path, PathBuf};

pub mod api;
pub mod kit;
pub mod app;


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .manage(Model::new())
        .mount("/", routes![index, files])
        .mount(
            "/view",
            routes![
                api::view::users::get,
                api::view::login::get,
                api::view::posts::get,
            ],
        )
        .mount("/query", routes![])
        .mount(
            "/update",
            routes![
                api::update::login::get,
                api::update::login::post,
                api::update::logout::get,
                api::update::logout::post,
                api::update::register::get,
                api::update::register::post,
                api::update::register::post_json,
                api::update::new_post::get,
                api::update::new_post::post,
                api::update::edit_post::get,
                api::update::edit_post::post,
                api::update::edit_post::post_json,
            ],
        )
        .launch();
}
