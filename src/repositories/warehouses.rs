use crate::data::warehouse_models::Warehouse;
use crate::repositories::DBConnection;
use crate::logic::errors::LogicError;
use super::serde_json::{Value, Map, Number};
use crate::repositories::database_models::DatabaseWarehouse;

const COLLECTION:&str="warehouses";

pub fn get(conn: &DBConnection, warehouse_id:i32) -> Result<Warehouse,LogicError>{
    let mut filter = Map::new();
    filter.insert("id".to_string(),Value::Number(Number::from(warehouse_id)));
    let result:DatabaseWarehouse = conn.get_one_from_db(COLLECTION, Some(filter))?;
    Ok(result.into())
}
pub fn get_all(conn: &DBConnection) ->Result<Vec<Warehouse>,LogicError>{
    let result:Vec<DatabaseWarehouse> = conn.get_many_from_db(COLLECTION,None)?;
    Ok(result.iter().map(|p|p.to_owned().into()).collect())
}
pub fn add(conn: &DBConnection, warehouse:Warehouse) -> Result<Warehouse,LogicError>{
    let id=conn.add_one_to_db(
        COLLECTION,
        DatabaseWarehouse::from(warehouse))?;
    get(conn, id)
}
pub fn update(conn: &DBConnection, warehouse:Warehouse, warehouse_id:i32) ->Result<Warehouse,LogicError>{
    let result = conn.replace_in_db(
        COLLECTION,
        warehouse_id,
        DatabaseWarehouse::from(warehouse)
    )?;
    Ok(result.into())
}
pub fn delete(conn: &DBConnection, warehouse_id:i32) -> Result<i8,LogicError>{
    match conn.delete_from_db(COLLECTION, warehouse_id){
        Ok(_)=>super::trucks::delete_all_from_warehouse(conn,warehouse_id),
        Err(e)=>Err(e)
    }

}