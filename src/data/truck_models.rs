use super::Coordinates;
use super::package_models::Package;
use serde::{Serialize,Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Truck{
    #[serde(rename = "_id")]
    id:i32,
    packages: Vec<Package>,
    route: Vec<Coordinates>,
    fuel:f32,
    fuel_capacity:f32,
    fuel_type:FuelType
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TruckRequest{
    id:i32,
    fuel_capacity:f32,
    fuel_type:FuelType
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FuelType{
    Diesel,
    Gasoline,
    NaturalGas
}

impl Truck{
    pub fn from_request(request:TruckRequest)->Truck{
        Truck{
            id:request.id,
            packages:vec![],
            route:vec![],
            fuel:request.fuel_capacity,
            fuel_capacity:request.fuel_capacity,
            fuel_type:request.fuel_type
        }
    }
}