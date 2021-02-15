use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains statistics about number of new members invited by a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsInviterInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier
    user_id: i32,
    /// Number of new members invited by the user
    added_member_count: i32,
}

impl RObject for ChatStatisticsInviterInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatStatisticsInviterInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatStatisticsInviterInfoBuilder {
        let mut inner = ChatStatisticsInviterInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatStatisticsInviterInfoBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn added_member_count(&self) -> i32 {
        self.added_member_count
    }
}

#[doc(hidden)]
pub struct RTDChatStatisticsInviterInfoBuilder {
    inner: ChatStatisticsInviterInfo,
}

impl RTDChatStatisticsInviterInfoBuilder {
    pub fn build(&self) -> ChatStatisticsInviterInfo {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn added_member_count(&mut self, added_member_count: i32) -> &mut Self {
        self.inner.added_member_count = added_member_count;
        self
    }
}

impl AsRef<ChatStatisticsInviterInfo> for ChatStatisticsInviterInfo {
    fn as_ref(&self) -> &ChatStatisticsInviterInfo {
        self
    }
}

impl AsRef<ChatStatisticsInviterInfo> for RTDChatStatisticsInviterInfoBuilder {
    fn as_ref(&self) -> &ChatStatisticsInviterInfo {
        &self.inner
    }
}
