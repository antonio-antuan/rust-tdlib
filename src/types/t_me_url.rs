
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a URL linking to an internal Telegram entity
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TMeUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL
  url: String,
  /// Type of the URL
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: TMeUrlType,
  
}

impl RObject for TMeUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tMeUrl" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TMeUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTMeUrlBuilder {
    let mut inner = TMeUrl::default();
    inner.td_name = "tMeUrl".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTMeUrlBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

  pub fn type_(&self) -> &TMeUrlType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDTMeUrlBuilder {
  inner: TMeUrl
}

impl RTDTMeUrlBuilder {
  pub fn build(&self) -> TMeUrl { self.inner.clone() }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

   
  pub fn type_<T: AsRef<TMeUrlType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<TMeUrl> for TMeUrl {
  fn as_ref(&self) -> &TMeUrl { self }
}

impl AsRef<TMeUrl> for RTDTMeUrlBuilder {
  fn as_ref(&self) -> &TMeUrl { &self.inner }
}



