use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about one session in a Telegram application used by the current user. Sessions must be shown to the user in the returned order
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Session identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// True, if this session is the current session

    #[serde(default)]
    is_current: bool,
    /// True, if a password is needed to complete authorization of the session

    #[serde(default)]
    is_password_pending: bool,
    /// True, if incoming secret chats can be accepted by the session

    #[serde(default)]
    can_accept_secret_chats: bool,
    /// True, if incoming calls can be accepted by the session

    #[serde(default)]
    can_accept_calls: bool,
    /// Telegram API identifier, as provided by the application

    #[serde(default)]
    api_id: i32,
    /// Name of the application, as provided by the application

    #[serde(default)]
    application_name: String,
    /// The version of the application, as provided by the application

    #[serde(default)]
    application_version: String,
    /// True, if the application is an official application or uses the api_id of an official application

    #[serde(default)]
    is_official_application: bool,
    /// Model of the device the application has been run or is running on, as provided by the application

    #[serde(default)]
    device_model: String,
    /// Operating system the application has been run or is running on, as provided by the application

    #[serde(default)]
    platform: String,
    /// Version of the operating system the application has been run or is running on, as provided by the application

    #[serde(default)]
    system_version: String,
    /// Point in time (Unix timestamp) when the user has logged in

    #[serde(default)]
    log_in_date: i32,
    /// Point in time (Unix timestamp) when the session was last used

    #[serde(default)]
    last_active_date: i32,
    /// IP address from which the session was created, in human-readable format

    #[serde(default)]
    ip: String,
    /// A two-letter country code for the country from which the session was created, based on the IP address

    #[serde(default)]
    country: String,
    /// Region code from which the session was created, based on the IP address

    #[serde(default)]
    region: String,
}

impl RObject for Session {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Session {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SessionBuilder {
        let mut inner = Session::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SessionBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn is_current(&self) -> bool {
        self.is_current
    }

    pub fn is_password_pending(&self) -> bool {
        self.is_password_pending
    }

    pub fn can_accept_secret_chats(&self) -> bool {
        self.can_accept_secret_chats
    }

    pub fn can_accept_calls(&self) -> bool {
        self.can_accept_calls
    }

    pub fn api_id(&self) -> i32 {
        self.api_id
    }

    pub fn application_name(&self) -> &String {
        &self.application_name
    }

    pub fn application_version(&self) -> &String {
        &self.application_version
    }

    pub fn is_official_application(&self) -> bool {
        self.is_official_application
    }

    pub fn device_model(&self) -> &String {
        &self.device_model
    }

    pub fn platform(&self) -> &String {
        &self.platform
    }

    pub fn system_version(&self) -> &String {
        &self.system_version
    }

    pub fn log_in_date(&self) -> i32 {
        self.log_in_date
    }

    pub fn last_active_date(&self) -> i32 {
        self.last_active_date
    }

    pub fn ip(&self) -> &String {
        &self.ip
    }

    pub fn country(&self) -> &String {
        &self.country
    }

    pub fn region(&self) -> &String {
        &self.region
    }
}

#[doc(hidden)]
pub struct SessionBuilder {
    inner: Session,
}

#[deprecated]
pub type RTDSessionBuilder = SessionBuilder;

impl SessionBuilder {
    pub fn build(&self) -> Session {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
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

    pub fn can_accept_secret_chats(&mut self, can_accept_secret_chats: bool) -> &mut Self {
        self.inner.can_accept_secret_chats = can_accept_secret_chats;
        self
    }

    pub fn can_accept_calls(&mut self, can_accept_calls: bool) -> &mut Self {
        self.inner.can_accept_calls = can_accept_calls;
        self
    }

    pub fn api_id(&mut self, api_id: i32) -> &mut Self {
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

    pub fn log_in_date(&mut self, log_in_date: i32) -> &mut Self {
        self.inner.log_in_date = log_in_date;
        self
    }

    pub fn last_active_date(&mut self, last_active_date: i32) -> &mut Self {
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
    fn as_ref(&self) -> &Session {
        self
    }
}

impl AsRef<Session> for SessionBuilder {
    fn as_ref(&self) -> &Session {
        &self.inner
    }
}
