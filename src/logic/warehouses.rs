use crate::repositories::{warehouses, trucks, packages, DBConnection};
use crate::data::warehouse_models::{WarehouseRequest, Warehouse};
use crate::logic::errors::LogicError;
use crate::data::truck_models::Truck;
use crate::data::package_models::Package;


pub fn get_warehouse(conn:DBConnection, id:i32)->Result<Warehouse,LogicError>{
    warehouses::get(&conn, id as i32)
}
pub fn get_all_warehouses(conn:DBConnection)->Result<Vec<Warehouse>,LogicError>{
    warehouses::get_all(&conn)
}
pub fn get_trucks_in_warehouse(conn:DBConnection, id:i32)->Result<Vec<Truck>,LogicError>{
    trucks::get_all_from_warehouse(&conn, id)
}
pub fn get_packages_in_warehouse(conn:DBConnection, id:i32)->Result<Vec<Package>,LogicError>{
    packages::get_all_from_warehouse(&conn, id)
}
pub fn add_warehouse(conn:DBConnection, warehouse:WarehouseRequest)->Result<Warehouse,LogicError>{
    warehouses::add(&conn, Warehouse::from_request(warehouse))
}
pub fn update_warehouse(conn:DBConnection, warehouse:WarehouseRequest, id:i32)->Result<Warehouse,LogicError>{
    warehouses::update(&conn, Warehouse::from_request(warehouse), id )
}
pub fn delete_warehouse(conn:DBConnection, id:i32)->Result<i8,LogicError>{
    warehouses::delete(&conn, id)
}