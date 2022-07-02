use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns list of chat administrators with number of their invite links. Requires owner privileges in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatInviteLinkCounts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatInviteLinkCounts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatInviteLinkCounts {}

impl GetChatInviteLinkCounts {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatInviteLinkCountsBuilder {
        let mut inner = GetChatInviteLinkCounts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatInviteLinkCounts".to_string();

        GetChatInviteLinkCountsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct GetChatInviteLinkCountsBuilder {
    inner: GetChatInviteLinkCounts,
}

#[deprecated]
pub type RTDGetChatInviteLinkCountsBuilder = GetChatInviteLinkCountsBuilder;

impl GetChatInviteLinkCountsBuilder {
    pub fn build(&self) -> GetChatInviteLinkCounts {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetChatInviteLinkCounts> for GetChatInviteLinkCounts {
    fn as_ref(&self) -> &GetChatInviteLinkCounts {
        self
    }
}

impl AsRef<GetChatInviteLinkCounts> for GetChatInviteLinkCountsBuilder {
    fn as_ref(&self) -> &GetChatInviteLinkCounts {
        &self.inner
    }
}
