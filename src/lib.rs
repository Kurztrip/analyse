#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate mongodb;

use repositories::database_configuration;

pub mod routes;
pub mod data;
pub mod repositories;

pub fn rocket_builder()->rocket::Rocket{
    let client = database_configuration::init()
        .unwrap_or_else(|error|panic!("Error connecting to DB:{} ",error));
    rocket::ignite()
        .mount("/trucks", routes![
            routes::trucks::add_truck,
            routes::trucks::get_trucks,
            routes::trucks::update_truck,
            routes::trucks::get_truck,
            routes::trucks::delete_truck,
            routes::trucks::change_truck_state
        ])
        .mount("/packages", routes![
            routes::packages::add_package,
            routes::packages::get_all_packages,
            routes::packages::get_package,
            routes::packages::update_package,
            routes::packages::delete_package
        ])
        .mount("/warehouses", routes![
            routes::warehouses::add_warehouse,
            routes::warehouses::get_all_warehouses,
            routes::warehouses::get_warehouse,
            routes::warehouses::delete_warehouse,
            routes::warehouses::update_warehouse,
            routes::warehouses::get_trucks_in_warehouse,
            routes::warehouses::get_packages_in_warehouse
        ])
        .register(catchers!(
            routes::catchers::not_found,
            routes::catchers::unprocessable_entity,
            routes::catchers::internal_err
        ))
        .manage(client)
}