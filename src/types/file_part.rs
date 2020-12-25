
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a part of a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilePart {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// File bytes
  data: String,
  
}

impl RObject for FilePart {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "filePart" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl FilePart {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFilePartBuilder {
    let mut inner = FilePart::default();
    inner.td_name = "filePart".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDFilePartBuilder { inner }
  }

  pub fn data(&self) -> &String { &self.data }

}

#[doc(hidden)]
pub struct RTDFilePartBuilder {
  inner: FilePart
}

impl RTDFilePartBuilder {
  pub fn build(&self) -> FilePart { self.inner.clone() }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

}

impl AsRef<FilePart> for FilePart {
  fn as_ref(&self) -> &FilePart { self }
}

impl AsRef<FilePart> for RTDFilePartBuilder {
  fn as_ref(&self) -> &FilePart { &self.inner }
}



