use std::fmt::Debug;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::{header, StatusCode};
use actix_web::http::header::HeaderValue;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 查 数据返回
pub struct ListData<T> {
  pub list: Vec<T>,
  pub total: usize,
  pub total_pages: usize,
  pub page_num: usize,
}

/// 分页参数
#[derive(Deserialize, Debug, Serialize, Default)]
pub struct PageParams {
  pub page_num: Option<usize>,
  pub page_size: Option<usize>,
}

/// 数据统一返回格式
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Res<T> {
  pub code: Option<i32>,
  pub data: Option<T>,
  pub msg: Option<String>,
}

/// 填入到extensions中的数据
#[derive(Debug)]
pub struct ResJsonString(pub String);

impl<R> Responder for Res<R>
  where
    R: Serialize + DeserializeOwned + Clone,
{
  type Body = String;

  fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
    let mut response = self.resp_json();
    let res_json_string = ResJsonString(response.body().clone());
    response.extensions_mut().insert(res_json_string);
    response
  }
}

impl<T: Serialize> Res<T> {
  pub fn with_data(data: T) -> Self {
    Self {
      code: Some(200),
      data: Some(data),
      msg: Some("success".to_string()),
    }
  }
  pub fn with_err(err: &str) -> Self {
    Self {
      code: Some(500),
      data: None,
      msg: Some(err.to_string()),
    }
  }
  pub fn with_err_msg(code: StatusCode, msg: String) -> Self {
    Self {
      code: Some(code.as_u16() as i32),
      data: None,
      msg: Some(msg),
    }
  }
  pub fn with_msg(msg: &str) -> Self {
    Self {
      code: Some(200),
      data: None,
      msg: Some(msg.to_string()),
    }
  }
  #[allow(dead_code)]
  pub fn with_data_msg(data: T, msg: &str) -> Self {
    Self {
      code: Some(200),
      data: Some(data),
      msg: Some(msg.to_string()),
    }
  }


  pub fn resp_json(&self) -> HttpResponse<String>
  {
    let mut response = HttpResponse::<String>::with_body(StatusCode::OK, serde_json::to_string(self).unwrap());
    response.headers_mut().append(header::CONTENT_TYPE, HeaderValue::from_static("text/json;charset=UTF-8"));
    response
  }
}

impl<T> ToString for Res<T>
  where
    T: Serialize + DeserializeOwned + Clone,
{
  fn to_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }

}
