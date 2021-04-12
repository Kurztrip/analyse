use crate::repositories::{trucks, warehouses, DBConnection};
use crate::data::truck_models::{TruckRequest, Truck, State};
use crate::logic::errors::LogicError;

pub fn get_trucks(conn:DBConnection)->Result<Vec<Truck>,LogicError>{
    trucks::get_all(&conn)
}
pub fn get_truck(conn:DBConnection, id:i32)->Result<Truck,LogicError>{
    trucks::get(&conn, id )
}
pub fn add_truck(conn:DBConnection, truck:TruckRequest)->Result<Truck,LogicError>{
    if let Err(LogicError::NotFound) = warehouses::get(&conn,truck.clone().warehouse){
        return Err(LogicError::InvalidWarehouse)
    }
    trucks::add(&conn, Truck::from_request(truck))
}
pub fn update_truck(conn:DBConnection, truck:TruckRequest, id:i32)->Result<Truck,LogicError>{
    if let Err(LogicError::NotFound) = warehouses::get(&conn,truck.clone().warehouse){
        return Err(LogicError::InvalidWarehouse)
    }
    trucks::update(&conn, Truck::from_request(truck), id)
}
pub fn change_truck_state(conn:DBConnection, state:String, id:i32)->Result<Truck,LogicError>{
    match State::from_string(&state){
        Ok(new_state)=>{
            trucks::update_state(&conn, new_state, id)
        },
        Err(e)=>Err(LogicError::InvalidState(e))
    }
}
pub fn delete_truck(conn:DBConnection, id:i32)->Result<i8,LogicError>{
    trucks::delete(&conn, id)
}