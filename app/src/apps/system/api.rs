mod common;
mod sys_api_db;
mod sys_dept;
mod sys_dict_data;
mod sys_dict_type;
mod sys_job;
mod sys_job_log;
mod sys_login_log;
mod sys_menu;
pub mod sys_oper_log;
mod sys_post;
mod sys_role;
// 角色管理
mod sys_update_log;

//
mod sys_user;
// 用户管理
mod sys_user_online; // 操作日志

use actix_web::{Resource, Route, Scope};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web;
pub use common::get_captcha;
pub use sys_user::login;
pub use sys_user_online::log_out;
use crate::middleware;

pub fn system_api(route: Scope) -> Scope<impl ServiceFactory<ServiceRequest, Response = ServiceResponse, Error = actix_web::Error, Config = (), InitError = ()>> {
  route.wrap(middleware::ApiAuth)
    // .service(web::post().to(sys_user::loginweb::scope("/login"))) //登录
    .service(sys_user_api(web::scope("/user"))) // 用户管理模块
    .service(sys_dict_type_api(web::scope("/dict/type"))) // 字典类型模块
    .service(sys_dict_data_api(web::scope("/dict/data"))) // 字典数据模块
    .service(sys_post_api(web::scope("/post"))) // 岗位模块
    .service(sys_dept_api(web::scope("/dept"))) // 部门模块
    .service(sys_role_api(web::scope("/role"))) // 角色模块
    .service(sys_menu_api(web::scope("/menu"))) // 路由 菜单 模块
    .service(sys_login_log_api(web::scope("/login-log"))) // 登录日志模块
    .service(sys_user_online_api(web::scope("/online"))) // 在线用户
    .service(sys_job_api(web::scope("/job"))) // 定时任务
    .service(sys_job_log_api(web::scope("/job_log"))) // 定时任务日志
    .service(sys_oper_log_api(web::scope("/oper_log"))) // 操作日志
    .service(sys_api_db_api(web::scope("/api_db"))) // 操作日志
    .service(sys_monitor_api(web::scope("/monitor"))) // 操作日志
    .service(sys_update_log_api(web::scope("/update_log"))) // 更新日志
}

fn sys_user_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(sys_user::get_sort_list))) // 获取全部用户
    .service(web::resource("/get_by_id").route(web::get().to(sys_user::get_by_id))) // 按id获取用户
    .service(web::resource("/get_profile").route(web::get().to(sys_user::get_profile))) // 按当前获取用户信息
    .service(web::resource("/update_profile").route(web::put().to(sys_user::update_profile))) // 更新用户信息
    .service(web::resource("/add").route(web::post().to(sys_user::add))) // 添加用户
    .service(web::resource("/edit").route(web::put().to(sys_user::edit))) // 更新用户
    .service(web::resource("/delete").route(web::delete().to(sys_user::delete))) // 硬删除用户
    .service(web::resource("/get_info").route(web::get().to(sys_user::get_info))) // 获取用户信息
    .service(web::resource("/reset_passwd").route(web::put().to(sys_user::reset_passwd))) // 重置密码
    .service(web::resource("/update_passwd").route(web::put().to(sys_user::update_passwd))) // 重置密码
    .service(web::resource("/change_status").route(web::put().to(sys_user::change_status))) // 修改状态
    .service(web::resource("/change_role").route(web::put().to(sys_user::change_role))) // 切换角色
    .service(web::resource("/change_dept").route(web::put().to(sys_user::change_dept))) // 切换部门
    .service(web::resource("/fresh_token").route(web::put().to(sys_user::fresh_token))) // 修改状态
    .service(web::resource("/update_avatar").route(web::post().to(sys_user::update_avatar))) // 修改头像
}

fn sys_dict_type_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(sys_dict_type::get_sort_list))) // 获取筛选分页
    .service(web::resource("/get_all").route(web::get().to(sys_dict_type::get_all))) // 获取筛选分页
    .service(web::resource("/get_by_id").route(web::get().to(sys_dict_type::get_by_id))) // 按id获取
    .service(web::resource("/add").route(web::post().to(sys_dict_type::add))) // 添加
    .service(web::resource("/edit").route(web::put().to(sys_dict_type::edit))) // 更新
    // .service(web::resource("/delete").route(web::delete().to(sys_dict_type::delete))) //软删除
    .service(web::resource("/delete").route(web::delete().to(sys_dict_type::delete))) // 硬删除
}

