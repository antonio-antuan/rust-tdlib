use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the draft message in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatDraftMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// If not 0, a message thread identifier in which the draft was changed
    message_thread_id: i64,
    /// New draft message; may be null
    draft_message: Option<DraftMessage>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatDraftMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatDraftMessage {}

impl SetChatDraftMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetChatDraftMessageBuilder {
        let mut inner = SetChatDraftMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatDraftMessage".to_string();

        RTDSetChatDraftMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn draft_message(&self) -> &Option<DraftMessage> {
        &self.draft_message
    }
}

#[doc(hidden)]
pub struct RTDSetChatDraftMessageBuilder {
    inner: SetChatDraftMessage,
}

impl RTDSetChatDraftMessageBuilder {
    pub fn build(&self) -> SetChatDraftMessage {
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

    pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
        self.inner.draft_message = Some(draft_message.as_ref().clone());
        self
    }
}

impl AsRef<SetChatDraftMessage> for SetChatDraftMessage {
    fn as_ref(&self) -> &SetChatDraftMessage {
        self
    }
}

impl AsRef<SetChatDraftMessage> for RTDSetChatDraftMessageBuilder {
    fn as_ref(&self) -> &SetChatDraftMessage {
        &self.inner
    }
}
