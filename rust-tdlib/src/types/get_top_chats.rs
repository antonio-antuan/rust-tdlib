use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of frequently used chats. Supported only if the chat info database is enabled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTopChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Category of chats to be returned

    #[serde(skip_serializing_if = "TopChatCategory::_is_default")]
    category: TopChatCategory,
    /// The maximum number of chats to be returned; up to 30
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetTopChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetTopChats {}

impl GetTopChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetTopChatsBuilder {
        let mut inner = GetTopChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getTopChats".to_string();

        RTDGetTopChatsBuilder { inner }
    }

    pub fn category(&self) -> &TopChatCategory {
        &self.category
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDGetTopChatsBuilder {
    inner: GetTopChats,
}

impl RTDGetTopChatsBuilder {
    pub fn build(&self) -> GetTopChats {
        self.inner.clone()
    }

    pub fn category<T: AsRef<TopChatCategory>>(&mut self, category: T) -> &mut Self {
        self.inner.category = category.as_ref().clone();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetTopChats> for GetTopChats {
    fn as_ref(&self) -> &GetTopChats {
        self
    }
}

impl AsRef<GetTopChats> for RTDGetTopChatsBuilder {
    fn as_ref(&self) -> &GetTopChats {
        &self.inner
    }
}
