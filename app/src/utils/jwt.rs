use actix_web::dev::ServiceRequest;
use actix_web::http::header::HeaderMap;
use actix_web::http::StatusCode;
use chrono::{Duration, Local};
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use sea_orm::debug_print;
use serde::{Deserialize, Serialize};
use db::common::errors::{Error, Result, BadRequest};
use tracing::{debug, error, info};
use crate::apps::system::check_user_online;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = &CFG.jwt.jwt_secret;
    info!("Current Secret: {}", secret);
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
        let claims = verify_token(token_v.as_str())?;
        let token_id = claims.token_id.clone();
        let (x, _) = check_user_online(None, token_id).await;
        if !x {
            return Err(BadRequest::msg("该账户已经退出"));
        }
        Ok(claims)
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
            let bearer = Authorization::bearer(token).unwrap();
            // Decode the user data
            let bearer_data = bearer.token();
            let cut = bearer_data.len() - scru128::scru128_string().len();
            debug!("get_bear_token {} {}", bearer_data, cut);
            Some(bearer_data["Bearer ".len()..cut].to_string().trim().to_string())
        }
        _ => None
    };
    return res;
}

/// create token
/// secret: your secret string
pub fn create_token(claims: &Claims) -> Result<String> {
    return match encode(
        &Header::default(),
        claims,
        &KEYS.encoding,
    ) {
        Ok(t) => Ok(t),
        Err(_) => Err(BadRequest::msg("Token creation error")), // in practice you would return the error
    };
}

/// verify token invalid
/// secret: your secret string
pub fn verify_token(token: &str) -> Result<Claims> {
    let validation = Validation::default();
    return match decode::<Claims>(
        token,
        &KEYS.decoding,
        &validation,
    ) {
        Ok(c) => Ok(c.claims),
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => return Err(BadRequest::msg("你的登录已失效，请重新登录")),
            ErrorKind::ExpiredSignature => return Err(BadRequest::msg("你的登录已失效，请重新登录")),
            _ => return {
                debug_print!("{:?}", err);
                Err(BadRequest::msg(format!("其他错误：{:?}", err).as_str()))
            }
        },
    };
}

pub async fn authorize(payload: AuthPayload, token_id: String) -> Result<AuthBody> {
    if payload.id.is_empty() || payload.name.is_empty() {
        return Err(BadRequest::msg("Missing credentials"));
    }
    let iat = Local::now();
    let exp = iat + Duration::minutes(CFG.jwt.jwt_exp);
    let claims = Claims {
        id: payload.id.to_owned(),
        token_id: token_id.clone(),
        name: payload.name,
        exp: exp.timestamp(),
    };
    // Create the authorization token
    let token = create_token(&claims)?;

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


#[cfg(test)]
mod test {
    use std::thread::sleep;
    use std::time::Duration;
    use rbatis::DateTimeNative;
    use rbatis::DateTimeNative;
    use crate::domain::vo::JWTToken;
    use crate::utils::jwt::{Claims, create_token, verify_token};

    #[test]
    fn test_jwt() {
        let claims = Claims { id: "00UT9J78PSU5QJRE3HSDUG94R2".to_string(), token_id: "011BKQ025ITM3CIM2GJ65OL2P8".to_string(), name: "user".to_string(), exp: 1650349605 };
        let res = create_token(&claims).unwrap();
        println!("{:?}", res);
        let res = verify_token(&res).unwrap();
        println!("{:?}", res);

    }
}
