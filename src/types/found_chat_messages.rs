use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of messages found by a search in a given chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundChatMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total number of messages found; 1 if unknown

    #[serde(default)]
    total_count: i32,
    /// List of messages

    #[serde(default)]
    messages: Vec<Message>,
    /// The offset for the next request. If 0, there are no more results

    #[serde(default)]
    next_from_message_id: i64,
}

impl RObject for FoundChatMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl FoundChatMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FoundChatMessagesBuilder {
        let mut inner = FoundChatMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FoundChatMessagesBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn next_from_message_id(&self) -> i64 {
        self.next_from_message_id
    }
}

#[doc(hidden)]
pub struct FoundChatMessagesBuilder {
    inner: FoundChatMessages,
}

#[deprecated]
pub type RTDFoundChatMessagesBuilder = FoundChatMessagesBuilder;

impl FoundChatMessagesBuilder {
    pub fn build(&self) -> FoundChatMessages {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn messages(&mut self, messages: Vec<Message>) -> &mut Self {
        self.inner.messages = messages;
        self
    }

    pub fn next_from_message_id(&mut self, next_from_message_id: i64) -> &mut Self {
        self.inner.next_from_message_id = next_from_message_id;
        self
    }
}

impl AsRef<FoundChatMessages> for FoundChatMessages {
    fn as_ref(&self) -> &FoundChatMessages {
        self
    }
}

impl AsRef<FoundChatMessages> for FoundChatMessagesBuilder {
    fn as_ref(&self) -> &FoundChatMessages {
        &self.inner
    }
}
