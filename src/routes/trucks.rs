use rocket::http::Status;
use crate::database_configuration::DBConnection;
use crate::repositories::trucks;
use crate::data::truck_models::{TruckRequest, Truck};
use rocket_contrib::json::{JsonValue, Json};

#[get("/")]
pub fn get_trucks(conn:DBConnection)->Result<JsonValue,Status>{
    match trucks::get_all_trucks(&*conn){
        Ok(result)=>Ok(json!(result)),
        Err(err)=>Err(err)
    }
}
#[get("/<id>")]
pub fn get_truck(conn:DBConnection, id:usize)->Result<JsonValue,Status>{
    match trucks::get_truck(&*conn,id as i64){
        Ok(result)=>Ok(json!(result)),
        Err(err)=>Err(err)
    }
}
#[post("/", data="<truck>")]
pub fn add_truck(conn:DBConnection, truck:Json<TruckRequest>)->Result<JsonValue,Status>{
    match trucks::add_truck(&*conn,Truck::from_request(truck.into_inner())){
        Ok(result)=>Ok(json!(result)),
        Err(err)=>Err(err)
    }
}
#[put("/<id>")]
pub fn update_truck(conn:DBConnection, id:usize)->Result<String,Status>{
    Err(Status::NotFound)
}
#[delete("/<id>")]
pub fn delete_truck(conn:DBConnection, id:usize)->Result<JsonValue,Status>{
    match trucks::delete_truck(&*conn,id as i64){
        Ok(result)=>Ok(json!(result)),
        Err(err)=>Err(err)
    }
}
