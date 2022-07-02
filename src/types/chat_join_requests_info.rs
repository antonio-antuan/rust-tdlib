use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about pending join requests for a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinRequestsInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Total number of pending join requests

    #[serde(default)]
    total_count: i32,
    /// Identifiers of at most 3 users sent the newest pending join requests

    #[serde(default)]
    user_ids: Vec<i64>,
}

impl RObject for ChatJoinRequestsInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatJoinRequestsInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatJoinRequestsInfoBuilder {
        let mut inner = ChatJoinRequestsInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatJoinRequestsInfoBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn user_ids(&self) -> &Vec<i64> {
        &self.user_ids
    }
}

#[doc(hidden)]
pub struct ChatJoinRequestsInfoBuilder {
    inner: ChatJoinRequestsInfo,
}

#[deprecated]
pub type RTDChatJoinRequestsInfoBuilder = ChatJoinRequestsInfoBuilder;

impl ChatJoinRequestsInfoBuilder {
    pub fn build(&self) -> ChatJoinRequestsInfo {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }
}

impl AsRef<ChatJoinRequestsInfo> for ChatJoinRequestsInfo {
    fn as_ref(&self) -> &ChatJoinRequestsInfo {
        self
    }
}

impl AsRef<ChatJoinRequestsInfo> for ChatJoinRequestsInfoBuilder {
    fn as_ref(&self) -> &ChatJoinRequestsInfo {
        &self.inner
    }
}
