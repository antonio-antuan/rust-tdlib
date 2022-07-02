use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a forwarded message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Origin of a forwarded message

    #[serde(skip_serializing_if = "MessageForwardOrigin::_is_default")]
    origin: MessageForwardOrigin,
    /// Point in time (Unix timestamp) when the message was originally sent

    #[serde(default)]
    date: i32,
    /// The type of a public service announcement for the forwarded message

    #[serde(default)]
    public_service_announcement_type: String,
    /// For messages forwarded to the chat with the current user (Saved Messages), to the Replies bot chat, or to the channel's discussion group, the identifier of the chat from which the message was forwarded last time; 0 if unknown

    #[serde(default)]
    from_chat_id: i64,
    /// For messages forwarded to the chat with the current user (Saved Messages), to the Replies bot chat, or to the channel's discussion group, the identifier of the original message from which the new message was forwarded last time; 0 if unknown

    #[serde(default)]
    from_message_id: i64,
}

impl RObject for MessageForwardInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageForwardInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageForwardInfoBuilder {
        let mut inner = MessageForwardInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageForwardInfoBuilder { inner }
    }

    pub fn origin(&self) -> &MessageForwardOrigin {
        &self.origin
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn public_service_announcement_type(&self) -> &String {
        &self.public_service_announcement_type
    }

    pub fn from_chat_id(&self) -> i64 {
        self.from_chat_id
    }

    pub fn from_message_id(&self) -> i64 {
        self.from_message_id
    }
}

#[doc(hidden)]
pub struct MessageForwardInfoBuilder {
    inner: MessageForwardInfo,
}

#[deprecated]
pub type RTDMessageForwardInfoBuilder = MessageForwardInfoBuilder;

impl MessageForwardInfoBuilder {
    pub fn build(&self) -> MessageForwardInfo {
        self.inner.clone()
    }

    pub fn origin<T: AsRef<MessageForwardOrigin>>(&mut self, origin: T) -> &mut Self {
        self.inner.origin = origin.as_ref().clone();
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn public_service_announcement_type<T: AsRef<str>>(
        &mut self,
        public_service_announcement_type: T,
    ) -> &mut Self {
        self.inner.public_service_announcement_type =
            public_service_announcement_type.as_ref().to_string();
        self
    }

    pub fn from_chat_id(&mut self, from_chat_id: i64) -> &mut Self {
        self.inner.from_chat_id = from_chat_id;
        self
    }

    pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self {
        self.inner.from_message_id = from_message_id;
        self
    }
}

impl AsRef<MessageForwardInfo> for MessageForwardInfo {
    fn as_ref(&self) -> &MessageForwardInfo {
        self
    }
}

impl AsRef<MessageForwardInfo> for MessageForwardInfoBuilder {
    fn as_ref(&self) -> &MessageForwardInfo {
        &self.inner
    }
}
