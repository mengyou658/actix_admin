use actix_web::{App, HttpResponse, HttpServer, middleware, web, error, dev};
use actix_http::body::{BodySize, MessageBody};
use actix_web::body::BoxBody;
use actix_web::dev::JsonBody::Body;
use actix_web::dev::ServiceResponse;
use actix_web::http::{header, StatusCode};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::web::JsonConfig;

pub fn get_error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new()
        .handler(StatusCode::INTERNAL_SERVER_ERROR, common_error)
        .handler(StatusCode::BAD_REQUEST, common_error)
}

pub fn get_json_config() -> JsonConfig {
    let json_config = web::JsonConfig::default()
        // .limit(4096)
        .error_handler(|err, _req| {
            // create custom error response
            // actix_web::error::InternalError::from_response(err, HttpResponse::Conflict().finish()).
            //     .into()
            actix_web::error::InternalError::from_response(
                "",
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .body(format!(r#"{{"msg":"{:?}"}}"#, err)),
            )
                .into()
        });
    json_config
}

fn common_error<B>(mut res: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<BoxBody>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json;charset=UTF-8"),
    );
    let errorMsg: String = match res.response().error() {
        Some(e) => format!("{:?}", e),
        None => String::from("Unknown Error")
    };
    let response = HttpResponse::build(res.status())
        .content_type(header::ContentType::json())
        .body(errorMsg);
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

