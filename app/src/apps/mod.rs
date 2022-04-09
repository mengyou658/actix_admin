use configs::CFG;
use actix_web::{Route, Scope};
use actix_web::web;
use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::web::service;
use anyhow::anyhow;
use db::common::ctx::UserInfo;
use db::common::errors;

use crate::middleware;
use crate::utils::ApiUtils;
use crate::utils::jwt::Claims;

pub mod system;
pub mod test;

pub fn api(route: Scope) -> Scope {
  route
    // 无需授权Api.通用模块
    .service(no_auth_api(web::scope("/comm")))
    // 系统管理模块
    .service(system::system_api(web::scope("/system")))
    //  测试模块
    .service(test::api::test_api(web::scope("/test")))
}

//

pub fn no_auth_api(route: Scope) -> Scope {
  route
    .service(web::resource("/login").route(web::post().to(system::SysLogin))) // 登录
    .service(web::resource("/get_captcha").route(web::get().to(system::get_captcha))) // 获取验证码
    .service(web::resource("/log_out").route(web::post().to(system::log_out))) // 退出登录
}
