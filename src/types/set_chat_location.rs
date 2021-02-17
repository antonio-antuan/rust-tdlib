use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the location of a chat. Available only for some location-based supergroups, use supergroupFullInfo.can_set_location to check whether the method is allowed to use
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// New location for the chat; must be valid and not null
    location: ChatLocation,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatLocation {}

impl SetChatLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetChatLocationBuilder {
        let mut inner = SetChatLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatLocation".to_string();

        RTDSetChatLocationBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn location(&self) -> &ChatLocation {
        &self.location
    }
}

#[doc(hidden)]
pub struct RTDSetChatLocationBuilder {
    inner: SetChatLocation,
}

impl RTDSetChatLocationBuilder {
    pub fn build(&self) -> SetChatLocation {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn location<T: AsRef<ChatLocation>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
        self
    }
}

impl AsRef<SetChatLocation> for SetChatLocation {
    fn as_ref(&self) -> &SetChatLocation {
        self
    }
}

impl AsRef<SetChatLocation> for RTDSetChatLocationBuilder {
    fn as_ref(&self) -> &SetChatLocation {
        &self.inner
    }
}
