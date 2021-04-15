use super::Coordinates;
use super::package_models::Package;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Truck{
    id:i32,
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

impl Truck {
    pub fn new(id: i32, packages: Vec<Package>, route: Vec<Coordinates>, fuel: f32, fuel_capacity: f32, fuel_type: FuelType, fuel_by_kilometer: f32, weight_capacity: f32, volume_capacity: f32, state: State, warehouse: i32) -> Self {
        Truck { id, packages, route, fuel, fuel_capacity, fuel_type, fuel_by_kilometer, weight_capacity, volume_capacity, state, warehouse }
    }
}

impl Truck {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn packages(&self) -> &Vec<Package> {
        &self.packages
    }
    pub fn route(&self) -> &Vec<Coordinates> {
        &self.route
    }
    pub fn fuel(&self) -> f32 {
        self.fuel
    }
    pub fn fuel_capacity(&self) -> f32 {
        self.fuel_capacity
    }
    pub fn fuel_type(&self) -> &FuelType {
        &self.fuel_type
    }
    pub fn fuel_by_kilometer(&self) -> f32 {
        self.fuel_by_kilometer
    }
    pub fn weight_capacity(&self) -> f32 {
        self.weight_capacity
    }
    pub fn volume_capacity(&self) -> f32 {
        self.volume_capacity
    }
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn warehouse(&self) -> i32 {
        self.warehouse
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TruckRequest{
    id:i32,
    fuel_capacity:f32,
    fuel_type:FuelType,
    fuel_by_kilometer:f32,
    weight_capacity:f32,
    volume_capacity:f32,
    warehouse:i32
}

impl TruckRequest {
    pub fn warehouse(&self) -> i32 {
        self.warehouse
    }
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