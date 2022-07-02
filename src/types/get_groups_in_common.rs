use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of common group chats with a given user. Chats are sorted by their type and creation date
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGroupsInCommon {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier

    #[serde(default)]
    user_id: i64,
    /// Chat identifier starting from which to return chats; use 0 for the first request

    #[serde(default)]
    offset_chat_id: i64,
    /// The maximum number of chats to be returned; up to 100

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetGroupsInCommon {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetGroupsInCommon {}

impl GetGroupsInCommon {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetGroupsInCommonBuilder {
        let mut inner = GetGroupsInCommon::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getGroupsInCommon".to_string();

        GetGroupsInCommonBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn offset_chat_id(&self) -> i64 {
        self.offset_chat_id
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetGroupsInCommonBuilder {
    inner: GetGroupsInCommon,
}

#[deprecated]
pub type RTDGetGroupsInCommonBuilder = GetGroupsInCommonBuilder;

impl GetGroupsInCommonBuilder {
    pub fn build(&self) -> GetGroupsInCommon {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self {
        self.inner.offset_chat_id = offset_chat_id;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetGroupsInCommon> for GetGroupsInCommon {
    fn as_ref(&self) -> &GetGroupsInCommon {
        self
    }
}

impl AsRef<GetGroupsInCommon> for GetGroupsInCommonBuilder {
    fn as_ref(&self) -> &GetGroupsInCommon {
        &self.inner
    }
}
