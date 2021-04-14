extern crate serde_json;
extern crate mongodb;
use serde_json::Value;
use mongodb::sync:: Database;
use mongodb::{bson, bson::{doc, Document}};
use mongodb::options::{FindOneAndReplaceOptions, FindOneAndUpdateOptions, ReturnDocument};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use rocket::request::FromRequest;
use rocket::{Request, request, State};
use crate::logic::errors::LogicError;
use serde_json::{Map,json};
use std::convert::TryFrom;
use crate::repositories::database_models::DatabaseCoordinates;

pub mod packages;
pub mod trucks;
pub mod warehouses;
pub mod database_configuration;
mod database_models;

#[derive(Debug)]
pub struct DBConnection(Database);

impl Deref for DBConnection{
    type Target = Database;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a, 'r> FromRequest<'a, 'r> for DBConnection {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<DBConnection, ()> {
        request.guard::<State<Database>>().map(|database|DBConnection(database.clone()))
    }
}
impl DBConnection {
    pub fn convert_filter(filter: Option<Map<String,Value>>) ->Option<Document>{
        if let Some(mut map) = filter {
            if map.contains_key("id"){
                map=map.iter_mut().map(|(k,v)|(
                    if k=="id"{"_id".to_string()}else{k.to_owned()}
                    ,std::mem::take(v)
                )).collect();
            }
            return Some(bson::document::Document::try_from(map).unwrap())
        }
        None
    }
    pub fn get_from_near<T: for<'de> Deserialize<'de>>(&self,source:&str, filter: Option<Map<String,Value>>,location_field:String, coordinates:DatabaseCoordinates ) -> Result<Vec<T>, LogicError>{
        let mut real_filter=Map::new();
        let mut near_location=Map::new();
        near_location.insert("$near".to_string(),json!(coordinates));
        real_filter.insert(location_field,Value::Object(near_location));
        if let Some(mut f) = filter{
            real_filter.append(&mut f);
        }
        self.get_many_from_db(source,Some(real_filter))
    }
    pub fn get_many_from_db<T: for<'de> Deserialize<'de>>(&self,source:&str, filter: Option<Map<String,Value>>) -> Result<Vec<T>, LogicError> {
        let real_filter=Self::convert_filter(filter);
        match self.collection(source).find(real_filter, None) {
            Ok(cursor) => {
                cursor.map(|result| match result {
                    Ok(doc) => {
                        Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
                    },
                    Err(e) => Err(LogicError::InternalError(Box::new(e)))
                }).collect::<Result<Vec<T>, LogicError>>()
            },
            Err(e) => Err(LogicError::InternalError(Box::new(e)))
        }
    }

    pub fn get_one_from_db<T: for<'de> Deserialize<'de>>(&self,source:&str, filter: Option<Map<String,Value>>) -> Result<T, LogicError> {
        let real_filter =Self::convert_filter(filter);
        match self.collection(source).find_one(real_filter, None) {
            Ok(cursor) => {
                if let Some(doc) = cursor {
                    Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
                } else {
                    Err(LogicError::NotFound)
                }
            },
            Err(e) => Err(LogicError::InternalError(Box::new(e)))
        }

    }
    pub fn add_one_to_db<T:Serialize>(&self,source:&str, data:T) -> Result<i32, LogicError> {
        match self.collection(source).insert_one(
            bson::to_bson(&data).unwrap().as_document().unwrap().to_owned(),
            None) {
            Ok(result) => {
                Ok(bson::from_bson(result.inserted_id).unwrap())
            },
            Err(_) => Err(LogicError::DuplicatedID)
        }
    }
    pub fn delete_from_db(&self,source:&str, id: i32) -> Result<i8, LogicError> {
        match self.collection(source).find_one_and_delete(doc! {"_id":id}, None) {
            Ok(_) => Ok(1),
            Err(_) => Err(LogicError::NotFound)
        }
    }
    pub fn delete_many_from_db(&self,source:&str, filter: Map<String,Value>) -> Result<i8, LogicError> {
        let real_filter = Self::convert_filter(Some(filter)).unwrap();
        match self.collection(source).delete_many(real_filter, None) {
            Ok(_) => Ok(1),
            Err(_) => Err(LogicError::NotFound)
        }
    }
    pub fn replace_in_db<T: for<'de> Deserialize<'de>+Serialize>(&self,source:&str, id: i32, data: T) -> Result<T, LogicError> {
        match self.collection(source).find_one_and_replace(
            doc! {"_id":id},
            bson::to_bson(&data).unwrap().as_document().unwrap().to_owned(),
            Some(FindOneAndReplaceOptions::builder().return_document(ReturnDocument::After).build())) {
            Ok(cursor) => {
                if let Some(doc) = cursor {
                    Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
                } else {
                    Err(LogicError::NotFound)
                }
            },
            Err(e) => Err(LogicError::InternalError(Box::new(e)))
        }
    }

    pub fn update_in_db<T: for<'de> Deserialize<'de>>(&self,source:&str, id: i32, data:  Map<String,Value>) -> Result<T, LogicError> {
        let real_data = Self::convert_filter(Some(data)).unwrap();
        let update = doc!{"$set":real_data};
        Self::mongo_update(&self,source,id,update)
    }
    pub fn add_to_array<T: for<'de> Deserialize<'de>>(&self,source:&str, id: i32, data:  Map<String,Value>) -> Result<T, LogicError>{
        let real_data = Self::convert_filter(Some(data)).unwrap();
        let update = doc!{"$push":real_data};
        Self::mongo_update(&self,source,id,update)
    }
    pub fn mongo_update<T: for<'de> Deserialize<'de>>(&self,source:&str, id: i32,update:Document)-> Result<T, LogicError>{
        match self.collection(source).find_one_and_update(
            doc! {"_id":id},
            update,
            Some(FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build())) {
            Ok(cursor) => {
                if let Some(doc) = cursor {
                    Ok(bson::from_bson(bson::Bson::Document(doc)).unwrap())
                } else {
                    Err(LogicError::NotFound)
                }
            },
            Err(e) => Err(LogicError::InternalError(Box::new(e)))
        }
    }
}
