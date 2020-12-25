
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about an encrypted Telegram Passport element; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EncryptedPassportElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of Telegram Passport element
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: PassportElementType,
  /// Encrypted JSON-encoded data about the user
  data: String,
  /// The front side of an identity document
  front_side: DatedFile,
  /// The reverse side of an identity document; may be null
  reverse_side: Option<DatedFile>,
  /// Selfie with the document; may be null
  selfie: Option<DatedFile>,
  /// List of files containing a certified English translation of the document
  translation: Vec<DatedFile>,
  /// List of attached files
  files: Vec<DatedFile>,
  /// Unencrypted data, phone number or email address
  value: String,
  /// Hash of the entire element
  hash: String,
  
}

impl RObject for EncryptedPassportElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "encryptedPassportElement" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl EncryptedPassportElement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEncryptedPassportElementBuilder {
    let mut inner = EncryptedPassportElement::default();
    inner.td_name = "encryptedPassportElement".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEncryptedPassportElementBuilder { inner }
  }

  pub fn type_(&self) -> &PassportElementType { &self.type_ }

  pub fn data(&self) -> &String { &self.data }

  pub fn front_side(&self) -> &DatedFile { &self.front_side }

  pub fn reverse_side(&self) -> &Option<DatedFile> { &self.reverse_side }

  pub fn selfie(&self) -> &Option<DatedFile> { &self.selfie }

  pub fn translation(&self) -> &Vec<DatedFile> { &self.translation }

  pub fn files(&self) -> &Vec<DatedFile> { &self.files }

  pub fn value(&self) -> &String { &self.value }

  pub fn hash(&self) -> &String { &self.hash }

}

#[doc(hidden)]
pub struct RTDEncryptedPassportElementBuilder {
  inner: EncryptedPassportElement
}

impl RTDEncryptedPassportElementBuilder {
  pub fn build(&self) -> EncryptedPassportElement { self.inner.clone() }

   
  pub fn type_<T: AsRef<PassportElementType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

   
  pub fn front_side<T: AsRef<DatedFile>>(&mut self, front_side: T) -> &mut Self {
    self.inner.front_side = front_side.as_ref().clone();
    self
  }

   
  pub fn reverse_side<T: AsRef<DatedFile>>(&mut self, reverse_side: T) -> &mut Self {
    self.inner.reverse_side = Some(reverse_side.as_ref().clone());
    self
  }

   
  pub fn selfie<T: AsRef<DatedFile>>(&mut self, selfie: T) -> &mut Self {
    self.inner.selfie = Some(selfie.as_ref().clone());
    self
  }

   
  pub fn translation(&mut self, translation: Vec<DatedFile>) -> &mut Self {
    self.inner.translation = translation;
    self
  }

   
  pub fn files(&mut self, files: Vec<DatedFile>) -> &mut Self {
    self.inner.files = files;
    self
  }

   
  pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().to_string();
    self
  }

   
  pub fn hash<T: AsRef<str>>(&mut self, hash: T) -> &mut Self {
    self.inner.hash = hash.as_ref().to_string();
    self
  }

}

impl AsRef<EncryptedPassportElement> for EncryptedPassportElement {
  fn as_ref(&self) -> &EncryptedPassportElement { self }
}

impl AsRef<EncryptedPassportElement> for RTDEncryptedPassportElementBuilder {
  fn as_ref(&self) -> &EncryptedPassportElement { &self.inner }
}



