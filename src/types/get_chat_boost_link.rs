use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTPS link to boost the specified channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatBoostLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatBoostLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatBoostLink {}

impl GetChatBoostLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatBoostLinkBuilder {
        let mut inner = GetChatBoostLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatBoostLink".to_string();

        GetChatBoostLinkBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct GetChatBoostLinkBuilder {
    inner: GetChatBoostLink,
}

#[deprecated]
pub type RTDGetChatBoostLinkBuilder = GetChatBoostLinkBuilder;

impl GetChatBoostLinkBuilder {
    pub fn build(&self) -> GetChatBoostLink {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetChatBoostLink> for GetChatBoostLink {
    fn as_ref(&self) -> &GetChatBoostLink {
        self
    }
}

impl AsRef<GetChatBoostLink> for GetChatBoostLinkBuilder {
    fn as_ref(&self) -> &GetChatBoostLink {
        &self.inner
    }
}
