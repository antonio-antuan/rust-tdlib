use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that a message with an animated emoji was clicked by the user. Returns a big animated sticker to be played or a 404 error if usual animation needs to be played
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClickAnimatedEmojiMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier of the message

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the clicked message

    #[serde(default)]
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ClickAnimatedEmojiMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ClickAnimatedEmojiMessage {}

impl ClickAnimatedEmojiMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ClickAnimatedEmojiMessageBuilder {
        let mut inner = ClickAnimatedEmojiMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "clickAnimatedEmojiMessage".to_string();

        ClickAnimatedEmojiMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct ClickAnimatedEmojiMessageBuilder {
    inner: ClickAnimatedEmojiMessage,
}

#[deprecated]
pub type RTDClickAnimatedEmojiMessageBuilder = ClickAnimatedEmojiMessageBuilder;

impl ClickAnimatedEmojiMessageBuilder {
    pub fn build(&self) -> ClickAnimatedEmojiMessage {
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

impl AsRef<ClickAnimatedEmojiMessage> for ClickAnimatedEmojiMessage {
    fn as_ref(&self) -> &ClickAnimatedEmojiMessage {
        self
    }
}

impl AsRef<ClickAnimatedEmojiMessage> for ClickAnimatedEmojiMessageBuilder {
    fn as_ref(&self) -> &ClickAnimatedEmojiMessage {
        &self.inner
    }
}
