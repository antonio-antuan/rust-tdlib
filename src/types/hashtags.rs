
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of hashtags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Hashtags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A list of hashtags
  hashtags: Vec<String>,
  
}

impl RObject for Hashtags {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "hashtags" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Hashtags {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDHashtagsBuilder {
    let mut inner = Hashtags::default();
    inner.td_name = "hashtags".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDHashtagsBuilder { inner }
  }

  pub fn hashtags(&self) -> &Vec<String> { &self.hashtags }

}

#[doc(hidden)]
pub struct RTDHashtagsBuilder {
  inner: Hashtags
}

impl RTDHashtagsBuilder {
  pub fn build(&self) -> Hashtags { self.inner.clone() }

   
  pub fn hashtags(&mut self, hashtags: Vec<String>) -> &mut Self {
    self.inner.hashtags = hashtags;
    self
  }

}

impl AsRef<Hashtags> for Hashtags {
  fn as_ref(&self) -> &Hashtags { self }
}

impl AsRef<Hashtags> for RTDHashtagsBuilder {
  fn as_ref(&self) -> &Hashtags { &self.inner }
}



