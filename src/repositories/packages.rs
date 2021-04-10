use crate::data::package_models::Package;
use mongodb::bson;
use mongodb::bson::doc;
use crate::routes::responses::ApiResponse;
use crate::repositories::DBConnection;

const COLLECTION:&str="packages";

pub fn add(conn: &DBConnection, package:Package) ->Result<Package,ApiResponse> {
    let id = conn.add_one_to_db(
        COLLECTION,
        bson::to_bson(&package).unwrap().as_document().unwrap().to_owned())?;
    get(conn, id)
}
pub fn get_all_from_warehouse(conn: &DBConnection, warehouse_id:i32) ->Result<Vec<Package>,ApiResponse>{
    conn.get_many_from_db(COLLECTION, Some(doc!{"warehouse":warehouse_id}))
}
pub fn get_all(conn: &DBConnection) ->Result<Vec<Package>,ApiResponse>{
    conn.get_many_from_db(COLLECTION,None)
}
pub fn get(conn: &DBConnection, package_id:i32) -> Result<Package,ApiResponse>{
    conn.get_one_from_db(COLLECTION, Some(doc!{"_id":package_id}))
}
pub fn update(conn: &DBConnection, package:Package, package_id:i32) ->Result<Package,ApiResponse>{
    conn.replace_in_db(
        COLLECTION,
        package_id,
        bson::to_bson(&package).unwrap().as_document().unwrap().to_owned()
    )
}
pub fn delete(conn: &DBConnection, package_id:i32) -> Result<i8,ApiResponse>{
    conn.delete_from_db(COLLECTION, package_id)
}