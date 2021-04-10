pub mod truck_models;
pub mod package_models;
pub mod warehouse_models;
use serde::{Serialize, Deserialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Deserialize, Debug, Clone)]
pub struct Coordinates {
    #[serde(rename = "type")]
    #[serde(skip_deserializing)]
    obj_type:String,
    coordinates:[f64;2],
    #[serde(skip)]
    pub des:bool
}
impl Serialize for Coordinates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("Coordinates", 2)?;
        if self.des {
            state.serialize_field("type", &self.obj_type)?;
        }
        state.serialize_field("coordinates", &self.coordinates)?;
        state.end()
    }
}

impl Default for Coordinates{
    fn default()->Self{
        Coordinates{
            obj_type: "Point".to_string(),
            coordinates: [0.0,0.0],
            des:false
        }
    }
}
