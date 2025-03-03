use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        entities::sys_post,
        models::sys_post::{AddReq, DeleteReq, EditReq, Resp, SearchReq},
    },
    DB,
};
use actix_web::web::{Json, Query, ReqData};

use super::super::service;
use crate::utils::jwt::Claims;

/// get_list 获取列表
/// page_params 分页参数

pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SearchReq>) -> Res<ListData<sys_post::Model>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_post::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// add 添加

pub async fn add(Json(req): Json<AddReq>, user: ReqData<Claims>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_post::add(db, req, user.id.clone()).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除

pub async fn delete(Json(req): Json<DeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_post::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改

pub async fn edit(Json(req): Json<EditReq>, user: ReqData<Claims>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_post::edit(db, req, user.id.clone()).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户

pub async fn get_by_id(Query(req): Query<SearchReq>) -> Res<Resp> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_post::get_by_id(db, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_all 获取全部

pub async fn get_all() -> Res<Vec<Resp>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_post::get_all(db).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
