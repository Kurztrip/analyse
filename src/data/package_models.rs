use super::Coordinates;
use serde::{Serialize,Deserialize};
// use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package{
    #[serde(rename = "_id")]
    id:i32,
    volume:f32,
    destination:Coordinates,
    receiver:String,
    sender:String,
    warehouse:i32
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestPackage{
    id:i32,
    volume:f32,
    destination:Coordinates,
    receiver:String,
    sender:String,
    warehouse:i32
}
impl Package{
    pub fn from_request(request:RequestPackage)->Package {
        Package{
            id:request.id,
            volume:request.volume,
            destination:request.destination,
            receiver:request.receiver,
            sender:request.sender,
            warehouse:request.warehouse
        }
    }
}