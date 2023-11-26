use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the current boost status for a channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatBoostStatus {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the channel chat

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatBoostStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatBoostStatus {}

impl GetChatBoostStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatBoostStatusBuilder {
        let mut inner = GetChatBoostStatus::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatBoostStatus".to_string();

        GetChatBoostStatusBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct GetChatBoostStatusBuilder {
    inner: GetChatBoostStatus,
}

#[deprecated]
pub type RTDGetChatBoostStatusBuilder = GetChatBoostStatusBuilder;

impl GetChatBoostStatusBuilder {
    pub fn build(&self) -> GetChatBoostStatus {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetChatBoostStatus> for GetChatBoostStatus {
    fn as_ref(&self) -> &GetChatBoostStatus {
        self
    }
}

impl AsRef<GetChatBoostStatus> for GetChatBoostStatusBuilder {
    fn as_ref(&self) -> &GetChatBoostStatus {
        &self.inner
    }
}
