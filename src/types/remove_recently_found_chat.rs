use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Removes a chat from the list of recently found chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveRecentlyFoundChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to be removed

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveRecentlyFoundChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveRecentlyFoundChat {}

impl RemoveRecentlyFoundChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RemoveRecentlyFoundChatBuilder {
        let mut inner = RemoveRecentlyFoundChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeRecentlyFoundChat".to_string();

        RemoveRecentlyFoundChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RemoveRecentlyFoundChatBuilder {
    inner: RemoveRecentlyFoundChat,
}

#[deprecated]
pub type RTDRemoveRecentlyFoundChatBuilder = RemoveRecentlyFoundChatBuilder;

impl RemoveRecentlyFoundChatBuilder {
    pub fn build(&self) -> RemoveRecentlyFoundChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<RemoveRecentlyFoundChat> for RemoveRecentlyFoundChat {
    fn as_ref(&self) -> &RemoveRecentlyFoundChat {
        self
    }
}

impl AsRef<RemoveRecentlyFoundChat> for RemoveRecentlyFoundChatBuilder {
    fn as_ref(&self) -> &RemoveRecentlyFoundChat {
        &self.inner
    }
}
