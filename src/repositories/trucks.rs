use crate::data::truck_models::{Truck, State};
use mongodb::sync::Database;
use mongodb::bson;
use mongodb::bson::doc;
use crate::routes::responses::ApiResponse;


const COLLECTION:&str="trucks";

pub fn get_all(conn: &Database) ->Result<Vec<Truck>,ApiResponse>{
    super::get_many_from_db(conn.collection(COLLECTION), None)
}

pub fn get_all_from_warehouse(conn: &Database, warehouse_id:i32) ->Result<Vec<Truck>,ApiResponse>{
    super::get_many_from_db(conn.collection(COLLECTION), Some(doc!{"warehouse":warehouse_id}))
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
pub fn update_state(conn: &Database, state:State, truck_id:i32) ->Result<Truck,ApiResponse>{
    super::update_in_db(
        conn.collection(COLLECTION),
        truck_id,
        doc!{"$set":{"state":bson::to_bson(&state).unwrap()}}
    )
}
pub fn get(conn: &Database, truck_id:i32) ->Result<Truck,ApiResponse>{
    super::get_one_from_db(conn.collection(COLLECTION), Some(doc!{"_id":truck_id}))
}
pub fn delete(conn: &Database, truck_id:i32) ->Result<i8,ApiResponse> {
    super::delete_from_db(conn.collection(COLLECTION), truck_id)
}
pub fn delete_all_from_warehouse(conn: &Database, warehouse_id:i32) ->Result<i8,ApiResponse>{
    super::delete_many_from_db(conn.collection(COLLECTION), doc!{"warehouse":warehouse_id})
}