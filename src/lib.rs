#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use repositories::database_configuration;

pub mod controllers;
pub mod data;
pub mod repositories;
pub mod logic;

pub fn rocket_builder()->rocket::Rocket{
    let database = database_configuration::init()
        .unwrap_or_else(|error|panic!("Error connecting to DB:{} ",error));
    rocket::ignite()
        .mount("/trucks", routes![
            controllers::trucks::add_truck,
            controllers::trucks::get_trucks,
            controllers::trucks::update_truck,
            controllers::trucks::get_truck,
            controllers::trucks::delete_truck,
            controllers::trucks::change_truck_state,
            controllers::trucks::end_route
        ])
        .mount("/packages", routes![
            controllers::packages::add_package,
            controllers::packages::get_all_packages,
            controllers::packages::get_package,
            controllers::packages::update_package,
            controllers::packages::delete_package
        ])
        .mount("/warehouses", routes![
            controllers::warehouses::add_warehouse,
            controllers::warehouses::get_all_warehouses,
            controllers::warehouses::get_warehouse,
            controllers::warehouses::delete_warehouse,
            controllers::warehouses::update_warehouse,
            controllers::warehouses::get_trucks_in_warehouse,
            controllers::warehouses::get_packages_in_warehouse,
            controllers::warehouses::create_routes
        ])
        .register(catchers!(
            controllers::catchers::not_found,
            controllers::catchers::unprocessable_entity,
            controllers::catchers::internal_err,
            controllers::catchers::bad_request
        ))
        .manage(database)
}