use super::Coordinates;
use super::package_models::Package;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Truck{
    #[serde(rename = "_id")]
    pub id:i32,
    packages: Vec<Package>,
    route: Vec<Coordinates>,
    fuel:f32,
    fuel_capacity:f32,
    fuel_type:FuelType,
    fuel_by_kilometer:f32,
    weight_capacity:f32,
    volume_capacity:f32,
    state:State,
    warehouse:i32
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TruckRequest{
    id:i32,
    fuel_capacity:f32,
    fuel_type:FuelType,
    fuel_by_kilometer:f32,
    weight_capacity:f32,
    volume_capacity:f32,
    pub warehouse:i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FuelType{
    Diesel,
    Gasoline,
    NaturalGas
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum State{
    Available,
    Maintenance,
    Unavailable
}
impl State{
    pub fn from_string(state:&str)->Result<State,String>{
        match state{
            "Available"=>Ok(State::Available),
            "Maintenance"=>Ok(State::Maintenance),
            "Unavailable"=>Ok(State::Unavailable),
            _=>Err(format!("{} is not a state",state))
        }
    }
}
impl Truck{
    pub fn from_request(request:TruckRequest)->Truck{
        Truck{
            id:request.id,
            packages:vec![],
            route:vec![],
            fuel:request.fuel_capacity,
            fuel_capacity:request.fuel_capacity,
            fuel_type:request.fuel_type,
            fuel_by_kilometer:request.fuel_by_kilometer,
            weight_capacity:request.weight_capacity,
            volume_capacity:request.volume_capacity,
            state:State::Available,
            warehouse:request.warehouse
        }
    }
}