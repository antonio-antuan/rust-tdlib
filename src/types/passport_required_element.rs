
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a description of the required Telegram Passport element that was requested by a service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportRequiredElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of Telegram Passport elements any of which is enough to provide
  suitable_elements: Vec<PassportSuitableElement>,
  
}

impl RObject for PassportRequiredElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportRequiredElement" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PassportRequiredElement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportRequiredElementBuilder {
    let mut inner = PassportRequiredElement::default();
    inner.td_name = "passportRequiredElement".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportRequiredElementBuilder { inner }
  }

  pub fn suitable_elements(&self) -> &Vec<PassportSuitableElement> { &self.suitable_elements }

}

#[doc(hidden)]
pub struct RTDPassportRequiredElementBuilder {
  inner: PassportRequiredElement
}

impl RTDPassportRequiredElementBuilder {
  pub fn build(&self) -> PassportRequiredElement { self.inner.clone() }

   
  pub fn suitable_elements(&mut self, suitable_elements: Vec<PassportSuitableElement>) -> &mut Self {
    self.inner.suitable_elements = suitable_elements;
    self
  }

}

impl AsRef<PassportRequiredElement> for PassportRequiredElement {
  fn as_ref(&self) -> &PassportRequiredElement { self }
}

impl AsRef<PassportRequiredElement> for RTDPassportRequiredElementBuilder {
  fn as_ref(&self) -> &PassportRequiredElement { &self.inner }
}



