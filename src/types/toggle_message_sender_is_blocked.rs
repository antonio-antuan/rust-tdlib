use crate::errors::Result;
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
    /// Identifier of a message sender to block/unblock

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender_id: MessageSender,
    /// New value of is_blocked

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleMessageSenderIsBlockedBuilder {
        let mut inner = ToggleMessageSenderIsBlocked::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleMessageSenderIsBlocked".to_string();

        ToggleMessageSenderIsBlockedBuilder { inner }
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }

    pub fn is_blocked(&self) -> bool {
        self.is_blocked
    }
}

#[doc(hidden)]
pub struct ToggleMessageSenderIsBlockedBuilder {
    inner: ToggleMessageSenderIsBlocked,
}

#[deprecated]
pub type RTDToggleMessageSenderIsBlockedBuilder = ToggleMessageSenderIsBlockedBuilder;

impl ToggleMessageSenderIsBlockedBuilder {
    pub fn build(&self) -> ToggleMessageSenderIsBlocked {
        self.inner.clone()
    }

    pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
        self.inner.sender_id = sender_id.as_ref().clone();
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

impl AsRef<ToggleMessageSenderIsBlocked> for ToggleMessageSenderIsBlockedBuilder {
    fn as_ref(&self) -> &ToggleMessageSenderIsBlocked {
        &self.inner
    }
}
