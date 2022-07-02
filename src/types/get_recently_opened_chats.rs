use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns recently opened chats, this is an offline request. Returns chats in the order of last opening
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRecentlyOpenedChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The maximum number of chats to be returned

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetRecentlyOpenedChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetRecentlyOpenedChats {}

impl GetRecentlyOpenedChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetRecentlyOpenedChatsBuilder {
        let mut inner = GetRecentlyOpenedChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getRecentlyOpenedChats".to_string();

        GetRecentlyOpenedChatsBuilder { inner }
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetRecentlyOpenedChatsBuilder {
    inner: GetRecentlyOpenedChats,
}

#[deprecated]
pub type RTDGetRecentlyOpenedChatsBuilder = GetRecentlyOpenedChatsBuilder;

impl GetRecentlyOpenedChatsBuilder {
    pub fn build(&self) -> GetRecentlyOpenedChats {
        self.inner.clone()
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetRecentlyOpenedChats> for GetRecentlyOpenedChats {
    fn as_ref(&self) -> &GetRecentlyOpenedChats {
        self
    }
}

impl AsRef<GetRecentlyOpenedChats> for GetRecentlyOpenedChatsBuilder {
    fn as_ref(&self) -> &GetRecentlyOpenedChats {
        &self.inner
    }
}
