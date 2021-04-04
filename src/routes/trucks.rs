use crate::database_configuration::DBConnection;
use crate::repositories::{trucks, warehouses};
use crate::data::truck_models::{TruckRequest, Truck, State};
use rocket_contrib::json::Json;
use crate::routes::responses::ApiResponse;
use rocket::http::RawStr;

#[get("/")]
pub fn get_trucks(conn:DBConnection)->ApiResponse{
    match trucks::get_all(&*conn){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(_)=>ApiResponse::internal_err()
    }
}
#[get("/<id>")]
pub fn get_truck(conn:DBConnection, id:usize)->ApiResponse{
    match trucks::get(&*conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}
#[post("/", data="<truck>")]
pub fn add_truck(conn:DBConnection, truck:Json<TruckRequest>)->ApiResponse{
    if let Err(_) = warehouses::get(&*conn,truck.clone().warehouse){
        return ApiResponse::invalid_warehouse()
    }
    match trucks::add(&*conn, Truck::from_request(truck.into_inner())){
        Ok(result)=>ApiResponse::created(json!(result)),
        Err(err)=>err
    }
}
#[put("/<id>",data="<truck>")]
pub fn update_truck(conn:DBConnection, truck:Json<TruckRequest>, id:usize)->ApiResponse{
    if let Err(_) = warehouses::get(&*conn,truck.clone().warehouse){
        return ApiResponse::invalid_warehouse()
    }
    match trucks::update(&*conn, Truck::from_request(truck.into_inner()), id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}
#[put("/<id>/state?<state>")]
pub fn change_truck_state(conn:DBConnection, state:&RawStr, id:usize)->ApiResponse{
    match State::from_string(state){
        Ok(new_state)=>{
            match trucks::update_state(&*conn, new_state, id as i32){
                Ok(result)=>ApiResponse::ok(json!(result)),
                Err(err)=>err
            }
        },
        Err(e)=>ApiResponse::invalid_state(e)
    }

}
#[delete("/<id>")]
pub fn delete_truck(conn:DBConnection, id:usize)->ApiResponse{
    match trucks::delete(&*conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>err
    }
}
