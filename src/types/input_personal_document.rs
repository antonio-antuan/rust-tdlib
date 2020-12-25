
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// A personal document to be saved to Telegram Passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPersonalDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of files containing the pages of the document
  files: Vec<InputFile>,
  /// List of files containing a certified English translation of the document
  translation: Vec<InputFile>,
  
}

impl RObject for InputPersonalDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPersonalDocument" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl InputPersonalDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPersonalDocumentBuilder {
    let mut inner = InputPersonalDocument::default();
    inner.td_name = "inputPersonalDocument".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInputPersonalDocumentBuilder { inner }
  }

  pub fn files(&self) -> &Vec<InputFile> { &self.files }

  pub fn translation(&self) -> &Vec<InputFile> { &self.translation }

}

#[doc(hidden)]
pub struct RTDInputPersonalDocumentBuilder {
  inner: InputPersonalDocument
}

impl RTDInputPersonalDocumentBuilder {
  pub fn build(&self) -> InputPersonalDocument { self.inner.clone() }

   
  pub fn files(&mut self, files: Vec<InputFile>) -> &mut Self {
    self.inner.files = files;
    self
  }

   
  pub fn translation(&mut self, translation: Vec<InputFile>) -> &mut Self {
    self.inner.translation = translation;
    self
  }

}

impl AsRef<InputPersonalDocument> for InputPersonalDocument {
  fn as_ref(&self) -> &InputPersonalDocument { self }
}

impl AsRef<InputPersonalDocument> for RTDInputPersonalDocumentBuilder {
  fn as_ref(&self) -> &InputPersonalDocument { &self.inner }
}



