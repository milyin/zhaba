#![feature(plugin, proc_macro)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;

mod pages;

fn main() {
    rocket::ignite()
        .mount("/", routes![pages::index::get])
        .launch();
}
