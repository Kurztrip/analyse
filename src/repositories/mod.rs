use mongodb::sync:: Database;
use mongodb::{bson, bson::{doc, Document}};
use mongodb::options::{FindOneAndReplaceOptions, FindOneAndUpdateOptions, ReturnDocument};
use serde::Deserialize;
use std::ops::Deref;
use rocket::request::FromRequest;
use rocket::{Request, request, State};
use crate::logic::errors::LogicError;

pub mod packages;
pub mod trucks;
pub mod warehouses;
pub mod database_configuration;

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
    pub fn get_many_from_db<T: for<'de> Deserialize<'de>>(&self,source:&str, filter: impl Into<Option<Document>>) -> Result<Vec<T>, LogicError> {
        match self.collection(source).find(filter, None) {
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

    pub fn get_one_from_db<T: for<'de> Deserialize<'de>>(&self,source:&str, filter: impl Into<Option<Document>>) -> Result<T, LogicError> {
        match self.collection(source).find_one(filter, None) {
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
    pub fn add_one_to_db(&self,source:&str, document: Document) -> Result<i32, LogicError> {
        match self.collection(source).insert_one(document, None) {
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
    pub fn delete_many_from_db(&self,source:&str, filter: Document) -> Result<i8, LogicError> {
        match self.collection(source).delete_many(filter, None) {
            Ok(_) => Ok(1),
            Err(_) => Err(LogicError::NotFound)
        }
    }
    pub fn replace_in_db<T: for<'de> Deserialize<'de>>(&self,source:&str, id: i32, document: Document) -> Result<T, LogicError> {
        match self.collection(source).find_one_and_replace(
            doc! {"_id":id},
            document,
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
    pub fn update_in_db<T: for<'de> Deserialize<'de>>(&self,source:&str, id: i32, document: Document) -> Result<T, LogicError> {
        match self.collection(source).find_one_and_update(
            doc! {"_id":id},
            document,
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
