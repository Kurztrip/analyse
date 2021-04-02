use super::Coordinates;
use serde::{Serialize,Deserialize};
// use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package{
    id:i32,
    height:f32,
    length:f32,
    width:f32,
    destination:Coordinates,
    receiver:String,
    sender:String
}