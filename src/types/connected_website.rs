
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about one website the current user is logged in with Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectedWebsite {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Website identifier
  id: isize,
  /// The domain name of the website
  domain_name: String,
  /// User identifier of a bot linked with the website
  bot_user_id: i64,
  /// The version of a browser used to log in
  browser: String,
  /// Operating system the browser is running on
  platform: String,
  /// Point in time (Unix timestamp) when the user was logged in
  log_in_date: i64,
  /// Point in time (Unix timestamp) when obtained authorization was last used
  last_active_date: i64,
  /// IP address from which the user was logged in, in human-readable format
  ip: String,
  /// Human-readable description of a country and a region, from which the user was logged in, based on the IP address
  location: String,
  
}

impl RObject for ConnectedWebsite {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectedWebsite" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ConnectedWebsite {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDConnectedWebsiteBuilder {
    let mut inner = ConnectedWebsite::default();
    inner.td_name = "connectedWebsite".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDConnectedWebsiteBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn domain_name(&self) -> &String { &self.domain_name }

  pub fn bot_user_id(&self) -> i64 { self.bot_user_id }

  pub fn browser(&self) -> &String { &self.browser }

  pub fn platform(&self) -> &String { &self.platform }

  pub fn log_in_date(&self) -> i64 { self.log_in_date }

  pub fn last_active_date(&self) -> i64 { self.last_active_date }

  pub fn ip(&self) -> &String { &self.ip }

  pub fn location(&self) -> &String { &self.location }

}

#[doc(hidden)]
pub struct RTDConnectedWebsiteBuilder {
  inner: ConnectedWebsite
}

impl RTDConnectedWebsiteBuilder {
  pub fn build(&self) -> ConnectedWebsite { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn domain_name<T: AsRef<str>>(&mut self, domain_name: T) -> &mut Self {
    self.inner.domain_name = domain_name.as_ref().to_string();
    self
  }

   
  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

   
  pub fn browser<T: AsRef<str>>(&mut self, browser: T) -> &mut Self {
    self.inner.browser = browser.as_ref().to_string();
    self
  }

   
  pub fn platform<T: AsRef<str>>(&mut self, platform: T) -> &mut Self {
    self.inner.platform = platform.as_ref().to_string();
    self
  }

   
  pub fn log_in_date(&mut self, log_in_date: i64) -> &mut Self {
    self.inner.log_in_date = log_in_date;
    self
  }

   
  pub fn last_active_date(&mut self, last_active_date: i64) -> &mut Self {
    self.inner.last_active_date = last_active_date;
    self
  }

   
  pub fn ip<T: AsRef<str>>(&mut self, ip: T) -> &mut Self {
    self.inner.ip = ip.as_ref().to_string();
    self
  }

   
  pub fn location<T: AsRef<str>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().to_string();
    self
  }

}

impl AsRef<ConnectedWebsite> for ConnectedWebsite {
  fn as_ref(&self) -> &ConnectedWebsite { self }
}

impl AsRef<ConnectedWebsite> for RTDConnectedWebsiteBuilder {
  fn as_ref(&self) -> &ConnectedWebsite { &self.inner }
}



