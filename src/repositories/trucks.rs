use crate::data::truck_models::{Truck, State};
use mongodb::bson;
use mongodb::bson::doc;
use crate::routes::responses::ApiResponse;
use crate::repositories::DBConnection;


const COLLECTION:&str="trucks";

pub fn get_all(conn: &DBConnection) ->Result<Vec<Truck>,ApiResponse>{
    conn.get_many_from_db(COLLECTION, None)
}

pub fn get_all_from_warehouse(conn: &DBConnection, warehouse_id:i32) ->Result<Vec<Truck>,ApiResponse>{
    conn.get_many_from_db(COLLECTION, Some(doc!{"warehouse":warehouse_id}))
}

pub fn add(conn: &DBConnection, truck:Truck) ->Result<Truck,ApiResponse>{
    let id = conn.add_one_to_db(
        COLLECTION,
        bson::to_bson(&truck).unwrap().as_document().unwrap().to_owned())?;
    get(conn, id)
}
pub fn update(conn: &DBConnection, truck:Truck, truck_id:i32) ->Result<Truck,ApiResponse>{
    conn.replace_in_db(
        COLLECTION,
        truck_id,
        bson::to_bson(&truck).unwrap().as_document().unwrap().to_owned()
    )
}
pub fn update_state(conn: &DBConnection, state:State, truck_id:i32) ->Result<Truck,ApiResponse>{
    conn.update_in_db(
        COLLECTION,
        truck_id,
        doc!{"$set":{"state":bson::to_bson(&state).unwrap()}}
    )
}
pub fn get(conn: &DBConnection, truck_id:i32) ->Result<Truck,ApiResponse>{
    conn.get_one_from_db(COLLECTION, Some(doc!{"_id":truck_id}))
}
pub fn delete(conn: &DBConnection, truck_id:i32) ->Result<i8,ApiResponse> {
    conn.delete_from_db(COLLECTION, truck_id)
}
pub fn delete_all_from_warehouse(conn: &DBConnection, warehouse_id:i32) ->Result<i8,ApiResponse>{
    conn.delete_many_from_db(COLLECTION, doc!{"warehouse":warehouse_id})
}