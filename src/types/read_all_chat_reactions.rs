use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Marks all reactions in a chat as read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadAllChatReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDReadAllChatReactionsBuilder {
        let mut inner = ReadAllChatReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "readAllChatReactions".to_string();

        RTDReadAllChatReactionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDReadAllChatReactionsBuilder {
    inner: ReadAllChatReactions,
}

impl RTDReadAllChatReactionsBuilder {
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

impl AsRef<ReadAllChatReactions> for RTDReadAllChatReactionsBuilder {
    fn as_ref(&self) -> &ReadAllChatReactions {
        &self.inner
    }
}
