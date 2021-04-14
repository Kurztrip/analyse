use crate::repositories::DBConnection;
use crate::data::truck_models::TruckRequest;
use crate::logic::trucks;
use rocket_contrib::json::Json;
use crate::controllers::responses::ApiResponse;
use rocket::http::RawStr;
use rocket_contrib::json;
#[get("/")]
pub fn get_trucks(conn:DBConnection)->ApiResponse{
    match trucks::get_trucks(conn){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(_)=>ApiResponse::internal_err()
    }
}
#[get("/<id>")]
pub fn get_truck(conn:DBConnection, id:usize)->ApiResponse{
    match trucks::get_truck(conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[post("/", data="<truck>")]
pub fn add_truck(conn:DBConnection, truck:Json<TruckRequest>)->ApiResponse{
    match trucks::add_truck(conn, truck.into_inner()){
        Ok(result)=>ApiResponse::created(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[put("/<id>",data="<truck>")]
pub fn update_truck(conn:DBConnection, truck:Json<TruckRequest>, id:usize)->ApiResponse{
    match trucks::update_truck(conn, truck.into_inner(), id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[put("/<id>/state?<state>")]
pub fn change_truck_state(conn:DBConnection, state:&RawStr, id:usize)->ApiResponse{
    match trucks::change_truck_state(conn, state.to_string(), id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[delete("/<id>")]
pub fn delete_truck(conn:DBConnection, id:usize)->ApiResponse{
    match trucks::delete_truck(conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
#[put("/<id>/end-route")]
pub fn end_route(conn:DBConnection, id:usize)->ApiResponse{
    match trucks::end_route(conn, id as i32){
        Ok(result)=>ApiResponse::ok(json!(result)),
        Err(err)=>ApiResponse::from(err)
    }
}
