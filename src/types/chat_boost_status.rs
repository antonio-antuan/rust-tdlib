use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes current boost status of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostStatus {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// An HTTP URL, which can be used to boost the chat

    #[serde(default)]
    boost_url: String,
    /// Identifiers of boost slots of the current user applied to the chat

    #[serde(default)]
    applied_slot_ids: Vec<i32>,
    /// Current boost level of the chat

    #[serde(default)]
    level: i32,
    /// The number of boosts received by the chat from created Telegram Premium gift codes and giveaways; always 0 if the current user isn't an administrator in the chat

    #[serde(default)]
    gift_code_boost_count: i32,
    /// The number of boosts received by the chat

    #[serde(default)]
    boost_count: i32,
    /// The number of boosts added to reach the current level

    #[serde(default)]
    current_level_boost_count: i32,
    /// The number of boosts needed to reach the next level; 0 if the next level isn't available

    #[serde(default)]
    next_level_boost_count: i32,
    /// Approximate number of Telegram Premium subscribers joined the chat; always 0 if the current user isn't an administrator in the chat

    #[serde(default)]
    premium_member_count: i32,
    /// A percentage of Telegram Premium subscribers joined the chat; always 0 if the current user isn't an administrator in the chat

    #[serde(default)]
    premium_member_percentage: f32,
    /// The list of prepaid giveaways available for the chat; only for chat administrators

    #[serde(default)]
    prepaid_giveaways: Vec<PrepaidPremiumGiveaway>,
}

impl RObject for ChatBoostStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatBoostStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatBoostStatusBuilder {
        let mut inner = ChatBoostStatus::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatBoostStatusBuilder { inner }
    }

    pub fn boost_url(&self) -> &String {
        &self.boost_url
    }

    pub fn applied_slot_ids(&self) -> &Vec<i32> {
        &self.applied_slot_ids
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn gift_code_boost_count(&self) -> i32 {
        self.gift_code_boost_count
    }

    pub fn boost_count(&self) -> i32 {
        self.boost_count
    }

    pub fn current_level_boost_count(&self) -> i32 {
        self.current_level_boost_count
    }

    pub fn next_level_boost_count(&self) -> i32 {
        self.next_level_boost_count
    }

    pub fn premium_member_count(&self) -> i32 {
        self.premium_member_count
    }

    pub fn premium_member_percentage(&self) -> f32 {
        self.premium_member_percentage
    }

    pub fn prepaid_giveaways(&self) -> &Vec<PrepaidPremiumGiveaway> {
        &self.prepaid_giveaways
    }
}

#[doc(hidden)]
pub struct ChatBoostStatusBuilder {
    inner: ChatBoostStatus,
}

#[deprecated]
pub type RTDChatBoostStatusBuilder = ChatBoostStatusBuilder;

impl ChatBoostStatusBuilder {
    pub fn build(&self) -> ChatBoostStatus {
        self.inner.clone()
    }

    pub fn boost_url<T: AsRef<str>>(&mut self, boost_url: T) -> &mut Self {
        self.inner.boost_url = boost_url.as_ref().to_string();
        self
    }

    pub fn applied_slot_ids(&mut self, applied_slot_ids: Vec<i32>) -> &mut Self {
        self.inner.applied_slot_ids = applied_slot_ids;
        self
    }

    pub fn level(&mut self, level: i32) -> &mut Self {
        self.inner.level = level;
        self
    }

    pub fn gift_code_boost_count(&mut self, gift_code_boost_count: i32) -> &mut Self {
        self.inner.gift_code_boost_count = gift_code_boost_count;
        self
    }

    pub fn boost_count(&mut self, boost_count: i32) -> &mut Self {
        self.inner.boost_count = boost_count;
        self
    }

    pub fn current_level_boost_count(&mut self, current_level_boost_count: i32) -> &mut Self {
        self.inner.current_level_boost_count = current_level_boost_count;
        self
    }

    pub fn next_level_boost_count(&mut self, next_level_boost_count: i32) -> &mut Self {
        self.inner.next_level_boost_count = next_level_boost_count;
        self
    }

    pub fn premium_member_count(&mut self, premium_member_count: i32) -> &mut Self {
        self.inner.premium_member_count = premium_member_count;
        self
    }

    pub fn premium_member_percentage(&mut self, premium_member_percentage: f32) -> &mut Self {
        self.inner.premium_member_percentage = premium_member_percentage;
        self
    }

    pub fn prepaid_giveaways(
        &mut self,
        prepaid_giveaways: Vec<PrepaidPremiumGiveaway>,
    ) -> &mut Self {
        self.inner.prepaid_giveaways = prepaid_giveaways;
        self
    }
}

impl AsRef<ChatBoostStatus> for ChatBoostStatus {
    fn as_ref(&self) -> &ChatBoostStatus {
        self
    }
}

impl AsRef<ChatBoostStatus> for ChatBoostStatusBuilder {
    fn as_ref(&self) -> &ChatBoostStatus {
        &self.inner
    }
}
