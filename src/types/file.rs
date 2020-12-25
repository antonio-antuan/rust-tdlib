
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct File {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique file identifier
  id: i64,
  /// File size; 0 if unknown
  size: i64,
  /// Expected file size in case the exact file size is unknown, but an approximate size is known. Can be used to show download/upload progress
  expected_size: i64,
  /// Information about the local copy of the file
  local: LocalFile,
  /// Information about the remote copy of the file
  remote: RemoteFile,
  
}

impl RObject for File {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "file" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl File {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileBuilder {
    let mut inner = File::default();
    inner.td_name = "file".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDFileBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

  pub fn size(&self) -> i64 { self.size }

  pub fn expected_size(&self) -> i64 { self.expected_size }

  pub fn local(&self) -> &LocalFile { &self.local }

  pub fn remote(&self) -> &RemoteFile { &self.remote }

}

#[doc(hidden)]
pub struct RTDFileBuilder {
  inner: File
}

impl RTDFileBuilder {
  pub fn build(&self) -> File { self.inner.clone() }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn size(&mut self, size: i64) -> &mut Self {
    self.inner.size = size;
    self
  }

   
  pub fn expected_size(&mut self, expected_size: i64) -> &mut Self {
    self.inner.expected_size = expected_size;
    self
  }

   
  pub fn local<T: AsRef<LocalFile>>(&mut self, local: T) -> &mut Self {
    self.inner.local = local.as_ref().clone();
    self
  }

   
  pub fn remote<T: AsRef<RemoteFile>>(&mut self, remote: T) -> &mut Self {
    self.inner.remote = remote.as_ref().clone();
    self
  }

}

impl AsRef<File> for File {
  fn as_ref(&self) -> &File { self }
}

impl AsRef<File> for RTDFileBuilder {
  fn as_ref(&self) -> &File { &self.inner }
}



