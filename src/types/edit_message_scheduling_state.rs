use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Edits the time when a scheduled message will be sent. Scheduling state of all messages in the same album or forwarded together with the message will be also changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditMessageSchedulingState {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat the message belongs to

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message

    #[serde(default)]
    message_id: i64,
    /// The new message scheduling state; pass null to send the message immediately

    #[serde(skip_serializing_if = "MessageSchedulingState::_is_default")]
    scheduling_state: MessageSchedulingState,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditMessageSchedulingState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditMessageSchedulingState {}

impl EditMessageSchedulingState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EditMessageSchedulingStateBuilder {
        let mut inner = EditMessageSchedulingState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editMessageSchedulingState".to_string();

        EditMessageSchedulingStateBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn scheduling_state(&self) -> &MessageSchedulingState {
        &self.scheduling_state
    }
}

#[doc(hidden)]
pub struct EditMessageSchedulingStateBuilder {
    inner: EditMessageSchedulingState,
}

#[deprecated]
pub type RTDEditMessageSchedulingStateBuilder = EditMessageSchedulingStateBuilder;

impl EditMessageSchedulingStateBuilder {
    pub fn build(&self) -> EditMessageSchedulingState {
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

    pub fn scheduling_state<T: AsRef<MessageSchedulingState>>(
        &mut self,
        scheduling_state: T,
    ) -> &mut Self {
        self.inner.scheduling_state = scheduling_state.as_ref().clone();
        self
    }
}

impl AsRef<EditMessageSchedulingState> for EditMessageSchedulingState {
    fn as_ref(&self) -> &EditMessageSchedulingState {
        self
    }
}

impl AsRef<EditMessageSchedulingState> for EditMessageSchedulingStateBuilder {
    fn as_ref(&self) -> &EditMessageSchedulingState {
        &self.inner
    }
}
