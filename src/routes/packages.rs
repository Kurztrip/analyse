use crate::database_configuration::DBConnection;
use crate::repositories::{packages, warehouses};
use crate::data::package_models::{Package, PackageRequest};
use rocket_contrib::json::Json;
use crate::routes::responses::ApiResponse;

#[get("/<id>")]
pub fn get_package(conn:DBConnection, id:usize)->ApiResponse{
    match packages::get(&*conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}
#[post("/", data="<package>")]
pub fn add_package(conn:DBConnection, package:Json<PackageRequest>) ->ApiResponse{
    if let Err(_) = warehouses::get(&*conn,package.clone().warehouse){
        return ApiResponse::invalid_warehouse()
    }
    match packages::add(&*conn, Package::from_request(package.into_inner())){
        Ok(result)=>ApiResponse::created(json!(result)),
        Err(err)=>err
    }
}
#[put("/<id>",data="<package>")]
pub fn update_package(conn:DBConnection, package:Json<PackageRequest>, id:usize)->ApiResponse{
    if let Err(_) = warehouses::get(&*conn,package.clone().warehouse){
        return ApiResponse::invalid_warehouse()
    }
    match packages::update(&*conn, Package::from_request(package.into_inner()), id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}
#[delete("/<id>")]
pub fn delete_package(conn:DBConnection, id:usize)->ApiResponse{
    match packages::delete(&*conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}