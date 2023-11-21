use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Marks all mentions in a forum topic as read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadAllMessageThreadMentions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Message thread identifier in which mentions are marked as read

    #[serde(default)]
    message_thread_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReadAllMessageThreadMentions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReadAllMessageThreadMentions {}

impl ReadAllMessageThreadMentions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReadAllMessageThreadMentionsBuilder {
        let mut inner = ReadAllMessageThreadMentions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "readAllMessageThreadMentions".to_string();

        ReadAllMessageThreadMentionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }
}

#[doc(hidden)]
pub struct ReadAllMessageThreadMentionsBuilder {
    inner: ReadAllMessageThreadMentions,
}

#[deprecated]
pub type RTDReadAllMessageThreadMentionsBuilder = ReadAllMessageThreadMentionsBuilder;

impl ReadAllMessageThreadMentionsBuilder {
    pub fn build(&self) -> ReadAllMessageThreadMentions {
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

impl AsRef<ReadAllMessageThreadMentions> for ReadAllMessageThreadMentions {
    fn as_ref(&self) -> &ReadAllMessageThreadMentions {
        self
    }
}

impl AsRef<ReadAllMessageThreadMentions> for ReadAllMessageThreadMentionsBuilder {
    fn as_ref(&self) -> &ReadAllMessageThreadMentions {
        &self.inner
    }
}
