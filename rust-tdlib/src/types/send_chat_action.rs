use crate::errors::*;
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
    chat_id: i64,
    /// If not 0, a message thread identifier in which the action was performed
    message_thread_id: i64,
    /// The action description

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendChatActionBuilder {
        let mut inner = SendChatAction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendChatAction".to_string();

        RTDSendChatActionBuilder { inner }
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
pub struct RTDSendChatActionBuilder {
    inner: SendChatAction,
}

impl RTDSendChatActionBuilder {
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

impl AsRef<SendChatAction> for RTDSendChatActionBuilder {
    fn as_ref(&self) -> &SendChatAction {
        &self.inner
    }
}
