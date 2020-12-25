
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a proxy server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Proxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the proxy
  id: i64,
  /// Proxy server IP address
  server: String,
  /// Proxy server port
  port: i64,
  /// Point in time (Unix timestamp) when the proxy was last used; 0 if never
  last_used_date: i64,
  /// True, if the proxy is enabled now
  is_enabled: bool,
  /// Type of the proxy
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: ProxyType,
  
}

impl RObject for Proxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "proxy" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Proxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDProxyBuilder {
    let mut inner = Proxy::default();
    inner.td_name = "proxy".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDProxyBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

  pub fn server(&self) -> &String { &self.server }

  pub fn port(&self) -> i64 { self.port }

  pub fn last_used_date(&self) -> i64 { self.last_used_date }

  pub fn is_enabled(&self) -> bool { self.is_enabled }

  pub fn type_(&self) -> &ProxyType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDProxyBuilder {
  inner: Proxy
}

impl RTDProxyBuilder {
  pub fn build(&self) -> Proxy { self.inner.clone() }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn server<T: AsRef<str>>(&mut self, server: T) -> &mut Self {
    self.inner.server = server.as_ref().to_string();
    self
  }

   
  pub fn port(&mut self, port: i64) -> &mut Self {
    self.inner.port = port;
    self
  }

   
  pub fn last_used_date(&mut self, last_used_date: i64) -> &mut Self {
    self.inner.last_used_date = last_used_date;
    self
  }

   
  pub fn is_enabled(&mut self, is_enabled: bool) -> &mut Self {
    self.inner.is_enabled = is_enabled;
    self
  }

   
  pub fn type_<T: AsRef<ProxyType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<Proxy> for Proxy {
  fn as_ref(&self) -> &Proxy { self }
}

impl AsRef<Proxy> for RTDProxyBuilder {
  fn as_ref(&self) -> &Proxy { &self.inner }
}



