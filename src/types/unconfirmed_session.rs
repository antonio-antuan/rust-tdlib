use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an unconfirmed session
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnconfirmedSession {
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
    /// Point in time (Unix timestamp) when the user has logged in

    #[serde(default)]
    log_in_date: i32,
    /// Model of the device that was used for the session creation, as provided by the application

    #[serde(default)]
    device_model: String,
    /// A human-readable description of the location from which the session was created, based on the IP address

    #[serde(default)]
    location: String,
}

impl RObject for UnconfirmedSession {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl UnconfirmedSession {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UnconfirmedSessionBuilder {
        let mut inner = UnconfirmedSession::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UnconfirmedSessionBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn log_in_date(&self) -> i32 {
        self.log_in_date
    }

    pub fn device_model(&self) -> &String {
        &self.device_model
    }

    pub fn location(&self) -> &String {
        &self.location
    }
}

#[doc(hidden)]
pub struct UnconfirmedSessionBuilder {
    inner: UnconfirmedSession,
}

#[deprecated]
pub type RTDUnconfirmedSessionBuilder = UnconfirmedSessionBuilder;

impl UnconfirmedSessionBuilder {
    pub fn build(&self) -> UnconfirmedSession {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn log_in_date(&mut self, log_in_date: i32) -> &mut Self {
        self.inner.log_in_date = log_in_date;
        self
    }

    pub fn device_model<T: AsRef<str>>(&mut self, device_model: T) -> &mut Self {
        self.inner.device_model = device_model.as_ref().to_string();
        self
    }

    pub fn location<T: AsRef<str>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().to_string();
        self
    }
}

impl AsRef<UnconfirmedSession> for UnconfirmedSession {
    fn as_ref(&self) -> &UnconfirmedSession {
        self
    }
}

impl AsRef<UnconfirmedSession> for UnconfirmedSessionBuilder {
    fn as_ref(&self) -> &UnconfirmedSession {
        &self.inner
    }
}
