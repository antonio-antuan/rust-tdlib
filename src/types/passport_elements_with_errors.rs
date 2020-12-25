
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a Telegram Passport elements and corresponding errors
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementsWithErrors {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Telegram Passport elements
  elements: Vec<PassportElement>,
  /// Errors in the elements that are already available
  errors: Vec<PassportElementError>,
  
}

impl RObject for PassportElementsWithErrors {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElementsWithErrors" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PassportElementsWithErrors {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementsWithErrorsBuilder {
    let mut inner = PassportElementsWithErrors::default();
    inner.td_name = "passportElementsWithErrors".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportElementsWithErrorsBuilder { inner }
  }

  pub fn elements(&self) -> &Vec<PassportElement> { &self.elements }

  pub fn errors(&self) -> &Vec<PassportElementError> { &self.errors }

}

#[doc(hidden)]
pub struct RTDPassportElementsWithErrorsBuilder {
  inner: PassportElementsWithErrors
}

impl RTDPassportElementsWithErrorsBuilder {
  pub fn build(&self) -> PassportElementsWithErrors { self.inner.clone() }

   
  pub fn elements(&mut self, elements: Vec<PassportElement>) -> &mut Self {
    self.inner.elements = elements;
    self
  }

   
  pub fn errors(&mut self, errors: Vec<PassportElementError>) -> &mut Self {
    self.inner.errors = errors;
    self
  }

}

impl AsRef<PassportElementsWithErrors> for PassportElementsWithErrors {
  fn as_ref(&self) -> &PassportElementsWithErrors { self }
}

impl AsRef<PassportElementsWithErrors> for RTDPassportElementsWithErrorsBuilder {
  fn as_ref(&self) -> &PassportElementsWithErrors { &self.inner }
}



