use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Marks all mentions in a chat as read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadAllChatMentions {
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

impl RObject for ReadAllChatMentions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReadAllChatMentions {}

impl ReadAllChatMentions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReadAllChatMentionsBuilder {
        let mut inner = ReadAllChatMentions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "readAllChatMentions".to_string();

        ReadAllChatMentionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct ReadAllChatMentionsBuilder {
    inner: ReadAllChatMentions,
}

#[deprecated]
pub type RTDReadAllChatMentionsBuilder = ReadAllChatMentionsBuilder;

impl ReadAllChatMentionsBuilder {
    pub fn build(&self) -> ReadAllChatMentions {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<ReadAllChatMentions> for ReadAllChatMentions {
    fn as_ref(&self) -> &ReadAllChatMentions {
        self
    }
}

impl AsRef<ReadAllChatMentions> for ReadAllChatMentionsBuilder {
    fn as_ref(&self) -> &ReadAllChatMentions {
        &self.inner
    }
}
