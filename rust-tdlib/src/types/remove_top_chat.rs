use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Removes a chat from the list of frequently used chats. Supported only if the chat info database is enabled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveTopChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Category of frequently used chats

    #[serde(skip_serializing_if = "TopChatCategory::_is_default")]
    category: TopChatCategory,
    /// Chat identifier
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveTopChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveTopChat {}

impl RemoveTopChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRemoveTopChatBuilder {
        let mut inner = RemoveTopChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeTopChat".to_string();

        RTDRemoveTopChatBuilder { inner }
    }

    pub fn category(&self) -> &TopChatCategory {
        &self.category
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDRemoveTopChatBuilder {
    inner: RemoveTopChat,
}

impl RTDRemoveTopChatBuilder {
    pub fn build(&self) -> RemoveTopChat {
        self.inner.clone()
    }

    pub fn category<T: AsRef<TopChatCategory>>(&mut self, category: T) -> &mut Self {
        self.inner.category = category.as_ref().clone();
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<RemoveTopChat> for RemoveTopChat {
    fn as_ref(&self) -> &RemoveTopChat {
        self
    }
}

impl AsRef<RemoveTopChat> for RTDRemoveTopChatBuilder {
    fn as_ref(&self) -> &RemoveTopChat {
        &self.inner
    }
}
