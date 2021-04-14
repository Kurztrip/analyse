use crate::data::truck_models::{Truck, State};
use crate::repositories::DBConnection;
use crate::data::package_models::Package;
use crate::logic::errors::LogicError;
use super::serde_json::{Value, Map, Number};
use super::serde_json::json;
use crate::repositories::database_models::DatabaseTruck;


const COLLECTION:&str="trucks";

pub fn get_all(conn: &DBConnection) ->Result<Vec<Truck>,LogicError>{
    let result :Vec<DatabaseTruck>= conn.get_many_from_db(COLLECTION, None)?;
    Ok(result.iter().map(|p|p.to_owned().into()).collect())
}

pub fn get_all_from_warehouse(conn: &DBConnection, warehouse_id:i32) ->Result<Vec<Truck>,LogicError>{
    let mut filter = Map::new();
    filter.insert("warehouse".to_string(),Value::Number(Number::from(warehouse_id)));
    let result : Vec<DatabaseTruck>=conn.get_many_from_db(COLLECTION, Some(filter))?;
    Ok(result.iter().map(|p|p.to_owned().into()).collect())
}

pub fn add(conn: &DBConnection, truck:Truck) ->Result<Truck,LogicError>{
    let id = conn.add_one_to_db(
        COLLECTION,
        DatabaseTruck::from(truck))?;
    get(conn, id)
}
pub fn update(conn: &DBConnection, truck:Truck, truck_id:i32) ->Result<Truck,LogicError>{
    let result= conn.replace_in_db(
        COLLECTION,
        truck_id,
        DatabaseTruck::from(truck)
    )?;
    Ok(result.into())
}
pub fn update_state(conn: &DBConnection, state:State, truck_id:i32) ->Result<Truck,LogicError>{
    let mut update = Map::new();
    update.insert("state".to_string(),json!(state));
    let result:DatabaseTruck = conn.update_in_db(
        COLLECTION,
        truck_id,
        update
    )?;
    Ok(result.into())
}
pub fn add_to_route(conn: &DBConnection, package:Package, truck_id:i32) ->Result<Truck,LogicError>{
    let mut update = Map::new();
    update.insert("route".to_string(),json!(&package.destination()));
    update.insert("packages".to_string(),json!(&package));
    let result : DatabaseTruck=conn.add_to_array(
        COLLECTION,
        truck_id,
        update
    )?;
    Ok(result.into())
}
pub fn get(conn: &DBConnection, truck_id:i32) ->Result<Truck,LogicError>{
    let mut filter = Map::new();
    filter.insert("id".to_string(),Value::Number(Number::from(truck_id)));
    let result :DatabaseTruck= conn.get_one_from_db(COLLECTION, Some(filter))?;
    Ok(result.into())

}
pub fn delete(conn: &DBConnection, truck_id:i32) ->Result<i8,LogicError> {
    conn.delete_from_db(COLLECTION, truck_id)
}
pub fn delete_all_from_warehouse(conn: &DBConnection, warehouse_id:i32) ->Result<i8,LogicError>{
    let mut filter = Map::new();
    filter.insert("warehouse".to_string(),json!(warehouse_id));
    conn.delete_many_from_db(COLLECTION, filter)
}