use super::Coordinates;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Warehouse{
    id:i32,
    location:Coordinates
}

impl Warehouse {
    pub fn new(id: i32, location: Coordinates) -> Self {
        Warehouse { id, location }
    }
}

impl Warehouse {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn location(&self) -> &Coordinates {
        &self.location
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WarehouseRequest{
    id:i32,
    location:Coordinates
}
impl Warehouse{
    pub fn from_request(request:WarehouseRequest)->Warehouse {
        Warehouse{
            id:request.id,
            location:request.location
        }
    }
}