#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod api;
mod jwt;

fn main() {
    rocket::ignite()
        .mount(
            "/auth",
            routes![api::auth, api::change_pw, api::sign_in, api::params],
        )
        .mount("/items", routes![api::sync])
        .launch();
}
