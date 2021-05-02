use crate::repositories::DBConnection;
use rocket_contrib::json::Json;
use crate::controllers::responses::ApiResponse;
use crate::data::warehouse_models::WarehouseRequest;
use crate::logic::{warehouses,routes};
use rocket_contrib::json;
use crate::data::truck_models::TruckRequest;

#[get("/<id>")]
pub fn get_warehouse(conn:DBConnection, id:usize)->ApiResponse{
    match warehouses::get_warehouse(conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[get("/")]
pub fn get_all_warehouses(conn:DBConnection)->ApiResponse{
    match warehouses::get_all_warehouses(conn){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[get("/<id>/trucks")]
pub fn get_trucks_in_warehouse(conn:DBConnection, id:usize)->ApiResponse{
    match warehouses::get_trucks_in_warehouse(conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[get("/<id>/packages")]
pub fn get_packages_in_warehouse(conn:DBConnection, id:usize)->ApiResponse{
    match warehouses::get_packages_in_warehouse(conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[post("/", data="<warehouse>")]
pub fn add_warehouse(conn:DBConnection, warehouse:Json<WarehouseRequest>)->ApiResponse{
    match warehouses::add_warehouse(conn, warehouse.into_inner()){
        Ok(result)=>ApiResponse::created(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[put("/<id>",data="<warehouse>")]
pub fn update_warehouse(conn:DBConnection, warehouse:Json<WarehouseRequest>, id:usize)->ApiResponse{
    match warehouses::update_warehouse(conn, warehouse.into_inner(), id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[delete("/<id>")]
pub fn delete_warehouse(conn:DBConnection, id:usize)->ApiResponse{
    match warehouses::delete_warehouse(conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[post("/<id>/create-route", data="<truck>")]
pub fn create_routes(conn:DBConnection, id:usize, truck:Json<TruckRequest>)->ApiResponse{
    match routes::generate_route(&conn,id as i32, truck.into_inner()){
        Ok(truck)=>ApiResponse::ok(json!(truck)),
        Err(err)=>ApiResponse::from(err)
    }
}