use std::fmt;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;

#[derive(Debug)]
pub enum UserError {
    NotFound,
    AddingUser,
    UpdatingUser,
    DeletingUser,
    DieselError(DieselError),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserError::NotFound => write!(f, "User not found"),
            UserError::AddingUser => write!(f, "Error adding user"),
            UserError::UpdatingUser => write!(f, "Error updating user"),
            UserError::DeletingUser => write!(f, "Error deleting user"),
            UserError::DieselError(diesel_error) => write!(f, "Diesel error: {}", diesel_error),
        }
    }
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserError::NotFound => HttpResponse::NotFound().json(self.to_string()),
            _ => HttpResponse::InternalServerError().json(self.to_string()),
        }
    }
}
