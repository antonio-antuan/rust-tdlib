
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a Telegram Passport element that was requested by a service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportSuitableElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the element
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: PassportElementType,
  /// True, if a selfie is required with the identity document
  is_selfie_required: bool,
  /// True, if a certified English translation is required with the document
  is_translation_required: bool,
  /// True, if personal details must include the user's name in the language of their country of residence
  is_native_name_required: bool,
  
}

impl RObject for PassportSuitableElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportSuitableElement" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PassportSuitableElement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportSuitableElementBuilder {
    let mut inner = PassportSuitableElement::default();
    inner.td_name = "passportSuitableElement".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPassportSuitableElementBuilder { inner }
  }

  pub fn type_(&self) -> &PassportElementType { &self.type_ }

  pub fn is_selfie_required(&self) -> bool { self.is_selfie_required }

  pub fn is_translation_required(&self) -> bool { self.is_translation_required }

  pub fn is_native_name_required(&self) -> bool { self.is_native_name_required }

}

#[doc(hidden)]
pub struct RTDPassportSuitableElementBuilder {
  inner: PassportSuitableElement
}

impl RTDPassportSuitableElementBuilder {
  pub fn build(&self) -> PassportSuitableElement { self.inner.clone() }

   
  pub fn type_<T: AsRef<PassportElementType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

   
  pub fn is_selfie_required(&mut self, is_selfie_required: bool) -> &mut Self {
    self.inner.is_selfie_required = is_selfie_required;
    self
  }

   
  pub fn is_translation_required(&mut self, is_translation_required: bool) -> &mut Self {
    self.inner.is_translation_required = is_translation_required;
    self
  }

   
  pub fn is_native_name_required(&mut self, is_native_name_required: bool) -> &mut Self {
    self.inner.is_native_name_required = is_native_name_required;
    self
  }

}

impl AsRef<PassportSuitableElement> for PassportSuitableElement {
  fn as_ref(&self) -> &PassportSuitableElement { self }
}

impl AsRef<PassportSuitableElement> for RTDPassportSuitableElementBuilder {
  fn as_ref(&self) -> &PassportSuitableElement { &self.inner }
}



