use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};
use app::my_env;
use app::utils::jwt::{Claims, create_token, verify_token};
use configs::CFG;

fn main() {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", &CFG.log.log_level);
    }
    my_env::setup();
    //  设置日志追踪
    // if &CFG.log.log_level == "TRACE" {
    //     LogTracer::builder()
    //         .with_max_level(log::LevelFilter::Trace)
    //         .init()
    //         .unwrap();
    // }

    // 系统变量设置
    let log_env = my_env::get_log_level();

    //  日志设置
    let format = my_env::get_log_format();

    // 文件输出
    let file_appender = tracing_appender::rolling::hourly(&CFG.log.dir, &CFG.log.file);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 标准控制台输出
    let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let logger = Registry::default()
        .with(EnvFilter::from_default_env().add_directive(log_env.into()))
        .with(fmt::Layer::default().with_writer(std_non_blocking).event_format(format.clone()).pretty())
        .with(fmt::Layer::default().with_writer(non_blocking).event_format(format))
        // .with(console_layer)
        ;
    tracing::subscriber::set_global_default(logger).unwrap();

    test2()
}

fn test2() {
    println!("&CFG.jwt.jwt_secret {}", &CFG.jwt.jwt_secret);
    let claims = Claims { id: "00UT9J78PSU5QJRE3HSDUG94R2".to_string(), token_id: "011BKQ025ITM3CIM2GJ65OL2P8".to_string(), name: "user".to_string(), exp: 1650349605 };
    let res = create_token(&claims).unwrap();
    println!("{:?}", res);
    let res = verify_token(&res).unwrap();
    println!("{:?}", res);
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjAwVVQ5Sjc4UFNVNVFKUkUzSFNEVUc5NFIyIiwidG9rZW5faWQiOiIwMTFDTjhSOVVUQUlLRzIzN0FKOExSSk80RCIsIm5hbWUiOiJ1c2VyIiwiZXhwIjoxNjUwNDIxODgxfQ.TC0xosCvPtun5tVHOWUzyFGlUSEuXhVJDNyqvuD6Aa8";

    let res = verify_token(token).unwrap();
    println!("{:?}", res);

}

fn test() {
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
