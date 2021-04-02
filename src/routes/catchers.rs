use rocket_contrib::json::JsonValue;
#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
#[catch(422)]
pub fn unprocessable_entity() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Unprocessable Entity"
    })
}
#[catch(500)]
pub fn internal_err() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Internal server error"
    })
}