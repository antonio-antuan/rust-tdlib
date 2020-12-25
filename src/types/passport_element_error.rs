
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains the description of an error in a Telegram Passport element
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementError {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the Telegram Passport element which has the error
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: PassportElementType,
  /// Error message
  message: String,
  /// Error source
  source: PassportElementErrorSource,
  
}

impl RObject for PassportElementError {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementError" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PassportElementError {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementErrorBuilder {
    let mut inner = PassportElementError::default();
    inner.td_name = "passportElementError".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportElementErrorBuilder { inner }
  }

  pub fn type_(&self) -> &PassportElementType { &self.type_ }

  pub fn message(&self) -> &String { &self.message }

  pub fn source(&self) -> &PassportElementErrorSource { &self.source }

}

#[doc(hidden)]
pub struct RTDPassportElementErrorBuilder {
  inner: PassportElementError
}

impl RTDPassportElementErrorBuilder {
  pub fn build(&self) -> PassportElementError { self.inner.clone() }

   
  pub fn type_<T: AsRef<PassportElementType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

   
  pub fn message<T: AsRef<str>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().to_string();
    self
  }

   
  pub fn source<T: AsRef<PassportElementErrorSource>>(&mut self, source: T) -> &mut Self {
    self.inner.source = source.as_ref().clone();
    self
  }

}

impl AsRef<PassportElementError> for PassportElementError {
  fn as_ref(&self) -> &PassportElementError { self }
}

impl AsRef<PassportElementError> for RTDPassportElementErrorBuilder {
  fn as_ref(&self) -> &PassportElementError { &self.inner }
}



