
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about one session in a Telegram application used by the current user. Sessions should be shown to the user in the returned order
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Session {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Session identifier
  id: isize,
  /// True, if this session is the current session
  is_current: bool,
  /// True, if a password is needed to complete authorization of the session
  is_password_pending: bool,
  /// Telegram API identifier, as provided by the application
  api_id: i64,
  /// Name of the application, as provided by the application
  application_name: String,
  /// The version of the application, as provided by the application
  application_version: String,
  /// True, if the application is an official application or uses the api_id of an official application
  is_official_application: bool,
  /// Model of the device the application has been run or is running on, as provided by the application
  device_model: String,
  /// Operating system the application has been run or is running on, as provided by the application
  platform: String,
  /// Version of the operating system the application has been run or is running on, as provided by the application
  system_version: String,
  /// Point in time (Unix timestamp) when the user has logged in
  log_in_date: i64,
  /// Point in time (Unix timestamp) when the session was last used
  last_active_date: i64,
  /// IP address from which the session was created, in human-readable format
  ip: String,
  /// A two-letter country code for the country from which the session was created, based on the IP address
  country: String,
  /// Region code from which the session was created, based on the IP address
  region: String,
  
}

impl RObject for Session {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "session" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Session {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSessionBuilder {
    let mut inner = Session::default();
    inner.td_name = "session".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSessionBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn is_current(&self) -> bool { self.is_current }

  pub fn is_password_pending(&self) -> bool { self.is_password_pending }

  pub fn api_id(&self) -> i64 { self.api_id }

  pub fn application_name(&self) -> &String { &self.application_name }

  pub fn application_version(&self) -> &String { &self.application_version }

  pub fn is_official_application(&self) -> bool { self.is_official_application }

  pub fn device_model(&self) -> &String { &self.device_model }

  pub fn platform(&self) -> &String { &self.platform }

  pub fn system_version(&self) -> &String { &self.system_version }

  pub fn log_in_date(&self) -> i64 { self.log_in_date }

  pub fn last_active_date(&self) -> i64 { self.last_active_date }

  pub fn ip(&self) -> &String { &self.ip }

  pub fn country(&self) -> &String { &self.country }

  pub fn region(&self) -> &String { &self.region }

}

#[doc(hidden)]
pub struct RTDSessionBuilder {
  inner: Session
}

impl RTDSessionBuilder {
  pub fn build(&self) -> Session { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn is_current(&mut self, is_current: bool) -> &mut Self {
    self.inner.is_current = is_current;
    self
  }

   
  pub fn is_password_pending(&mut self, is_password_pending: bool) -> &mut Self {
    self.inner.is_password_pending = is_password_pending;
    self
  }

   
  pub fn api_id(&mut self, api_id: i64) -> &mut Self {
    self.inner.api_id = api_id;
    self
  }

   
  pub fn application_name<T: AsRef<str>>(&mut self, application_name: T) -> &mut Self {
    self.inner.application_name = application_name.as_ref().to_string();
    self
  }

   
  pub fn application_version<T: AsRef<str>>(&mut self, application_version: T) -> &mut Self {
    self.inner.application_version = application_version.as_ref().to_string();
    self
  }

   
  pub fn is_official_application(&mut self, is_official_application: bool) -> &mut Self {
    self.inner.is_official_application = is_official_application;
    self
  }

   
  pub fn device_model<T: AsRef<str>>(&mut self, device_model: T) -> &mut Self {
    self.inner.device_model = device_model.as_ref().to_string();
    self
  }

   
  pub fn platform<T: AsRef<str>>(&mut self, platform: T) -> &mut Self {
    self.inner.platform = platform.as_ref().to_string();
    self
  }

   
  pub fn system_version<T: AsRef<str>>(&mut self, system_version: T) -> &mut Self {
    self.inner.system_version = system_version.as_ref().to_string();
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

   
  pub fn country<T: AsRef<str>>(&mut self, country: T) -> &mut Self {
    self.inner.country = country.as_ref().to_string();
    self
  }

   
  pub fn region<T: AsRef<str>>(&mut self, region: T) -> &mut Self {
    self.inner.region = region.as_ref().to_string();
    self
  }

}

impl AsRef<Session> for Session {
  fn as_ref(&self) -> &Session { self }
}

impl AsRef<Session> for RTDSessionBuilder {
  fn as_ref(&self) -> &Session { &self.inner }
}



