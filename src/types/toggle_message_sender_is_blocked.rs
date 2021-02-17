use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the block state of a message sender. Currently, only users and supergroup chats can be blocked
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleMessageSenderIsBlocked {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message Sender

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender: MessageSender,
    /// New value of is_blocked
    is_blocked: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleMessageSenderIsBlocked {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleMessageSenderIsBlocked {}

impl ToggleMessageSenderIsBlocked {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDToggleMessageSenderIsBlockedBuilder {
        let mut inner = ToggleMessageSenderIsBlocked::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleMessageSenderIsBlocked".to_string();

        RTDToggleMessageSenderIsBlockedBuilder { inner }
    }

    pub fn sender(&self) -> &MessageSender {
        &self.sender
    }

    pub fn is_blocked(&self) -> bool {
        self.is_blocked
    }
}

#[doc(hidden)]
pub struct RTDToggleMessageSenderIsBlockedBuilder {
    inner: ToggleMessageSenderIsBlocked,
}

impl RTDToggleMessageSenderIsBlockedBuilder {
    pub fn build(&self) -> ToggleMessageSenderIsBlocked {
        self.inner.clone()
    }

    pub fn sender<T: AsRef<MessageSender>>(&mut self, sender: T) -> &mut Self {
        self.inner.sender = sender.as_ref().clone();
        self
    }

    pub fn is_blocked(&mut self, is_blocked: bool) -> &mut Self {
        self.inner.is_blocked = is_blocked;
        self
    }
}

impl AsRef<ToggleMessageSenderIsBlocked> for ToggleMessageSenderIsBlocked {
    fn as_ref(&self) -> &ToggleMessageSenderIsBlocked {
        self
    }
}

impl AsRef<ToggleMessageSenderIsBlocked> for RTDToggleMessageSenderIsBlockedBuilder {
    fn as_ref(&self) -> &ToggleMessageSenderIsBlocked {
        &self.inner
    }
}
