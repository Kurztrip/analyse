use super::Coordinates;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package{
    id:i32,
    volume:f32,
    weight:f32,
    destination:Coordinates,
    receiver:String,
    sender:String,
    warehouse:i32
}

impl Package {
    pub fn new(id: i32, volume: f32, weight: f32, destination: Coordinates, receiver: String, sender: String, warehouse: i32) -> Self {
        Package { id, volume, weight, destination, receiver, sender, warehouse }
    }
}

impl Package {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn volume(&self) -> f32 {
        self.volume
    }
    pub fn weight(&self) -> f32 {
        self.weight
    }
    pub fn destination(&self) -> &Coordinates {
        &self.destination
    }
    pub fn receiver(&self) -> &str {
        &self.receiver
    }
    pub fn sender(&self) -> &str {
        &self.sender
    }
    pub fn warehouse(&self) -> i32 {
        self.warehouse
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageRequest {
    id:i32,
    volume:f32,
    weight:f32,
    destination:Coordinates,
    receiver:String,
    sender:String,
    pub warehouse:i32
}
impl Package{
    pub fn from_request(request: PackageRequest) ->Package {
        Package{
            id:request.id,
            volume:request.volume,
            weight:request.weight,
            destination:request.destination,
            receiver:request.receiver,
            sender:request.sender,
            warehouse:request.warehouse
        }
    }
}