use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sends a notification about user activity in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendChatAction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// If not 0, a message thread identifier in which the action was performed

    #[serde(default)]
    message_thread_id: i64,
    /// The action description; pass null to cancel the currently active action

    #[serde(skip_serializing_if = "ChatAction::_is_default")]
    action: ChatAction,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendChatAction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendChatAction {}

impl SendChatAction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SendChatActionBuilder {
        let mut inner = SendChatAction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendChatAction".to_string();

        SendChatActionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn action(&self) -> &ChatAction {
        &self.action
    }
}

#[doc(hidden)]
pub struct SendChatActionBuilder {
    inner: SendChatAction,
}

#[deprecated]
pub type RTDSendChatActionBuilder = SendChatActionBuilder;

impl SendChatActionBuilder {
    pub fn build(&self) -> SendChatAction {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }

    pub fn action<T: AsRef<ChatAction>>(&mut self, action: T) -> &mut Self {
        self.inner.action = action.as_ref().clone();
        self
    }
}

impl AsRef<SendChatAction> for SendChatAction {
    fn as_ref(&self) -> &SendChatAction {
        self
    }
}

impl AsRef<SendChatAction> for SendChatActionBuilder {
    fn as_ref(&self) -> &SendChatAction {
        &self.inner
    }
}
