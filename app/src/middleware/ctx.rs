// use bytes::Bytes;
// use configs::CFG;
// use db::common::ctx::{ReqCtx, UserInfo};
//
// use crate::utils::jwt::Claims;
//
// use std::future::{ready, Ready};
//
// use anyhow::{anyhow, format_err};
// use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage, HttpRequest};
// use actix_web::dev::Payload;
// use actix_web::error::ErrorBadRequest;
// use actix_web::http::StatusCode;
// use futures::future::LocalBoxFuture;
// use db::common::{errors, errors::Result};
//
// // There are two steps in middleware processing.
// // 1. Middleware initialization, middleware factory gets called with
// //    next service in chain as parameter.
// // 2. Middleware's call method gets called with normal request.
// pub struct Context;
//
// // Middleware factory is `Transform` trait
// // `S` - type of the next service
// // `B` - type of response's body
// impl<S, B> Transform<S, ServiceRequest> for Context
//   where
//     S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//   type Response = ServiceResponse<B>;
//   type Error = Error;
//   type InitError = ();
//   type Transform = ContextMiddleware<S>;
//   type Future = Ready<Result<Self::Transform, Self::InitError>>;
//
//   fn new_transform(&self, inner: S) -> Self::Future {
//     ready(Ok(ContextMiddleware { inner }))
//   }
// }
//
// pub struct ContextMiddleware<S> {
//   inner: S,
// }
//
// impl<S, B> Service<ServiceRequest> for ContextMiddleware<S>
//   where
//     S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//   type Response = ServiceResponse<B>;
//   type Error = Error;
//   type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
//
//   forward_ready!(inner);
//
//   /// req上下文注入中间件 同时进行jwt授权验证
//   fn call(&self, req: ServiceRequest) -> Self::Future {
//     // 开始请求数据
//     Box::pin(async move {
//       // 请求信息ctx注入
//       let user = match Claims::from_request_without_body(&req).await {
//         Err(e) => return Err(ErrorBadRequest(e)),
//         Ok(claims) => UserInfo {
//           id: claims.id,
//           token_id: claims.token_id,
//           name: claims.name,
//         },
//       };
//
//       let ori_uri_path = req.uri().path().to_string();
//       let method = req.method().to_string();
//       let path = req.uri().path().replacen(&(CFG.server.api_prefix.clone() + "/"), "", 1);
//       let path_params = req.uri().query().unwrap_or("").to_string();
//       let req_ctx = ReqCtx {
//         ori_uri: if path_params.is_empty() { ori_uri_path } else { ori_uri_path + "?" + &path_params },
//         path,
//         path_params,
//         method: method.clone(),
//         user,
//         data: "".to_string(),
//       };
//
//       req.extensions_mut().insert(req_ctx);
//       let fut = self.inner.call(reqR);
//
//       let res_end = fut.await?;
//       Ok(res_end)
//     })
//   }
// }
//
//
// /// 获取body数据
// async fn get_body_data(body: &Payload) -> Result<(Bytes, String)> {
//   Ok((Bytes::new(), "暂时没有找到方法解析body".to_string()))
// }
