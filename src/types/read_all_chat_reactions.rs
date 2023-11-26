use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Marks all reactions in a chat or a forum topic as read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadAllChatReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReadAllChatReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReadAllChatReactions {}

impl ReadAllChatReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReadAllChatReactionsBuilder {
        let mut inner = ReadAllChatReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "readAllChatReactions".to_string();

        ReadAllChatReactionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct ReadAllChatReactionsBuilder {
    inner: ReadAllChatReactions,
}

#[deprecated]
pub type RTDReadAllChatReactionsBuilder = ReadAllChatReactionsBuilder;

impl ReadAllChatReactionsBuilder {
    pub fn build(&self) -> ReadAllChatReactions {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<ReadAllChatReactions> for ReadAllChatReactions {
    fn as_ref(&self) -> &ReadAllChatReactions {
        self
    }
}

impl AsRef<ReadAllChatReactions> for ReadAllChatReactionsBuilder {
    fn as_ref(&self) -> &ReadAllChatReactions {
        &self.inner
    }
}
