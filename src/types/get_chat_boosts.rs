use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns list of boosts applied to a chat; requires administrator rights in the channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatBoosts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,
    /// Pass true to receive only boosts received from gift codes and giveaways created by the chat

    #[serde(default)]
    only_gift_codes: bool,
    /// Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results

    #[serde(default)]
    offset: String,
    /// The maximum number of boosts to be returned; up to 100. For optimal performance, the number of returned boosts can be smaller than the specified limit

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatBoosts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatBoosts {}

impl GetChatBoosts {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatBoostsBuilder {
        let mut inner = GetChatBoosts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatBoosts".to_string();

        GetChatBoostsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn only_gift_codes(&self) -> bool {
        self.only_gift_codes
    }

    pub fn offset(&self) -> &String {
        &self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetChatBoostsBuilder {
    inner: GetChatBoosts,
}

#[deprecated]
pub type RTDGetChatBoostsBuilder = GetChatBoostsBuilder;

impl GetChatBoostsBuilder {
    pub fn build(&self) -> GetChatBoosts {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn only_gift_codes(&mut self, only_gift_codes: bool) -> &mut Self {
        self.inner.only_gift_codes = only_gift_codes;
        self
    }

    pub fn offset<T: AsRef<str>>(&mut self, offset: T) -> &mut Self {
        self.inner.offset = offset.as_ref().to_string();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetChatBoosts> for GetChatBoosts {
    fn as_ref(&self) -> &GetChatBoosts {
        self
    }
}

impl AsRef<GetChatBoosts> for GetChatBoostsBuilder {
    fn as_ref(&self) -> &GetChatBoosts {
        &self.inner
    }
}
