use mongodb::sync::Database;
use mongodb::bson;
use mongodb::bson::doc;
use crate::routes::responses::ApiResponse;
use crate::data::warehouse_models::Warehouse;

const COLLECTION:&str="warehouses";

pub fn get(conn: &Database, warehouse_id:i32) -> Result<Warehouse,ApiResponse>{
    super::get_one_from_db(conn.collection(COLLECTION), Some(doc!{"_id":warehouse_id}))
}
pub fn add(conn: &Database, warehouse:Warehouse) -> Result<Warehouse,ApiResponse>{
    let id=super::add_one_to_db(
        conn.collection(COLLECTION),
        bson::to_bson(&warehouse).unwrap().as_document().unwrap().to_owned())?;
    get(conn, id)
}
pub fn delete(conn: &Database, warehouse_id:i32) -> Result<i8,ApiResponse>{
    super::delete_from_db(conn.collection(COLLECTION), warehouse_id)
}