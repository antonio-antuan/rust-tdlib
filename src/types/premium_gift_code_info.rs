use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a Telegram Premium gift code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiftCodeInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a chat or a user that created the gift code

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    creator_id: MessageSender,
    /// Point in time (Unix timestamp) when the code was created

    #[serde(default)]
    creation_date: i32,
    /// True, if the gift code was created for a giveaway

    #[serde(default)]
    is_from_giveaway: bool,
    /// Identifier of the corresponding giveaway message in the creator_id chat; can be 0 or an identifier of a deleted message

    #[serde(default)]
    giveaway_message_id: i64,
    /// Number of month the Telegram Premium subscription will be active after code activation

    #[serde(default)]
    month_count: i32,
    /// Identifier of a user for which the code was created; 0 if none

    #[serde(default)]
    user_id: i64,
    /// Point in time (Unix timestamp) when the code was activated; 0 if none

    #[serde(default)]
    use_date: i32,
}

impl RObject for PremiumGiftCodeInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PremiumGiftCodeInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumGiftCodeInfoBuilder {
        let mut inner = PremiumGiftCodeInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumGiftCodeInfoBuilder { inner }
    }

    pub fn creator_id(&self) -> &MessageSender {
        &self.creator_id
    }

    pub fn creation_date(&self) -> i32 {
        self.creation_date
    }

    pub fn is_from_giveaway(&self) -> bool {
        self.is_from_giveaway
    }

    pub fn giveaway_message_id(&self) -> i64 {
        self.giveaway_message_id
    }

    pub fn month_count(&self) -> i32 {
        self.month_count
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn use_date(&self) -> i32 {
        self.use_date
    }
}

#[doc(hidden)]
pub struct PremiumGiftCodeInfoBuilder {
    inner: PremiumGiftCodeInfo,
}

#[deprecated]
pub type RTDPremiumGiftCodeInfoBuilder = PremiumGiftCodeInfoBuilder;

impl PremiumGiftCodeInfoBuilder {
    pub fn build(&self) -> PremiumGiftCodeInfo {
        self.inner.clone()
    }

    pub fn creator_id<T: AsRef<MessageSender>>(&mut self, creator_id: T) -> &mut Self {
        self.inner.creator_id = creator_id.as_ref().clone();
        self
    }

    pub fn creation_date(&mut self, creation_date: i32) -> &mut Self {
        self.inner.creation_date = creation_date;
        self
    }

    pub fn is_from_giveaway(&mut self, is_from_giveaway: bool) -> &mut Self {
        self.inner.is_from_giveaway = is_from_giveaway;
        self
    }

    pub fn giveaway_message_id(&mut self, giveaway_message_id: i64) -> &mut Self {
        self.inner.giveaway_message_id = giveaway_message_id;
        self
    }

    pub fn month_count(&mut self, month_count: i32) -> &mut Self {
        self.inner.month_count = month_count;
        self
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn use_date(&mut self, use_date: i32) -> &mut Self {
        self.inner.use_date = use_date;
        self
    }
}

impl AsRef<PremiumGiftCodeInfo> for PremiumGiftCodeInfo {
    fn as_ref(&self) -> &PremiumGiftCodeInfo {
        self
    }
}

impl AsRef<PremiumGiftCodeInfo> for PremiumGiftCodeInfoBuilder {
    fn as_ref(&self) -> &PremiumGiftCodeInfo {
        &self.inner
    }
}
