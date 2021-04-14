pub mod truck_models;
pub mod package_models;
pub mod warehouse_models;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Coordinates(pub f64, pub f64);
