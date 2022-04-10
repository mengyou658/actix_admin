use std::fmt::{Display, Formatter};
use actix_web::http::{header, StatusCode};
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::web::Json;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use tokio::task::JoinError;
use crate::common::res::Res;
use mime::Mime;
use derive_more::{Add, Display, From, Into};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
  #[error("Failed to read application context")]
  ReadContext,

  #[error("{0}")]
  Authenticate(#[from] AuthenticateError),

  #[error("{0}")]
  BadRequest(#[from] BadRequest),

  #[error("{0}")]
  NotFound(#[from] NotFound),

  #[error("{0}")]
  RunSyncTask(#[from] JoinError),

}

impl Error {
  fn get_codes(&self) -> (StatusCode, u16) {
    match self {
      // 4XX Errors
      // Error::ParseObjectID(_) => (StatusCode::BAD_REQUEST, 400),
      // Error::Wither(WitherError::Mongo(MongoError { ref kind, .. })) => {
      //   let mongo_error = kind.as_ref();
      //   match mongo_error {
      //     // MongoDB E11000 error code represent a duplicate key error
      //     MongoErrorKind::CommandError(MongoCommandError { code: 11000, .. }) => {
      //       (StatusCode::BAD_REQUEST, 40002)
      //     }
      //     _ => (StatusCode::INTERNAL_SERVER_ERROR, 5003),
      //   }
      // }
      Error::BadRequest(b) => (b.code.clone(), 40001),
      Error::NotFound(_) => (StatusCode::BAD_REQUEST, 40003),

      Error::Authenticate(AuthenticateError::WrongCredentials) => (StatusCode::BAD_REQUEST, 40003),
      Error::Authenticate(AuthenticateError::InvalidToken) => (StatusCode::BAD_REQUEST, 40003),
      Error::Authenticate(AuthenticateError::Locked) => (StatusCode::BAD_REQUEST, 40003),
      Error::Authenticate(AuthenticateError::UNAUTHORIZED) => (StatusCode::UNAUTHORIZED, 401),

      // 5XX Errors
      Error::ReadContext => (StatusCode::INTERNAL_SERVER_ERROR, 5001),
      Error::Authenticate(AuthenticateError::TokenCreation) => {
        (StatusCode::INTERNAL_SERVER_ERROR, 5001)
      }
      Error::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5009),
    }
  }
}


impl ResponseError for Error {
  fn status_code(&self) -> StatusCode {
    let (status_code, _) = self.get_codes();
    status_code
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code())
      .insert_header(header::ContentType(mime::APPLICATION_JSON))
      .body(self.to_string())
  }

}

impl From<actix_multipart::MultipartError> for Error {
  fn from(err: actix_multipart::MultipartError) -> Error {
    BadRequest::msg(err.to_string().as_str())
  }
}
impl From<delay_timer::error::TaskError> for Error {
  fn from(err: delay_timer::error::TaskError) -> Error {
    BadRequest::msg(err.to_string().as_str())
  }
}
impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Error {
    BadRequest::msg(err.to_string().as_str())
  }
}
impl From<anyhow::Error> for Error {
  fn from(err: anyhow::Error) -> Error {
    BadRequest::msg(err.to_string().as_str())
  }
}
// impl<T> From<T> for Error
//     where T: std::error::Error + Error
// {
//   fn from(err: T) -> Error {
//     BadRequest::msg(err.to_string().as_str())
//   }
// }
impl From<chrono::ParseError> for Error {
  fn from(err: chrono::ParseError) -> Error {
    BadRequest::msg(err.to_string().as_str())
  }
}
impl From<sea_orm::DbErr> for Error {
  fn from(err: sea_orm::DbErr) -> Error {
    BadRequest::msg(err.to_string().as_str())
  }
}
impl From<actix_web::Error> for Error {
  fn from(err: actix_web::Error) -> Error {
    BadRequest::msg(err.to_string().as_str())
  }
}

impl Into<Box<dyn ResponseError>> for Error {
  fn into(self) -> Box<dyn ResponseError> {
    Box::new(self)
  }
}



impl Responder for Error
{
  type Body = String;

  fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
    let (status_code, code) = self.get_codes();
    let message = self.to_string();
    Res::<String>::with_err_msg(status_code, message).resp_json()
  }
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AuthenticateError {
  #[error("Wrong authentication credentials")]
  WrongCredentials,
  #[error("Failed to create authentication token")]
  TokenCreation,
  #[error("Invalid authentication credentials")]
  InvalidToken,
  #[error("User is locked")]
  Locked,
  #[error("没有登录")]
  UNAUTHORIZED,
}

#[derive(thiserror::Error, Debug)]
#[error("Bad request. Code: {code}, message: {message}")]
pub struct BadRequest {
  pub code: StatusCode,
  pub message: String,
}

impl BadRequest {
  pub fn new(message: &str, code: StatusCode) -> Error {
    Error::BadRequest(BadRequest { code, message: message.to_string() })
  }
  pub fn msg(message: &str) -> Error {
    Error::BadRequest(BadRequest { code: StatusCode::BAD_REQUEST, message: message.to_string() })
  }

  pub fn empty() -> Error {
    Error::BadRequest(BadRequest {
      code: StatusCode::BAD_REQUEST,
      message: String::new(),
    })
  }
}

#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFound {
  resource: String,
  message: String,
}

impl NotFound {
  pub fn new(resource: String) -> Self {
    NotFound {
      resource: resource.clone(),
      message: format!("{} not found", resource),
    }
  }
}
