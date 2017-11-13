#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate tomorrow_core;
extern crate tomorrow_http;
extern crate tomorrow_recuperator;

extern crate recuperator_google_maps;
extern crate recuperator_google;

extern crate rocket;
extern crate rocket_contrib;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate time;

mod api_response;
mod services;
mod errors;

fn main() {
    rocket::ignite()
        .mount("/services/maps", routes![services::maps::search])
        .mount("/services/search", routes![services::search::search])
        .catch(errors![errors::not_found, errors::internal_server_error])
        .launch();
}