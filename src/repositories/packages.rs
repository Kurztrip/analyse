use crate::data::package_models::Package;
use mongodb::sync::Database;
use mongodb::bson;
use mongodb::bson::doc;
use crate::routes::responses::ApiResponse;

const COLLECTION:&str="packages";

pub fn add(conn: &Database, package:Package) ->Result<Package,ApiResponse> {
    let id = super::add_one_to_db(
        conn.collection(COLLECTION),
        bson::to_bson(&package).unwrap().as_document().unwrap().to_owned())?;
    get(conn, id)
}
pub fn get_all_from_warehouse(conn: &Database, warehouse_id:i32) ->Result<Vec<Package>,ApiResponse>{
    super::get_many_from_db(conn.collection(COLLECTION), Some(doc!{"warehouse":warehouse_id}))
}
pub fn get_all(conn: &Database) ->Result<Vec<Package>,ApiResponse>{
    super::get_many_from_db(conn.collection(COLLECTION),None)
}
pub fn get(conn: &Database, package_id:i32) -> Result<Package,ApiResponse>{
    super::get_one_from_db(conn.collection(COLLECTION), Some(doc!{"_id":package_id}))
}
pub fn update(conn: &Database, package:Package, package_id:i32) ->Result<Package,ApiResponse>{
    super::replace_in_db(
        conn.collection(COLLECTION),
        package_id,
        bson::to_bson(&package).unwrap().as_document().unwrap().to_owned()
    )
}
pub fn delete(conn: &Database, package_id:i32) -> Result<i8,ApiResponse>{
    super::delete_from_db(conn.collection(COLLECTION), package_id)
}