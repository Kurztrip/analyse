use crate::data::truck_models::{FuelType, State, Truck};
use crate::data::warehouse_models::Warehouse;
use crate::data::Coordinates;
use crate::data::package_models::Package;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseWarehouse{
    #[serde(rename = "_id")]
    id:i32,
    location:DatabaseCoordinates
}

impl DatabaseWarehouse {
    pub fn new(id: i32, location: DatabaseCoordinates) -> Self {
        DatabaseWarehouse { id, location }
    }
}

impl From<Warehouse> for DatabaseWarehouse{
    fn from(warehouse:Warehouse)->Self{
        Self::new(warehouse.id(),
                  DatabaseCoordinates::from(warehouse.location().to_owned()))
    }
}
impl From<DatabaseWarehouse> for Warehouse{
    fn from(warehouse:DatabaseWarehouse)->Self{
        Warehouse::new(
            warehouse.id,
            warehouse.location.into()
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseCoordinates {
    #[serde(rename = "type")]
    obj_type:String,
    coordinates:[f64;2]
}

impl DatabaseCoordinates {
    pub fn new(coordinates: [f64; 2]) -> Self {
        DatabaseCoordinates {
            obj_type: String::from("Point"),
            coordinates }
    }
}
impl From<Coordinates> for DatabaseCoordinates{
    fn from(coordinates:Coordinates)->Self{
        DatabaseCoordinates::new([
            coordinates.0,
            coordinates.1])
    }
}
impl From<DatabaseCoordinates> for Coordinates{
    fn from(coordinates:DatabaseCoordinates)->Self{
        Coordinates(
            coordinates.coordinates[0],
            coordinates.coordinates[1])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseTruck{
    #[serde(rename = "_id")]
    id:i32,
    packages: Vec<DatabasePackage>,
    route: Vec<DatabaseCoordinates>,
    fuel:f32,
    fuel_capacity:f32,
    fuel_type:FuelType,
    fuel_by_kilometer:f32,
    weight_capacity:f32,
    volume_capacity:f32,
    state:State,
    warehouse:i32
}

impl DatabaseTruck {
    pub fn new(id: i32, packages: Vec<DatabasePackage>, route: Vec<DatabaseCoordinates>, fuel: f32, fuel_capacity: f32, fuel_type: FuelType, fuel_by_kilometer: f32, weight_capacity: f32, volume_capacity: f32, state: State, warehouse: i32) -> Self {
        DatabaseTruck {
            id,
            packages,
            route,
            fuel,
            fuel_capacity,
            fuel_type,
            fuel_by_kilometer,
            weight_capacity,
            volume_capacity,
            state,
            warehouse
        }
    }
}
impl From<Truck> for DatabaseTruck{
    fn from(truck:Truck)->Self{
        Self::new(
            truck.id(),
            truck.packages().iter().map(|package|DatabasePackage::from(package.to_owned())).collect(),
            truck.route().iter().map(|coordinates|DatabaseCoordinates::from(coordinates.to_owned())).collect(),
            truck.fuel(),
            truck.fuel_capacity(),
            truck.fuel_type().to_owned(),
            truck.fuel_by_kilometer(),
            truck.weight_capacity(),
            truck.volume_capacity(),
            truck.state().to_owned(),
            truck.warehouse()
        )
    }
}
impl From<DatabaseTruck> for Truck{
    fn from(truck:DatabaseTruck)->Self{
        Self::new(
            truck.id,
            truck.packages.iter().map(|package|package.to_owned().into()).collect(),
            truck.route.iter().map(|coordinates|coordinates.to_owned().into()).collect(),
            truck.fuel,
            truck.fuel_capacity,
            truck.fuel_type.to_owned(),
            truck.fuel_by_kilometer,
            truck.weight_capacity,
            truck.volume_capacity,
            truck.state.to_owned(),
            truck.warehouse
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabasePackage{
    #[serde(rename = "_id")]
    id:i32,
    volume:f32,
    weight:f32,
    destination:DatabaseCoordinates,
    warehouse:i32
}

impl DatabasePackage {
    pub fn new(id: i32, volume: f32, weight: f32, destination: DatabaseCoordinates, warehouse: i32) -> Self {
        DatabasePackage {
            id,
            volume,
            weight,
            destination,
            warehouse
        }
    }
}
impl From<Package> for DatabasePackage{
    fn from(package: Package) -> Self {
        DatabasePackage::new(
            package.id(),
            package.volume(),
            package.weight(),
            package.destination().to_owned().into(),
            package.warehouse()
        )
    }
}
impl From<DatabasePackage> for Package{
    fn from(package: DatabasePackage) -> Self {
        Package::new(
            package.id,
            package.volume,
            package.weight,
            package.destination.into(),
            package.warehouse
        )
    }
}