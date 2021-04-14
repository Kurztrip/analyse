use rocket::{response, Request};
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket_contrib::json::JsonValue;
use crate::routes::catchers;
use crate::logic::errors::LogicError;
use crate::logic::errors::LogicError::*;
use rocket_contrib::json;

#[derive(Debug)]
pub struct ApiResponse {
    status: Status,
    message: JsonValue,
}
impl ApiResponse {
    pub fn new(message:JsonValue, status:Status) -> Self{
        ApiResponse {
            status,
            message,
        }
    }
    pub fn ok(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::Ok,
            message,
        }
    }
    pub fn created(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::Created,
            message,
        }
    }
    pub fn invalid_warehouse() -> Self {
        ApiResponse {
            status:Status::BadRequest,
            message:json!({
                "status": "error",
                "reason": "The warehouse id doesn't exist"
            })
        }
    }
    pub fn invalid_state(e:String) -> Self {
        ApiResponse {
            status:Status::BadRequest,
            message:json!({
                "status": "error",
                "reason": e
            })
        }
    }
    pub fn duplicated_id() -> Self {
        ApiResponse {
            status:Status::BadRequest,
            message:json!({
                "status": "error",
                "reason": "The id already exists"
            })
        }
    }
    pub fn not_found() -> Self {
        ApiResponse {
            status: Status::NotFound,
            message: catchers::not_found(),
        }
    }
    pub fn internal_err() -> Self {
        ApiResponse {
            status: Status::InternalServerError,
            message: catchers::internal_err()
        }
    }
}
impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.message.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
impl From<LogicError> for ApiResponse{
    fn from(error:LogicError)->ApiResponse{
        match error{
            InvalidWarehouse=>ApiResponse::invalid_warehouse(),
            InvalidState(state)=>ApiResponse::invalid_state(state),
            DuplicatedID=>ApiResponse::duplicated_id(),
            NotFound=>ApiResponse::not_found(),
            _=>ApiResponse::internal_err()
        }
    }
}