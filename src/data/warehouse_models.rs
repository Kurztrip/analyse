use super::Coordinates;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Warehouse{
    #[serde(rename = "_id")]
    id:i32,
    name:String,
    location:Coordinates
}

impl Warehouse {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn location(&self) -> &Coordinates {
        &self.location
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WarehouseRequest{
    id:i32,
    name:String,
    location:Coordinates
}
impl Warehouse{
    pub fn from_request(mut request:WarehouseRequest)->Warehouse {
        request.location.des = true;
        Warehouse{
            id:request.id,
            name:request.name,
            location:request.location
        }
    }
}