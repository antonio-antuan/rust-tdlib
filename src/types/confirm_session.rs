use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Confirms an unconfirmed session of the current user from another device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfirmSession {
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
    session_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ConfirmSession {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ConfirmSession {}

impl ConfirmSession {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ConfirmSessionBuilder {
        let mut inner = ConfirmSession::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "confirmSession".to_string();

        ConfirmSessionBuilder { inner }
    }

    pub fn session_id(&self) -> i64 {
        self.session_id
    }
}

#[doc(hidden)]
pub struct ConfirmSessionBuilder {
    inner: ConfirmSession,
}

#[deprecated]
pub type RTDConfirmSessionBuilder = ConfirmSessionBuilder;

impl ConfirmSessionBuilder {
    pub fn build(&self) -> ConfirmSession {
        self.inner.clone()
    }

    pub fn session_id(&mut self, session_id: i64) -> &mut Self {
        self.inner.session_id = session_id;
        self
    }
}

impl AsRef<ConfirmSession> for ConfirmSession {
    fn as_ref(&self) -> &ConfirmSession {
        self
    }
}

impl AsRef<ConfirmSession> for ConfirmSessionBuilder {
    fn as_ref(&self) -> &ConfirmSession {
        &self.inner
    }
}
