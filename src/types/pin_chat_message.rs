use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Pins a message in a chat; requires can_pin_messages rights or can_edit_messages rights in the channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PinChatMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat
    chat_id: i64,
    /// Identifier of the new pinned message
    message_id: i64,
    /// True, if there should be no notification about the pinned message. Notifications are always disabled in channels and private chats
    disable_notification: bool,
    /// True, if the message needs to be pinned for one side only; private chats only
    only_for_self: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for PinChatMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for PinChatMessage {}

impl PinChatMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPinChatMessageBuilder {
        let mut inner = PinChatMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "pinChatMessage".to_string();

        RTDPinChatMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn disable_notification(&self) -> bool {
        self.disable_notification
    }

    pub fn only_for_self(&self) -> bool {
        self.only_for_self
    }
}

#[doc(hidden)]
pub struct RTDPinChatMessageBuilder {
    inner: PinChatMessage,
}

impl RTDPinChatMessageBuilder {
    pub fn build(&self) -> PinChatMessage {
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

    pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
        self.inner.disable_notification = disable_notification;
        self
    }

    pub fn only_for_self(&mut self, only_for_self: bool) -> &mut Self {
        self.inner.only_for_self = only_for_self;
        self
    }
}

impl AsRef<PinChatMessage> for PinChatMessage {
    fn as_ref(&self) -> &PinChatMessage {
        self
    }
}

impl AsRef<PinChatMessage> for RTDPinChatMessageBuilder {
    fn as_ref(&self) -> &PinChatMessage {
        &self.inner
    }
}