fn sys_dict_data_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(sys_dict_data::get_sort_list))) // 获取筛选分页
    .service(web::resource("/get_all").route(web::get().to(sys_dict_data::get_all))) // 获取筛选分页
    .service(web::resource("/get_by_id").route(web::get().to(sys_dict_data::get_by_id))) // 按id获取
    .service(web::resource("/get_by_type").route(web::get().to(sys_dict_data::get_by_type))) // 按id获取
    .service(web::resource("/add").route(web::post().to(sys_dict_data::add))) // 添加
    .service(web::resource("/edit").route(web::put().to(sys_dict_data::edit))) // 更新
    .service(web::resource("/delete").route(web::delete().to(sys_dict_data::delete))) // 硬删除
}

fn sys_post_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(sys_post::get_sort_list))) // 获取筛选分页
    .service(web::resource("/get_all").route(web::get().to(sys_post::get_all))) // 获取筛选分页
    .service(web::resource("/get_by_id").route(web::get().to(sys_post::get_by_id))) // 按id获取
    .service(web::resource("/add").route(web::post().to(sys_post::add))) // 添加
    .service(web::resource("/edit").route(web::put().to(sys_post::edit))) // 更新
    // .service(web::resource("/delete").route(web::delete().to(sys_post::delete))) //软删除
    .service(web::resource("/delete").route(web::delete().to(sys_post::delete))) // 硬删除
}

fn sys_dept_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(sys_dept::get_sort_list))) // 获取筛选分页
    .service(web::resource("/get_all").route(web::get().to(sys_dept::get_all))) // 获取筛选分页
    .service(web::resource("/get_dept_tree").route(web::get().to(sys_dept::get_dept_tree))) // 获取部门树
    .service(web::resource("/get_by_id").route(web::get().to(sys_dept::get_by_id))) // 按id获取
    .service(web::resource("/add").route(web::post().to(sys_dept::add))) // 添加
    .service(web::resource("/edit").route(web::put().to(sys_dept::edit))) // 更新
    .service(web::resource("/delete").route(web::delete().to(sys_dept::delete))) // 硬删除
}

fn sys_role_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(sys_role::get_sort_list))) // 获取筛选分页
    .service(web::resource("/get_all").route(web::get().to(sys_role::get_all))) // 获取筛选分页
    .service(web::resource("/get_by_id").route(web::get().to(sys_role::get_by_id))) // 按id获取
    .service(web::resource("/add").route(web::post().to(sys_role::add))) // 添加
    .service(web::resource("/edit").route(web::put().to(sys_role::edit))) // 更新
    .service(web::resource("/update_auth_role").route(web::put().to(sys_role::update_auth_role))) // 更新角色授权
    .service(web::resource("/change_status").route(web::put().to(sys_role::change_status))) // 设置状态
    .service(web::resource("/set_data_scope").route(web::put().to(sys_role::set_data_scope))) // 设置数据权限范围
    .service(web::resource("/delete").route(web::delete().to(sys_role::delete))) // 硬删除
    .service(web::resource("/get_role_menu").route(web::get().to(sys_role::get_role_menu))) // 获取角色菜单
    .service(web::resource("/get_role_dept").route(web::get().to(sys_role::get_role_dept))) // 获取角色部门
    .service(web::resource("/cancel_auth_user").route(web::put().to(sys_role::cancel_auth_user))) // 批量用户取消角色授权
    .service(web::resource("/add_auth_user").route(web::put().to(sys_role::add_auth_user))) // 批量用户角色授权
    .service(web::resource("/get_auth_users_by_role_id").route(web::get().to(sys_role::get_auth_users_by_role_id))) // 获取角色对应用户
    .service(web::resource("/get_un_auth_users_by_role_id").route(web::get().to(sys_role::get_un_auth_users_by_role_id)))
  // 获取角色对应未授权用户
}

