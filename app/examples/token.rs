use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use app::utils::jwt::Claims;

fn main() {
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjAwVVQ5Sjc4UFNVNVFKUkUzSFNEVUc5NFIyIiwidG9rZW5faWQiOiIwMTFCS1EwMjVJVE0zQ0lNMkdKNjVPTDJQOCIsIm5hbWUiOiJ1c2VyIiwiZXhwIjoxNjUwMzQ5NjA1fQ.Htg3IWZUUN7l2cJ-S2S_3C7e68c0YBw_nhXQ8uX6mgU011BKQ025ITM3CIM2GJ65OL2P8";
    let secret = "secret";
    let validation = Validation::default();
    let dec = &DecodingKey::from_secret(secret.as_bytes());
    let enc = &EncodingKey::from_secret(secret.as_bytes());
    let iat = Local::now();
    let exp = iat + Duration::minutes(3600);
    let claims = Claims { id: "00UT9J78PSU5QJRE3HSDUG94R2".to_string(), token_id: "011BKQ025ITM3CIM2GJ65OL2P8".to_string(), name: "user".to_string(), exp: 1650349605 };
    let res = encode::<Claims>(&Header::default(), &claims, enc).unwrap();
    println!("{:?}", res);
    let res = decode::<Claims>(&res, dec, &validation).unwrap();
    println!("{:?}", res);
    let res = decode::<Claims>(token, dec, &validation).unwrap();
    println!("{:?}", res);
}

