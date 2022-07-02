use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Chats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total count of chats found

    #[serde(default)]
    total_count: i32,
    /// List of chat identifiers

    #[serde(default)]
    chat_ids: Vec<i64>,
}

impl RObject for Chats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Chats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatsBuilder {
        let mut inner = Chats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatsBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn chat_ids(&self) -> &Vec<i64> {
        &self.chat_ids
    }
}

#[doc(hidden)]
pub struct ChatsBuilder {
    inner: Chats,
}

#[deprecated]
pub type RTDChatsBuilder = ChatsBuilder;

impl ChatsBuilder {
    pub fn build(&self) -> Chats {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
        self.inner.chat_ids = chat_ids;
        self
    }
}

impl AsRef<Chats> for Chats {
    fn as_ref(&self) -> &Chats {
        self
    }
}

impl AsRef<Chats> for ChatsBuilder {
    fn as_ref(&self) -> &Chats {
        &self.inner
    }
}
