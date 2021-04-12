use mongodb::bson;
use mongodb::bson::doc;
use crate::data::warehouse_models::Warehouse;
use crate::repositories::DBConnection;
use crate::logic::errors::LogicError;

const COLLECTION:&str="warehouses";

pub fn get(conn: &DBConnection, warehouse_id:i32) -> Result<Warehouse,LogicError>{
    conn.get_one_from_db(COLLECTION, Some(doc!{"_id":warehouse_id}))
}
pub fn get_all(conn: &DBConnection) ->Result<Vec<Warehouse>,LogicError>{
    conn.get_many_from_db(COLLECTION,None)
}
pub fn add(conn: &DBConnection, warehouse:Warehouse) -> Result<Warehouse,LogicError>{
    let id=conn.add_one_to_db(
        COLLECTION,
        bson::to_bson(&warehouse).unwrap().as_document().unwrap().to_owned())?;
    get(conn, id)
}
pub fn update(conn: &DBConnection, warehouse:Warehouse, warehouse_id:i32) ->Result<Warehouse,LogicError>{
    conn.replace_in_db(
        COLLECTION,
        warehouse_id,
        bson::to_bson(&warehouse).unwrap().as_document().unwrap().to_owned()
    )
}
pub fn delete(conn: &DBConnection, warehouse_id:i32) -> Result<i8,LogicError>{
    match conn.delete_from_db(COLLECTION, warehouse_id){
        Ok(_)=>super::trucks::delete_all_from_warehouse(conn,warehouse_id),
        Err(e)=>Err(e)
    }

}