
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// A personal document, containing some information about a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonalDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of files containing the pages of the document
  files: Vec<DatedFile>,
  /// List of files containing a certified English translation of the document
  translation: Vec<DatedFile>,
  
}

impl RObject for PersonalDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "personalDocument" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PersonalDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPersonalDocumentBuilder {
    let mut inner = PersonalDocument::default();
    inner.td_name = "personalDocument".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPersonalDocumentBuilder { inner }
  }

  pub fn files(&self) -> &Vec<DatedFile> { &self.files }

  pub fn translation(&self) -> &Vec<DatedFile> { &self.translation }

}

#[doc(hidden)]
pub struct RTDPersonalDocumentBuilder {
  inner: PersonalDocument
}

impl RTDPersonalDocumentBuilder {
  pub fn build(&self) -> PersonalDocument { self.inner.clone() }

   
  pub fn files(&mut self, files: Vec<DatedFile>) -> &mut Self {
    self.inner.files = files;
    self
  }

   
  pub fn translation(&mut self, translation: Vec<DatedFile>) -> &mut Self {
    self.inner.translation = translation;
    self
  }

}

impl AsRef<PersonalDocument> for PersonalDocument {
  fn as_ref(&self) -> &PersonalDocument { self }
}

impl AsRef<PersonalDocument> for RTDPersonalDocumentBuilder {
  fn as_ref(&self) -> &PersonalDocument { &self.inner }
}



