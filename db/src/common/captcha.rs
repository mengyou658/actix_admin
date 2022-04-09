use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CaptchaImage {
  pub captcha_on_off: bool,
  pub uuid: String,
  pub img: String,
}
