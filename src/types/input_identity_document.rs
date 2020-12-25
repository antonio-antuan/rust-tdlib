
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// An identity document to be saved to Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputIdentityDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Document number; 1-24 characters
  number: String,
  /// Document expiry date, if available
  expiry_date: Date,
  /// Front side of the document
  front_side: InputFile,
  /// Reverse side of the document; only for driver license and identity card
  reverse_side: InputFile,
  /// Selfie with the document, if available
  selfie: InputFile,
  /// List of files containing a certified English translation of the document
  translation: Vec<InputFile>,
  
}

impl RObject for InputIdentityDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputIdentityDocument" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl InputIdentityDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputIdentityDocumentBuilder {
    let mut inner = InputIdentityDocument::default();
    inner.td_name = "inputIdentityDocument".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInputIdentityDocumentBuilder { inner }
  }

  pub fn number(&self) -> &String { &self.number }

  pub fn expiry_date(&self) -> &Date { &self.expiry_date }

  pub fn front_side(&self) -> &InputFile { &self.front_side }

  pub fn reverse_side(&self) -> &InputFile { &self.reverse_side }

  pub fn selfie(&self) -> &InputFile { &self.selfie }

  pub fn translation(&self) -> &Vec<InputFile> { &self.translation }

}

#[doc(hidden)]
pub struct RTDInputIdentityDocumentBuilder {
  inner: InputIdentityDocument
}

impl RTDInputIdentityDocumentBuilder {
  pub fn build(&self) -> InputIdentityDocument { self.inner.clone() }

   
  pub fn number<T: AsRef<str>>(&mut self, number: T) -> &mut Self {
    self.inner.number = number.as_ref().to_string();
    self
  }

   
  pub fn expiry_date<T: AsRef<Date>>(&mut self, expiry_date: T) -> &mut Self {
    self.inner.expiry_date = expiry_date.as_ref().clone();
    self
  }

   
  pub fn front_side<T: AsRef<InputFile>>(&mut self, front_side: T) -> &mut Self {
    self.inner.front_side = front_side.as_ref().clone();
    self
  }

   
  pub fn reverse_side<T: AsRef<InputFile>>(&mut self, reverse_side: T) -> &mut Self {
    self.inner.reverse_side = reverse_side.as_ref().clone();
    self
  }

   
  pub fn selfie<T: AsRef<InputFile>>(&mut self, selfie: T) -> &mut Self {
    self.inner.selfie = selfie.as_ref().clone();
    self
  }

   
  pub fn translation(&mut self, translation: Vec<InputFile>) -> &mut Self {
    self.inner.translation = translation;
    self
  }

}

impl AsRef<InputIdentityDocument> for InputIdentityDocument {
  fn as_ref(&self) -> &InputIdentityDocument { self }
}

impl AsRef<InputIdentityDocument> for RTDInputIdentityDocumentBuilder {
  fn as_ref(&self) -> &InputIdentityDocument { &self.inner }
}



