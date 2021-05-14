use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a newest pinned message in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatPinnedMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat the message belongs to
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatPinnedMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatPinnedMessage {}

impl GetChatPinnedMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetChatPinnedMessageBuilder {
        let mut inner = GetChatPinnedMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatPinnedMessage".to_string();

        RTDGetChatPinnedMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDGetChatPinnedMessageBuilder {
    inner: GetChatPinnedMessage,
}

impl RTDGetChatPinnedMessageBuilder {
    pub fn build(&self) -> GetChatPinnedMessage {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetChatPinnedMessage> for GetChatPinnedMessage {
    fn as_ref(&self) -> &GetChatPinnedMessage {
        self
    }
}

impl AsRef<GetChatPinnedMessage> for RTDGetChatPinnedMessageBuilder {
    fn as_ref(&self) -> &GetChatPinnedMessage {
        &self.inner
    }
}
