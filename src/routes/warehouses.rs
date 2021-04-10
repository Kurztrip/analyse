use crate::repositories::{warehouses, trucks, packages, DBConnection};
use rocket_contrib::json::Json;
use crate::routes::responses::ApiResponse;
use crate::data::warehouse_models::{WarehouseRequest, Warehouse};

#[get("/<id>")]
pub fn get_warehouse(conn:DBConnection, id:usize)->ApiResponse{
    match warehouses::get(&conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}
#[get("/")]
pub fn get_all_warehouses(conn:DBConnection)->ApiResponse{
    match warehouses::get_all(&conn){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}
#[get("/<id>/trucks")]
pub fn get_trucks_in_warehouse(conn:DBConnection, id:usize)->ApiResponse{
    match trucks::get_all_from_warehouse(&conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}
#[get("/<id>/packages")]
pub fn get_packages_in_warehouse(conn:DBConnection, id:usize)->ApiResponse{
    match packages::get_all_from_warehouse(&conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}
#[post("/", data="<warehouse>")]
pub fn add_warehouse(conn:DBConnection, warehouse:Json<WarehouseRequest>)->ApiResponse{
    match warehouses::add(&conn, Warehouse::from_request(warehouse.into_inner())){
        Ok(result)=>ApiResponse::created(json!(result)),
        Err(err)=>err
    }
}
#[put("/<id>",data="<warehouse>")]
pub fn update_warehouse(conn:DBConnection, warehouse:Json<WarehouseRequest>, id:usize)->ApiResponse{
    match warehouses::update(&conn, Warehouse::from_request(warehouse.into_inner()), id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}
#[delete("/<id>")]
pub fn delete_warehouse(conn:DBConnection, id:usize)->ApiResponse{
    match warehouses::delete(&conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}