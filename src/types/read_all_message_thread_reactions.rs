use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Marks all reactions in a forum topic as read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadAllMessageThreadReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Message thread identifier in which reactions are marked as read

    #[serde(default)]
    message_thread_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReadAllMessageThreadReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReadAllMessageThreadReactions {}

impl ReadAllMessageThreadReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReadAllMessageThreadReactionsBuilder {
        let mut inner = ReadAllMessageThreadReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "readAllMessageThreadReactions".to_string();

        ReadAllMessageThreadReactionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }
}

#[doc(hidden)]
pub struct ReadAllMessageThreadReactionsBuilder {
    inner: ReadAllMessageThreadReactions,
}

#[deprecated]
pub type RTDReadAllMessageThreadReactionsBuilder = ReadAllMessageThreadReactionsBuilder;

impl ReadAllMessageThreadReactionsBuilder {
    pub fn build(&self) -> ReadAllMessageThreadReactions {
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

impl AsRef<ReadAllMessageThreadReactions> for ReadAllMessageThreadReactions {
    fn as_ref(&self) -> &ReadAllMessageThreadReactions {
        self
    }
}

impl AsRef<ReadAllMessageThreadReactions> for ReadAllMessageThreadReactionsBuilder {
    fn as_ref(&self) -> &ReadAllMessageThreadReactions {
        &self.inner
    }
}
