use rocket::{response, Request};
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket_contrib::json::JsonValue;
use crate::routes::catchers;

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