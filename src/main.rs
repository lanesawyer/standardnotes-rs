#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate log;
#[macro_use]
extern crate rocket;

mod api;
mod db;
mod jwt;
mod models;
mod schema;

fn main() {
    //env_logger::init();

    rocket::ignite()
        .mount(
            "/auth",
            routes![api::auth, api::change_pw, api::sign_in, api::params],
        )
        .mount("/items", routes![api::sync])
        .launch();
}
