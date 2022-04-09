use actix_web::dev::ServiceRequest;
use actix_web::http::header::HeaderMap;
use actix_web::http::StatusCode;
use chrono::{Duration, Local};
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Error, Result};
use tracing::debug;
use crate::apps::system::check_user_online;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
  let secret = &CFG.jwt.jwt_secret;
    debug!("Current Secret: {}", secret);
  Keys::new(secret.as_bytes())
});

use configs::CFG;
use db::common::errors;

pub struct Keys {
  pub encoding: EncodingKey,
  pub decoding: DecodingKey,
}

impl Keys {
  fn new(secret: &[u8]) -> Self {
    Self {
      encoding: EncodingKey::from_secret(secret),
      decoding: DecodingKey::from_secret(secret),
    }
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthPayload {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
  pub id: String,
  pub token_id: String,
  pub name: String,
  pub exp: i64,
}

impl Claims {
  // type Error = AuthError;
  /// 将用户信息注入request
  pub async fn from_request(req: HeaderMap) -> Result<Self> {
    let token_v = get_bear_token(req).await.unwrap();
      debug!("authorize token: {}", token_v);
      let token_data = match decode::<Claims>(token_v.as_str(), &KEYS.decoding, &Validation::default()) {
      Ok(token) => {
        let token_id = token.claims.token_id.clone();
        let (x, _) = check_user_online(None, token_id).await;
        if x {
          token
        } else {
          return Err(anyhow!(errors::BadRequest::new("该账户已经退出", StatusCode::UNAUTHORIZED)));
        }
      }
      Err(err) => match *err.kind() {
        ErrorKind::InvalidToken => {
          return Err(anyhow!(errors::BadRequest::new("你的登录已失效，请重新登录", StatusCode::UNAUTHORIZED)));
        }
        ErrorKind::ExpiredSignature => {
          return Err(anyhow!(errors::BadRequest::new("你的登录已失效，请重新登录", StatusCode::UNAUTHORIZED)));
        }
        _ => {
          return Err(anyhow!(errors::BadRequest::new(err.to_string().as_str(), StatusCode::UNAUTHORIZED)));
        }
      },
    };
    Ok(token_data.claims)
  }
  pub async fn from_request_without_body(req: HeaderMap) -> Result<Self> {
    Self::from_request(req).await
  }
}

pub async fn get_bear_token(req: HeaderMap) -> Option<String> {
  let t = req.get("Authorization");
  let res = match t {
    Some(token) => {
      let token = token.to_str().unwrap_or("");
      if token.is_empty() || !token.starts_with("Bearer ") {
        None
      } else {
        Some(token.replace("Bearer ", "").clone().trim().to_string())
      }
    }
    _ => None
  };
  return res;
}

pub async fn authorize(payload: AuthPayload, token_id: String) -> Result<AuthBody> {
  if payload.id.is_empty() || payload.name.is_empty() {
    return Err(anyhow!(errors::BadRequest::msg("Missing credentials")));
  }
  let iat = Local::now();
  let exp = iat + Duration::minutes(CFG.jwt.jwt_exp);
  let claims = Claims {
    id: payload.id.to_owned(),
    token_id: token_id.clone(),
    name: payload.name,
    exp: exp.timestamp(),
  };
    debug!("authorize claims: {:?}", claims);
    // Create the authorization token
  let token = encode::<Claims>(&Header::default(), &claims, &KEYS.encoding).map_err(|_| anyhow!(errors::BadRequest::new("Token creation error", StatusCode::INTERNAL_SERVER_ERROR)))?;

  // Send the authorized token
  Ok(AuthBody::new(token, claims.exp, CFG.jwt.jwt_exp, token_id))
}

// #[derive(Debug)]
// pub enum AuthError {
//     WrongCredentials,
//     MissingCredentials,
//     TokenCreation,
//     InvalidToken,
// }
// impl IntoResponse for AuthError {
//     fn into_response(self) -> Response {
//         let (status, error_message) = match self {
//             AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong
// credentials"),             AuthError::MissingCredentials =>
// (StatusCode::BAD_REQUEST, "Missing credentials"),
// AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token
// creation error"),             AuthError::InvalidToken =>
// (StatusCode::BAD_REQUEST, "Invalid token"),         };
//         let body = Json(json!({
//             "error": error_message,
//         }));
//         (status, body).into_response()
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
  token: String,
  token_type: String,
  pub exp: i64,
  exp_in: i64,
}

impl AuthBody {
  fn new(access_token: String, exp: i64, exp_in: i64, token_id: String) -> Self {
    Self {
      token: access_token + &token_id,
      token_type: "Bearer".to_string(),
      exp,
      exp_in,
    }
  }
}
