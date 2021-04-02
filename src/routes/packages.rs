use crate::database_configuration::DBConnection;
use crate::repositories::packages;
use crate::data::package_models::{Package,RequestPackage};
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
pub fn add_package(conn:DBConnection, package:Json<RequestPackage>)->ApiResponse{
    match packages::add(&*conn, Package::from_request(package.into_inner())){
        Ok(result)=>ApiResponse::created(json!(result)),
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