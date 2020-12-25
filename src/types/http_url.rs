
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains an HTTP URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HttpUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The URL
  url: String,
  
}

impl RObject for HttpUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "httpUrl" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl HttpUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDHttpUrlBuilder {
    let mut inner = HttpUrl::default();
    inner.td_name = "httpUrl".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDHttpUrlBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

}

#[doc(hidden)]
pub struct RTDHttpUrlBuilder {
  inner: HttpUrl
}

impl RTDHttpUrlBuilder {
  pub fn build(&self) -> HttpUrl { self.inner.clone() }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

}

impl AsRef<HttpUrl> for HttpUrl {
  fn as_ref(&self) -> &HttpUrl { self }
}

impl AsRef<HttpUrl> for RTDHttpUrlBuilder {
  fn as_ref(&self) -> &HttpUrl { &self.inner }
}



