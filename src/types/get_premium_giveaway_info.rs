use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a Telegram Premium giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPremiumGiveawayInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the channel chat which started the giveaway

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the giveaway message in the chat

    #[serde(default)]
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPremiumGiveawayInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumGiveawayInfo for GetPremiumGiveawayInfo {}

impl RFunction for GetPremiumGiveawayInfo {}

impl GetPremiumGiveawayInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetPremiumGiveawayInfoBuilder {
        let mut inner = GetPremiumGiveawayInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPremiumGiveawayInfo".to_string();

        GetPremiumGiveawayInfoBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct GetPremiumGiveawayInfoBuilder {
    inner: GetPremiumGiveawayInfo,
}

#[deprecated]
pub type RTDGetPremiumGiveawayInfoBuilder = GetPremiumGiveawayInfoBuilder;

impl GetPremiumGiveawayInfoBuilder {
    pub fn build(&self) -> GetPremiumGiveawayInfo {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }
}

impl AsRef<GetPremiumGiveawayInfo> for GetPremiumGiveawayInfo {
    fn as_ref(&self) -> &GetPremiumGiveawayInfo {
        self
    }
}

impl AsRef<GetPremiumGiveawayInfo> for GetPremiumGiveawayInfoBuilder {
    fn as_ref(&self) -> &GetPremiumGiveawayInfo {
        &self.inner
    }
}
