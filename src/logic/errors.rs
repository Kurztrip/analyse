use std::error::Error;

pub enum LogicError{
    InvalidWarehouse,
    InvalidState(String),
    DuplicatedID,
    NotFound,
    InsufficientTrucks,
    NotCurrentRoute,
    InternalError(Box<dyn Error>)
}