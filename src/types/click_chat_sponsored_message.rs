use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that the user opened the sponsored chat via the button, the name, the photo, or a mention in the sponsored message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClickChatSponsoredMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier of the sponsored message

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the sponsored message

    #[serde(default)]
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ClickChatSponsoredMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ClickChatSponsoredMessage {}

impl ClickChatSponsoredMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ClickChatSponsoredMessageBuilder {
        let mut inner = ClickChatSponsoredMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "clickChatSponsoredMessage".to_string();

        ClickChatSponsoredMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct ClickChatSponsoredMessageBuilder {
    inner: ClickChatSponsoredMessage,
}

#[deprecated]
pub type RTDClickChatSponsoredMessageBuilder = ClickChatSponsoredMessageBuilder;

impl ClickChatSponsoredMessageBuilder {
    pub fn build(&self) -> ClickChatSponsoredMessage {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }
}

impl AsRef<ClickChatSponsoredMessage> for ClickChatSponsoredMessage {
    fn as_ref(&self) -> &ClickChatSponsoredMessage {
        self
    }
}

impl AsRef<ClickChatSponsoredMessage> for ClickChatSponsoredMessageBuilder {
    fn as_ref(&self) -> &ClickChatSponsoredMessage {
        &self.inner
    }
}
