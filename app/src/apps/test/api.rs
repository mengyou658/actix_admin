pub mod test_data_scope;

use actix_web::{Route, Scope};
use actix_web::web;

pub fn test_api(route: Scope) -> Scope {
  route.service(test_data_scope_api(web::scope("/data_scope"))) // 数据权限测试
}

fn test_data_scope_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(test_data_scope::get_sort_list))) // 获取筛选分页
    .service(web::resource("/add").route(web::post().to(test_data_scope::add))) // 添加
    .service(web::resource("/delete").route(web::delete().to(test_data_scope::delete))) // 硬删除
}
