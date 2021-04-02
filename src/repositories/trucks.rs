use crate::data::truck_models::Truck;
use mongodb::sync::Database;
use mongodb::bson;
use mongodb::bson::doc;
use crate::routes::responses::ApiResponse;


const COLLECTION:&str="trucks";

pub fn get_all(conn: &Database) ->Result<Vec<Truck>,ApiResponse>{
    super::get_many_from_db(conn.collection(COLLECTION), None)
}

pub fn add(conn: &Database, truck:Truck) ->Result<Truck,ApiResponse>{
    let id = super::add_one_to_db(
        conn.collection(COLLECTION),
        bson::to_bson(&truck).unwrap().as_document().unwrap().to_owned())?;
    get(conn, id)
}
pub fn update(conn: &Database, truck:Truck, truck_id:i32) ->Result<Truck,ApiResponse>{
    super::replace_in_db(
        conn.collection(COLLECTION),
        truck_id,
        bson::to_bson(&truck).unwrap().as_document().unwrap().to_owned()
    )
}
pub fn get(conn: &Database, truck_id:i32) ->Result<Truck,ApiResponse>{
    super::get_one_from_db(conn.collection(COLLECTION), Some(doc!{"_id":truck_id}))
}
pub fn delete(conn: &Database, truck_id:i32) ->Result<i8,ApiResponse> {
    super::delete_from_db(conn.collection(COLLECTION), truck_id)
}