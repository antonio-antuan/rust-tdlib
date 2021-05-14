use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns users and chats that were blocked by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBlockedMessageSenders {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number of users and chats to skip in the result; must be non-negative
    offset: i32,
    /// The maximum number of users and chats to return; up to 100
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetBlockedMessageSenders {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetBlockedMessageSenders {}

impl GetBlockedMessageSenders {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetBlockedMessageSendersBuilder {
        let mut inner = GetBlockedMessageSenders::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getBlockedMessageSenders".to_string();

        RTDGetBlockedMessageSendersBuilder { inner }
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDGetBlockedMessageSendersBuilder {
    inner: GetBlockedMessageSenders,
}

impl RTDGetBlockedMessageSendersBuilder {
    pub fn build(&self) -> GetBlockedMessageSenders {
        self.inner.clone()
    }

    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.inner.offset = offset;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetBlockedMessageSenders> for GetBlockedMessageSenders {
    fn as_ref(&self) -> &GetBlockedMessageSenders {
        self
    }
}

impl AsRef<GetBlockedMessageSenders> for RTDGetBlockedMessageSendersBuilder {
    fn as_ref(&self) -> &GetBlockedMessageSenders {
        &self.inner
    }
}
