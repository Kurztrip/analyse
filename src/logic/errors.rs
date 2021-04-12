use std::error::Error;

pub enum LogicError{
    InvalidWarehouse,
    InvalidState(String),
    DuplicatedID,
    NotFound,
    InternalError(Box<dyn Error>)
}