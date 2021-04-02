use crate::data::truck_models::Truck;
use mongodb::sync::Database;
use mongodb::bson;
use mongodb::bson::doc;
use crate::routes::responses::ApiResponse;


const COLLECTION:&str="trucks";

pub fn get_all_trucks(conn: &Database)->Result<Vec<Truck>,ApiResponse>{
    match conn.collection(COLLECTION).find(None,None){
       Ok(cursor)=>{
           cursor.map(|result|match result{
               Ok(doc)=>{
                   Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
               },
               Err(_)=>Err(ApiResponse::internal_err())
           }).collect::<Result<Vec<Truck>,ApiResponse>>()
       },
        Err(_)=>Err(ApiResponse::internal_err())
    }
}

pub fn add_truck(conn: &Database,truck:Truck)->Result<Truck,ApiResponse>{
    let collection = conn.collection(COLLECTION);
    match collection.insert_one(
            bson::to_bson(&truck).unwrap().as_document().unwrap().to_owned()
            ,None){
        Ok(result)=>{
            get_truck(conn,bson::from_bson(result.inserted_id).unwrap())
        },
        Err(_)=>Err(ApiResponse::internal_err())
    }
}
pub fn update_truck(conn: &Database,truck:Truck,truck_id:i32)->Result<Truck,ApiResponse>{
    let collection = conn.collection(COLLECTION);
    match collection.find_one_and_replace(
        doc!{"_id":truck_id},
        bson::to_bson(&truck).unwrap().as_document().unwrap().to_owned(),
        None
    ){
        Ok(_)=>{
            get_truck(conn,truck.id)
        },
        Err(_)=>Err(ApiResponse::internal_err())
    }
}
pub fn get_truck(conn: &Database,truck_id:i32)->Result<Truck,ApiResponse>{
    match conn.collection(COLLECTION).find_one(Some(doc!{"_id":truck_id}),None){
        Ok(cursor)=>{
            if let Some(doc)=cursor{
                Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
            }else{
                Err(ApiResponse::internal_err())
            }
        },
        Err(_)=>Err(ApiResponse::not_found())
    }
}
pub fn delete_truck(conn: &Database,truck_id:i32)->Result<i8,ApiResponse> {
    match conn.collection(COLLECTION).find_one_and_delete(doc! {"_id":truck_id}, None) {
        Ok(_) => Ok(1),
        Err(_) => Err(ApiResponse::not_found())
    }
}