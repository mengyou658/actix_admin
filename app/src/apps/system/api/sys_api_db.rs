use actix_web::web::{Json, Query};
use db::{
    common::res::Res,
    db_conn,
    system::{
        entities::sys_api_db,
        models::sys_api_db::{AddEditReq, SearchReq},
    },
    DB,
};

use super::super::service;

/// add 添加

pub async fn add(Json(req): Json<AddEditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_api_db::add(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 按id获取
/// db 数据库连接

pub async fn get_by_id(Query(req): Query<SearchReq>) -> Res<Vec<sys_api_db::Model>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_api_db::get_by_id(db, &req.api_id).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
