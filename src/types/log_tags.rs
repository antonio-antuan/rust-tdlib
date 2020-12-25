
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of available TDLib internal log tags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogTags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of log tags
  tags: Vec<String>,
  
}

impl RObject for LogTags {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logTags" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl LogTags {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLogTagsBuilder {
    let mut inner = LogTags::default();
    inner.td_name = "logTags".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLogTagsBuilder { inner }
  }

  pub fn tags(&self) -> &Vec<String> { &self.tags }

}

#[doc(hidden)]
pub struct RTDLogTagsBuilder {
  inner: LogTags
}

impl RTDLogTagsBuilder {
  pub fn build(&self) -> LogTags { self.inner.clone() }

   
  pub fn tags(&mut self, tags: Vec<String>) -> &mut Self {
    self.inner.tags = tags;
    self
  }

}

impl AsRef<LogTags> for LogTags {
  fn as_ref(&self) -> &LogTags { self }
}

impl AsRef<LogTags> for RTDLogTagsBuilder {
  fn as_ref(&self) -> &LogTags { &self.inner }
}



