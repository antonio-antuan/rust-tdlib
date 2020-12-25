
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// An object of this type can be returned on every function call, in case of an error
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Error {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Error code; subject to future changes. If the error code is 406, the error message must not be processed in any way and must not be displayed to the user
  code: i64,
  /// Error message; subject to future changes
  message: String,
  
}

impl RObject for Error {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "error" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Error {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDErrorBuilder {
    let mut inner = Error::default();
    inner.td_name = "error".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDErrorBuilder { inner }
  }

  pub fn code(&self) -> i64 { self.code }

  pub fn message(&self) -> &String { &self.message }

}

#[doc(hidden)]
pub struct RTDErrorBuilder {
  inner: Error
}

impl RTDErrorBuilder {
  pub fn build(&self) -> Error { self.inner.clone() }

   
  pub fn code(&mut self, code: i64) -> &mut Self {
    self.inner.code = code;
    self
  }

   
  pub fn message<T: AsRef<str>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().to_string();
    self
  }

}

impl AsRef<Error> for Error {
  fn as_ref(&self) -> &Error { self }
}

impl AsRef<Error> for RTDErrorBuilder {
  fn as_ref(&self) -> &Error { &self.inner }
}



