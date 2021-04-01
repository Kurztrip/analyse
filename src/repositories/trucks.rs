use rocket::http::Status;
use crate::data::truck_models::Truck;
use mongodb::sync::Database;
use mongodb::bson;
use mongodb::bson::doc;


const COLLECTION:&str="trucks";

pub fn get_all_trucks(conn: &Database)->Result<Vec<Truck>,Status>{
    match conn.collection(COLLECTION).find(None,None){
       Ok(cursor)=>{
           cursor.map(|result|match result{
               Ok(doc)=>{
                   Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
               },
               Err(_)=>Err(Status::InternalServerError)
           }).collect::<Result<Vec<Truck>,Status>>()
       },
        Err(_)=>Err(Status::NotFound)
    }
}

pub fn add_truck(conn: &Database,truck:Truck)->Result<Truck,Status>{
    let collection = conn.collection(COLLECTION);
    match collection.insert_one(
            bson::to_bson(&truck).unwrap().as_document().unwrap().to_owned()
            ,None){
        Ok(result)=>{
            get_truck(conn,bson::from_bson(result.inserted_id).unwrap())
        },
        Err(_)=>Err(Status::InternalServerError)
    }
}
pub fn get_truck(conn: &Database,truck_id:i64)->Result<Truck,Status>{
    match conn.collection(COLLECTION).find_one(Some(doc!{"_id":truck_id}),None){
        Ok(cursor)=>{
            if let Some(doc)=cursor{
                Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
            }else{
                Err(Status::InternalServerError)
            }
        },
        Err(_)=>Err(Status::NotFound)
    }
}
pub fn delete_truck(conn: &Database,truck_id:i64)->Result<i8,Status> {
    match conn.collection(COLLECTION).find_one_and_delete(doc! {"_id":truck_id}, None) {
        Ok(cursor) => Ok(1),
        Err(_) => Err(Status::NotFound)
    }
}