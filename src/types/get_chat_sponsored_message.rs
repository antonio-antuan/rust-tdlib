use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns sponsored message to be shown in a chat; for channel chats only. Returns a 404 error if there is no sponsored message in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatSponsoredMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatSponsoredMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatSponsoredMessage {}

impl GetChatSponsoredMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetChatSponsoredMessageBuilder {
        let mut inner = GetChatSponsoredMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatSponsoredMessage".to_string();

        RTDGetChatSponsoredMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDGetChatSponsoredMessageBuilder {
    inner: GetChatSponsoredMessage,
}

impl RTDGetChatSponsoredMessageBuilder {
    pub fn build(&self) -> GetChatSponsoredMessage {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetChatSponsoredMessage> for GetChatSponsoredMessage {
    fn as_ref(&self) -> &GetChatSponsoredMessage {
        self
    }
}

impl AsRef<GetChatSponsoredMessage> for RTDGetChatSponsoredMessageBuilder {
    fn as_ref(&self) -> &GetChatSponsoredMessage {
        &self.inner
    }
}
