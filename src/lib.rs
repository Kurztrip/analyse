#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate mongodb;
pub mod routes;
pub mod data;
pub mod repositories;
pub mod database_configuration;

pub fn rocket_builder()->rocket::Rocket{
    let client = database_configuration::init()
        .unwrap_or_else(|error|panic!("Error al conectar:{} ",error));
    rocket::ignite()
        .mount("/trucks", routes![
            routes::trucks::add_truck,
            routes::trucks::get_trucks,
            routes::trucks::update_truck,
            routes::trucks::get_truck,
            routes::trucks::delete_truck
        ])
        .register(catchers!(
            routes::catchers::not_found
        ))
        .manage(client)
}