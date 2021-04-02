pub mod truck_models;
pub mod package_models;
pub mod warehouse_models;
use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coordinates {
    latitude:f32,
    longitude:f32
}