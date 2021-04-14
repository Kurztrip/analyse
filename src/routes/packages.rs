use crate::repositories::DBConnection;
use crate::logic::packages;
use crate::data::package_models::PackageRequest;
use rocket_contrib::json::Json;
use crate::routes::responses::ApiResponse;
use rocket_contrib::json;

#[get("/")]
pub fn get_all_packages(conn:DBConnection)->ApiResponse{
    match packages::get_all_packages(conn){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[get("/<id>")]
pub fn get_package(conn:DBConnection, id:usize)->ApiResponse{
    match packages::get_package(conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[post("/", data="<package>")]
pub fn add_package(conn:DBConnection, package:Json<PackageRequest>) ->ApiResponse{
    match packages::add_package(conn, package.into_inner()){
        Ok(result)=>ApiResponse::created(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[put("/<id>",data="<package>")]
pub fn update_package(conn:DBConnection, package:Json<PackageRequest>, id:usize)->ApiResponse{
    match packages::update_package(conn, package.into_inner(), id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[delete("/<id>")]
pub fn delete_package(conn:DBConnection, id:usize)->ApiResponse{
    match packages::delete_package(conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}