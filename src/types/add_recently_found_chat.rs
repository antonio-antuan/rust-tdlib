use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Adds a chat to the list of recently found chats. The chat is added to the beginning of the list. If the chat is already in the list, it will be removed from the list first
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddRecentlyFoundChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to add
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddRecentlyFoundChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddRecentlyFoundChat {}

impl AddRecentlyFoundChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAddRecentlyFoundChatBuilder {
        let mut inner = AddRecentlyFoundChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addRecentlyFoundChat".to_string();

        RTDAddRecentlyFoundChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDAddRecentlyFoundChatBuilder {
    inner: AddRecentlyFoundChat,
}

impl RTDAddRecentlyFoundChatBuilder {
    pub fn build(&self) -> AddRecentlyFoundChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<AddRecentlyFoundChat> for AddRecentlyFoundChat {
    fn as_ref(&self) -> &AddRecentlyFoundChat {
        self
    }
}

impl AsRef<AddRecentlyFoundChat> for RTDAddRecentlyFoundChatBuilder {
    fn as_ref(&self) -> &AddRecentlyFoundChat {
        &self.inner
    }
}
