use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns list of boosts applied to a chat by a given user; requires administrator rights in the channel chat; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserChatBoosts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the user

    #[serde(default)]
    user_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetUserChatBoosts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetUserChatBoosts {}

impl GetUserChatBoosts {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetUserChatBoostsBuilder {
        let mut inner = GetUserChatBoosts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getUserChatBoosts".to_string();

        GetUserChatBoostsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct GetUserChatBoostsBuilder {
    inner: GetUserChatBoosts,
}

#[deprecated]
pub type RTDGetUserChatBoostsBuilder = GetUserChatBoostsBuilder;

impl GetUserChatBoostsBuilder {
    pub fn build(&self) -> GetUserChatBoosts {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<GetUserChatBoosts> for GetUserChatBoosts {
    fn as_ref(&self) -> &GetUserChatBoosts {
        self
    }
}

impl AsRef<GetUserChatBoosts> for GetUserChatBoostsBuilder {
    fn as_ref(&self) -> &GetUserChatBoosts {
        &self.inner
    }
}
