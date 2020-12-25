
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about saved Telegram Passport elements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElements {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Telegram Passport elements
  elements: Vec<PassportElement>,
  
}

impl RObject for PassportElements {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElements" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PassportElements {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementsBuilder {
    let mut inner = PassportElements::default();
    inner.td_name = "passportElements".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportElementsBuilder { inner }
  }

  pub fn elements(&self) -> &Vec<PassportElement> { &self.elements }

}

#[doc(hidden)]
pub struct RTDPassportElementsBuilder {
  inner: PassportElements
}

impl RTDPassportElementsBuilder {
  pub fn build(&self) -> PassportElements { self.inner.clone() }

   
  pub fn elements(&mut self, elements: Vec<PassportElement>) -> &mut Self {
    self.inner.elements = elements;
    self
  }

}

impl AsRef<PassportElements> for PassportElements {
  fn as_ref(&self) -> &PassportElements { self }
}

impl AsRef<PassportElements> for RTDPassportElementsBuilder {
  fn as_ref(&self) -> &PassportElements { &self.inner }
}



