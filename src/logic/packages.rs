use crate::repositories::{packages, warehouses, DBConnection};
use crate::data::package_models::{Package, PackageRequest};
use crate::logic::errors::LogicError;

pub fn get_all_packages(conn:DBConnection)->Result<Vec<Package>,LogicError>{
    packages::get_all(&conn)
}
pub fn get_package(conn:DBConnection, id:i32)->Result<Package,LogicError>{
    packages::get(&conn, id )
}
pub fn add_package(conn:DBConnection, package:PackageRequest) ->Result<Package,LogicError>{
    if let Err(LogicError::NotFound) = warehouses::get(&conn,package.clone().warehouse){
        return Err(LogicError::InvalidWarehouse)
    }
    packages::add(&conn, Package::from_request(package))
}
pub fn update_package(conn:DBConnection, package:PackageRequest, id:i32)->Result<Package,LogicError>{
    if let Err(LogicError::NotFound) = warehouses::get(&conn,package.clone().warehouse){
        return Err(LogicError::InvalidWarehouse)
    }
    packages::update(&conn, Package::from_request(package), id as i32)
}
pub fn delete_package(conn:DBConnection, id:i32)->Result<i8,LogicError>{
    packages::delete(&conn, id)
}