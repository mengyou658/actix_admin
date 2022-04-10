use configs::CFG;
use db::common::ctx::{ReqCtx, UserInfo};

use crate::utils::ApiUtils;

use std::future::{ready, Ready};

use db::common::errors::{Result, BadRequest};
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use actix_web::dev::Payload;
use actix_web::error::ErrorBadRequest;
use actix_web::http::StatusCode;
use futures::future::LocalBoxFuture;
use tracing::debug;
use db::common::errors;
use crate::utils::jwt::Claims;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
  where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type InitError = ();
  type Transform = AuthMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, inner: S) -> Self::Future {
    ready(Ok(AuthMiddleware { inner }))
  }
}

pub struct AuthMiddleware<S> {
  inner: S,
}

/// 菜单授权中间件
impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
  where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(inner);

  /// req上下文注入中间件 同时进行jwt授权验证
  fn call(&self, req: ServiceRequest) -> Self::Future {
    // 权限
    // 登录权限
    let method = req.method().to_string();
    let path = req.uri().path().replacen(&(CFG.server.api_prefix.clone() + "/"), "", 1);
    let header = req.headers().clone();
    // 开始请求数据
    let fut = self.inner.call(req);

    Box::pin(async move {
      let user = match Claims::from_request_without_body(header).await {
        Err(e) => return Err(ErrorBadRequest(e)),
        Ok(claims) => UserInfo {
          id: claims.id,
          token_id: claims.token_id,
          name: claims.name,
        },
      };
      // 如果是超级用户，则不需要验证权限，直接放行
      if !CFG.system.super_user.contains(&user.id) {
        // 验证api权限，如果不在路由表中，则放行，否则验证权限
        if ApiUtils::is_in(&path).await {
          if !ApiUtils::check_api_permission(&path, &method, &user.id).await {
            return Err(ErrorBadRequest(errors::BadRequest::new("你没有权限访问该页面/API", StatusCode::FORBIDDEN)));
          }
        }
      }
      let mut res = fut.await?;
      Ok(res)
    })
  }
}
