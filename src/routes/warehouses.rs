use crate::database_configuration::DBConnection;
use crate::repositories::warehouses;
use rocket_contrib::json::Json;
use crate::routes::responses::ApiResponse;
use crate::data::warehouse_models::{WarehouseRequest, Warehouse};

#[get("/<id>")]
pub fn get_warehouse(conn:DBConnection, id:usize)->ApiResponse{
    match warehouses::get(&*conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}
#[post("/", data="<warehouse>")]
pub fn add_warehouse(conn:DBConnection, warehouse:Json<WarehouseRequest>)->ApiResponse{
    match warehouses::add(&*conn, Warehouse::from_request(warehouse.into_inner())){
        Ok(result)=>ApiResponse::created(json!(result)),
        Err(err)=>err
    }
}
#[delete("/<id>")]
pub fn delete_warehouse(conn:DBConnection, id:usize)->ApiResponse{
    match warehouses::delete(&*conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}