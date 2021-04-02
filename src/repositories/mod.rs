use mongodb::{sync::Collection, bson, bson::{Document, doc}};
use crate::routes::responses::ApiResponse;
use serde::Deserialize;
use mongodb::options::{ReturnDocument, FindOneAndReplaceOptions};

pub mod packages;
pub mod trucks;
pub mod warehouses;

pub fn get_many_from_db<T:for<'de> Deserialize<'de>>(collection: Collection, filter:impl Into<Option<Document>>) ->Result<Vec<T>,ApiResponse>{
    match collection.find(filter,None){
        Ok(cursor)=>{
            cursor.map(|result|match result{
                Ok(doc)=>{
                    Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
                },
                Err(_)=>Err(ApiResponse::internal_err())
            }).collect::<Result<Vec<T>,ApiResponse>>()
        },
        Err(_)=>Err(ApiResponse::internal_err())
    }
}

pub fn get_one_from_db<T:for<'de> Deserialize<'de>>(collection: Collection, filter:impl Into<Option<Document>>) ->Result<T,ApiResponse>{
    match collection.find_one(filter,None) {
        Ok(cursor) => {
            if let Some(doc) = cursor {
                Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
            } else {
                Err(ApiResponse::not_found())
            }
        },
        Err(_) => Err(ApiResponse::internal_err())
    }
}
pub fn add_one_to_db(collection: Collection, document:Document) ->Result<i32,ApiResponse>{
    match collection.insert_one(document,None){
        Ok(result)=>{
            Ok(bson::from_bson(result.inserted_id).unwrap())
        },
        Err(_)=>Err(ApiResponse::duplicated_id())
    }
}
pub fn delete_from_db(collection: Collection, id:i32) ->Result<i8,ApiResponse>{
    match collection.find_one_and_delete(doc! {"_id":id}, None) {
        Ok(_) => Ok(1),
        Err(_) => Err(ApiResponse::not_found())
    }
}
pub fn replace_in_db<T:for<'de> Deserialize<'de>>(collection: Collection, id:i32, document:Document)->Result<T,ApiResponse>{
    match collection.find_one_and_replace(
        doc! {"_id":id},
        document,
        Some(FindOneAndReplaceOptions::builder().return_document(ReturnDocument::After).build())) {
        Ok(cursor) => {
            if let Some(doc) = cursor {
                Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
            } else {
                Err(ApiResponse::not_found())
            }
        },
        Err(_) => Err(ApiResponse::internal_err())
    }
}