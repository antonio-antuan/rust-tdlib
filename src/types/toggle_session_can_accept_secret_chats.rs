use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether a session can accept incoming secret chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSessionCanAcceptSecretChats {
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
    /// True, if incoming secret chats can be accepted by the session

    #[serde(default)]
    can_accept_secret_chats: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleSessionCanAcceptSecretChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleSessionCanAcceptSecretChats {}

impl ToggleSessionCanAcceptSecretChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleSessionCanAcceptSecretChatsBuilder {
        let mut inner = ToggleSessionCanAcceptSecretChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleSessionCanAcceptSecretChats".to_string();

        ToggleSessionCanAcceptSecretChatsBuilder { inner }
    }

    pub fn session_id(&self) -> i64 {
        self.session_id
    }

    pub fn can_accept_secret_chats(&self) -> bool {
        self.can_accept_secret_chats
    }
}

#[doc(hidden)]
pub struct ToggleSessionCanAcceptSecretChatsBuilder {
    inner: ToggleSessionCanAcceptSecretChats,
}

#[deprecated]
pub type RTDToggleSessionCanAcceptSecretChatsBuilder = ToggleSessionCanAcceptSecretChatsBuilder;

impl ToggleSessionCanAcceptSecretChatsBuilder {
    pub fn build(&self) -> ToggleSessionCanAcceptSecretChats {
        self.inner.clone()
    }

    pub fn session_id(&mut self, session_id: i64) -> &mut Self {
        self.inner.session_id = session_id;
        self
    }

    pub fn can_accept_secret_chats(&mut self, can_accept_secret_chats: bool) -> &mut Self {
        self.inner.can_accept_secret_chats = can_accept_secret_chats;
        self
    }
}

impl AsRef<ToggleSessionCanAcceptSecretChats> for ToggleSessionCanAcceptSecretChats {
    fn as_ref(&self) -> &ToggleSessionCanAcceptSecretChats {
        self
    }
}

impl AsRef<ToggleSessionCanAcceptSecretChats> for ToggleSessionCanAcceptSecretChatsBuilder {
    fn as_ref(&self) -> &ToggleSessionCanAcceptSecretChats {
        &self.inner
    }
}
