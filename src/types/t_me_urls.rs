
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of t.me URLs
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TMeUrls {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of URLs
  urls: Vec<TMeUrl>,
  
}

impl RObject for TMeUrls {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tMeUrls" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TMeUrls {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTMeUrlsBuilder {
    let mut inner = TMeUrls::default();
    inner.td_name = "tMeUrls".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTMeUrlsBuilder { inner }
  }

  pub fn urls(&self) -> &Vec<TMeUrl> { &self.urls }

}

#[doc(hidden)]
pub struct RTDTMeUrlsBuilder {
  inner: TMeUrls
}

impl RTDTMeUrlsBuilder {
  pub fn build(&self) -> TMeUrls { self.inner.clone() }

   
  pub fn urls(&mut self, urls: Vec<TMeUrl>) -> &mut Self {
    self.inner.urls = urls;
    self
  }

}

impl AsRef<TMeUrls> for TMeUrls {
  fn as_ref(&self) -> &TMeUrls { self }
}

impl AsRef<TMeUrls> for RTDTMeUrlsBuilder {
  fn as_ref(&self) -> &TMeUrls { &self.inner }
}