fn sys_menu_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(sys_menu::get_sort_list))) // 获取筛选分页
    // .service(web::resource("/get_auth_list").route(web::get().to(sys_menu::get_auth_list))) // 权限查询列表
    .service(web::resource("/get_by_id").route(web::get().to(sys_menu::get_by_id))) // 按id获取
    .service(web::resource("/add").route(web::post().to(sys_menu::add))) // 添加
    .service(web::resource("/edit").route(web::put().to(sys_menu::edit))) // 更新
    .service(web::resource("/update_log_cache_method").route(web::put().to(sys_menu::update_log_cache_method))) // 更新api缓存方式和日志记录方式
    .service(web::resource("/delete").route(web::delete().to(sys_menu::delete))) // 硬删除
    .service(web::resource("/get_all_enabled_menu_tree").route(web::get().to(sys_menu::get_all_enabled_menu_tree))) // 获取全部正常的路由菜单树
    .service(web::resource("/get_routers").route(web::get().to(sys_menu::get_routers))) // 获取用户菜单树
    .service(web::resource("/get_auth_list").route(web::get().to(sys_menu::get_related_api_and_db))) // 获取用户菜单树
}

fn sys_login_log_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(sys_login_log::get_sort_list))) // 获取筛选分页
    .service(web::resource("/clean").route(web::delete().to(sys_login_log::clean))) // 清空
    .service(web::resource("/delete").route(web::delete().to(sys_login_log::delete))) // 硬删除
}

fn sys_user_online_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(sys_user_online::get_sort_list))) // 获取筛选分页
    .service(web::resource("/delete").route(web::delete().to(sys_user_online::delete))) // 删除
}

fn sys_job_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(sys_job::get_sort_list))) // 获取筛选分页
    .service(web::resource("/get_by_id").route(web::get().to(sys_job::get_by_id))) // 按id获取
    .service(web::resource("/change_status").route(web::put().to(sys_job::change_status))) // 设置状态
    .service(web::resource("/run_task_once").route(web::put().to(sys_job::run_task_once))) // 设置状态
    .service(web::resource("/add").route(web::post().to(sys_job::add))) // 添加
    .service(web::resource("/edit").route(web::put().to(sys_job::edit))) // 更新
    .service(web::resource("/delete").route(web::delete().to(sys_job::delete))) // 硬删除
}

fn sys_job_log_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(sys_job_log::get_sort_list))) // 获取筛选分页
    // .service(web::resource("/get_by_id").route(web::get().to(sys_job_log::get_by_id))) // 按id获取
    .service(web::resource("/clean").route(web::delete().to(sys_job_log::clean))) // 清空
    .service(web::resource("/delete").route(web::delete().to(sys_job_log::delete))) // 硬删除
}

fn sys_oper_log_api(route: Scope) -> Scope {
  route
    .service(web::resource("/list").route(web::get().to(sys_oper_log::get_sort_list))) // 获取筛选分页
    .service(web::resource("/get_by_id").route(web::get().to(sys_oper_log::get_by_id))) // 按id获取
    .service(web::resource("/clean").route(web::delete().to(sys_oper_log::clean))) // 清空
    .service(web::resource("/delete").route(web::delete().to(sys_oper_log::delete))) // 硬删除
}

fn sys_api_db_api(route: Scope) -> Scope {
  route
    .service(web::resource("/get_by_id").route(web::get().to(sys_api_db::get_by_id))) // 按id获取
    .service(web::resource("/add").route(web::post().to(sys_api_db::add))) // 添加
}

fn sys_monitor_api(route: Scope) -> Scope {
  route.service(web::resource("/server").route(web::get().to(common::get_server_info))) // 服务器信息
}

fn sys_update_log_api(route: Scope) -> Scope {
  route
    .service(web::resource("/add").route(web::post().to(sys_update_log::add))) // 添加
    .service(web::resource("/edit").route(web::put().to(sys_update_log::edit))) // 更新
    .service(web::resource("/delete").route(web::delete().to(sys_update_log::delete))) // 硬删除
    .service(web::resource("/get_all").route(web::get().to(sys_update_log::get_all))) // 获取全部
}
