use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Deletes all messages in a forum topic; requires can_delete_messages administrator right in the supergroup unless the user is creator of the topic, the topic has no messages from other users and has at most 11 messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteForumTopic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,
    /// Message thread identifier of the forum topic

    #[serde(default)]
    message_thread_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteForumTopic {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteForumTopic {}

impl DeleteForumTopic {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeleteForumTopicBuilder {
        let mut inner = DeleteForumTopic::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteForumTopic".to_string();

        DeleteForumTopicBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }
}

#[doc(hidden)]
pub struct DeleteForumTopicBuilder {
    inner: DeleteForumTopic,
}

#[deprecated]
pub type RTDDeleteForumTopicBuilder = DeleteForumTopicBuilder;

impl DeleteForumTopicBuilder {
    pub fn build(&self) -> DeleteForumTopic {
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
}

impl AsRef<DeleteForumTopic> for DeleteForumTopic {
    fn as_ref(&self) -> &DeleteForumTopic {
        self
    }
}

impl AsRef<DeleteForumTopic> for DeleteForumTopicBuilder {
    fn as_ref(&self) -> &DeleteForumTopic {
        &self.inner
    }
}
