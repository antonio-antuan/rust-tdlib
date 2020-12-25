
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains the description of an error in a Telegram Passport element; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementError {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of Telegram Passport element that has the error
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: PassportElementType,
  /// Error message
  message: String,
  /// Error source
  source: InputPassportElementErrorSource,
  
}

impl RObject for InputPassportElementError {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementError" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl InputPassportElementError {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementErrorBuilder {
    let mut inner = InputPassportElementError::default();
    inner.td_name = "inputPassportElementError".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInputPassportElementErrorBuilder { inner }
  }

  pub fn type_(&self) -> &PassportElementType { &self.type_ }

  pub fn message(&self) -> &String { &self.message }

  pub fn source(&self) -> &InputPassportElementErrorSource { &self.source }

}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorBuilder {
  inner: InputPassportElementError
}

impl RTDInputPassportElementErrorBuilder {
  pub fn build(&self) -> InputPassportElementError { self.inner.clone() }

   
  pub fn type_<T: AsRef<PassportElementType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

   
  pub fn message<T: AsRef<str>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().to_string();
    self
  }

   
  pub fn source<T: AsRef<InputPassportElementErrorSource>>(&mut self, source: T) -> &mut Self {
    self.inner.source = source.as_ref().clone();
    self
  }

}

impl AsRef<InputPassportElementError> for InputPassportElementError {
  fn as_ref(&self) -> &InputPassportElementError { self }
}

impl AsRef<InputPassportElementError> for RTDInputPassportElementErrorBuilder {
  fn as_ref(&self) -> &InputPassportElementError { &self.inner }
}



