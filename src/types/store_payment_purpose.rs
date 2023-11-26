use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a purpose of an in-store payment
pub trait TDStorePaymentPurpose: Debug + RObject {}

/// Describes a purpose of an in-store payment
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum StorePaymentPurpose {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The user gifting Telegram Premium to another user
    #[serde(rename = "storePaymentPurposeGiftedPremium")]
    GiftedPremium(StorePaymentPurposeGiftedPremium),
    /// The user creating Telegram Premium gift codes for other users
    #[serde(rename = "storePaymentPurposePremiumGiftCodes")]
    PremiumGiftCodes(StorePaymentPurposePremiumGiftCodes),
    /// The user creating a Telegram Premium giveaway for subscribers of channel chats; requires can_post_messages rights in the channels
    #[serde(rename = "storePaymentPurposePremiumGiveaway")]
    PremiumGiveaway(StorePaymentPurposePremiumGiveaway),
    /// The user subscribing to Telegram Premium
    #[serde(rename = "storePaymentPurposePremiumSubscription")]
    PremiumSubscription(StorePaymentPurposePremiumSubscription),
}

impl RObject for StorePaymentPurpose {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            StorePaymentPurpose::GiftedPremium(t) => t.extra(),
            StorePaymentPurpose::PremiumGiftCodes(t) => t.extra(),
            StorePaymentPurpose::PremiumGiveaway(t) => t.extra(),
            StorePaymentPurpose::PremiumSubscription(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            StorePaymentPurpose::GiftedPremium(t) => t.client_id(),
            StorePaymentPurpose::PremiumGiftCodes(t) => t.client_id(),
            StorePaymentPurpose::PremiumGiveaway(t) => t.client_id(),
            StorePaymentPurpose::PremiumSubscription(t) => t.client_id(),

            _ => None,
        }
    }
}

impl StorePaymentPurpose {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, StorePaymentPurpose::_Default)
    }
}

impl AsRef<StorePaymentPurpose> for StorePaymentPurpose {
    fn as_ref(&self) -> &StorePaymentPurpose {
        self
    }
}

/// The user gifting Telegram Premium to another user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorePaymentPurposeGiftedPremium {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the user to which Premium was gifted

    #[serde(default)]
    user_id: i64,
    /// ISO 4217 currency code of the payment currency

    #[serde(default)]
    currency: String,
    /// Paid amount, in the smallest units of the currency

    #[serde(default)]
    amount: i64,
}

impl RObject for StorePaymentPurposeGiftedPremium {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStorePaymentPurpose for StorePaymentPurposeGiftedPremium {}

impl StorePaymentPurposeGiftedPremium {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StorePaymentPurposeGiftedPremiumBuilder {
        let mut inner = StorePaymentPurposeGiftedPremium::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StorePaymentPurposeGiftedPremiumBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn currency(&self) -> &String {
        &self.currency
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }
}

#[doc(hidden)]
pub struct StorePaymentPurposeGiftedPremiumBuilder {
    inner: StorePaymentPurposeGiftedPremium,
}

#[deprecated]
pub type RTDStorePaymentPurposeGiftedPremiumBuilder = StorePaymentPurposeGiftedPremiumBuilder;

impl StorePaymentPurposeGiftedPremiumBuilder {
    pub fn build(&self) -> StorePaymentPurposeGiftedPremium {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
        self.inner.currency = currency.as_ref().to_string();
        self
    }

    pub fn amount(&mut self, amount: i64) -> &mut Self {
        self.inner.amount = amount;
        self
    }
}

impl AsRef<StorePaymentPurposeGiftedPremium> for StorePaymentPurposeGiftedPremium {
    fn as_ref(&self) -> &StorePaymentPurposeGiftedPremium {
        self
    }
}

impl AsRef<StorePaymentPurposeGiftedPremium> for StorePaymentPurposeGiftedPremiumBuilder {
    fn as_ref(&self) -> &StorePaymentPurposeGiftedPremium {
        &self.inner
    }
}

/// The user creating Telegram Premium gift codes for other users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorePaymentPurposePremiumGiftCodes {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the channel chat, which will be automatically boosted by the users for duration of the Premium subscription and which is administered by the user; 0 if none

    #[serde(default)]
    boosted_chat_id: i64,
    /// ISO 4217 currency code of the payment currency

    #[serde(default)]
    currency: String,
    /// Paid amount, in the smallest units of the currency

    #[serde(default)]
    amount: i64,
    /// Identifiers of the users which can activate the gift codes

    #[serde(default)]
    user_ids: Vec<i64>,
}

impl RObject for StorePaymentPurposePremiumGiftCodes {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStorePaymentPurpose for StorePaymentPurposePremiumGiftCodes {}

impl StorePaymentPurposePremiumGiftCodes {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StorePaymentPurposePremiumGiftCodesBuilder {
        let mut inner = StorePaymentPurposePremiumGiftCodes::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StorePaymentPurposePremiumGiftCodesBuilder { inner }
    }

    pub fn boosted_chat_id(&self) -> i64 {
        self.boosted_chat_id
    }

    pub fn currency(&self) -> &String {
        &self.currency
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }

    pub fn user_ids(&self) -> &Vec<i64> {
        &self.user_ids
    }
}

#[doc(hidden)]
pub struct StorePaymentPurposePremiumGiftCodesBuilder {
    inner: StorePaymentPurposePremiumGiftCodes,
}

#[deprecated]
pub type RTDStorePaymentPurposePremiumGiftCodesBuilder = StorePaymentPurposePremiumGiftCodesBuilder;

impl StorePaymentPurposePremiumGiftCodesBuilder {
    pub fn build(&self) -> StorePaymentPurposePremiumGiftCodes {
        self.inner.clone()
    }

    pub fn boosted_chat_id(&mut self, boosted_chat_id: i64) -> &mut Self {
        self.inner.boosted_chat_id = boosted_chat_id;
        self
    }

    pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
        self.inner.currency = currency.as_ref().to_string();
        self
    }

    pub fn amount(&mut self, amount: i64) -> &mut Self {
        self.inner.amount = amount;
        self
    }

    pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }
}

impl AsRef<StorePaymentPurposePremiumGiftCodes> for StorePaymentPurposePremiumGiftCodes {
    fn as_ref(&self) -> &StorePaymentPurposePremiumGiftCodes {
        self
    }
}

impl AsRef<StorePaymentPurposePremiumGiftCodes> for StorePaymentPurposePremiumGiftCodesBuilder {
    fn as_ref(&self) -> &StorePaymentPurposePremiumGiftCodes {
        &self.inner
    }
}

/// The user creating a Telegram Premium giveaway for subscribers of channel chats; requires can_post_messages rights in the channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorePaymentPurposePremiumGiveaway {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Giveaway parameters
    parameters: PremiumGiveawayParameters,
    /// ISO 4217 currency code of the payment currency

    #[serde(default)]
    currency: String,
    /// Paid amount, in the smallest units of the currency

    #[serde(default)]
    amount: i64,
}

impl RObject for StorePaymentPurposePremiumGiveaway {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStorePaymentPurpose for StorePaymentPurposePremiumGiveaway {}

impl StorePaymentPurposePremiumGiveaway {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StorePaymentPurposePremiumGiveawayBuilder {
        let mut inner = StorePaymentPurposePremiumGiveaway::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StorePaymentPurposePremiumGiveawayBuilder { inner }
    }

    pub fn parameters(&self) -> &PremiumGiveawayParameters {
        &self.parameters
    }

    pub fn currency(&self) -> &String {
        &self.currency
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }
}

#[doc(hidden)]
pub struct StorePaymentPurposePremiumGiveawayBuilder {
    inner: StorePaymentPurposePremiumGiveaway,
}

#[deprecated]
pub type RTDStorePaymentPurposePremiumGiveawayBuilder = StorePaymentPurposePremiumGiveawayBuilder;

impl StorePaymentPurposePremiumGiveawayBuilder {
    pub fn build(&self) -> StorePaymentPurposePremiumGiveaway {
        self.inner.clone()
    }

    pub fn parameters<T: AsRef<PremiumGiveawayParameters>>(&mut self, parameters: T) -> &mut Self {
        self.inner.parameters = parameters.as_ref().clone();
        self
    }

    pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
        self.inner.currency = currency.as_ref().to_string();
        self
    }

    pub fn amount(&mut self, amount: i64) -> &mut Self {
        self.inner.amount = amount;
        self
    }
}

impl AsRef<StorePaymentPurposePremiumGiveaway> for StorePaymentPurposePremiumGiveaway {
    fn as_ref(&self) -> &StorePaymentPurposePremiumGiveaway {
        self
    }
}

impl AsRef<StorePaymentPurposePremiumGiveaway> for StorePaymentPurposePremiumGiveawayBuilder {
    fn as_ref(&self) -> &StorePaymentPurposePremiumGiveaway {
        &self.inner
    }
}

/// The user subscribing to Telegram Premium
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorePaymentPurposePremiumSubscription {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true if this is a restore of a Telegram Premium purchase; only for App Store

    #[serde(default)]
    is_restore: bool,
    /// Pass true if this is an upgrade from a monthly subscription to early subscription; only for App Store

    #[serde(default)]
    is_upgrade: bool,
}

impl RObject for StorePaymentPurposePremiumSubscription {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStorePaymentPurpose for StorePaymentPurposePremiumSubscription {}

impl StorePaymentPurposePremiumSubscription {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StorePaymentPurposePremiumSubscriptionBuilder {
        let mut inner = StorePaymentPurposePremiumSubscription::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StorePaymentPurposePremiumSubscriptionBuilder { inner }
    }

    pub fn is_restore(&self) -> bool {
        self.is_restore
    }

    pub fn is_upgrade(&self) -> bool {
        self.is_upgrade
    }
}

#[doc(hidden)]
pub struct StorePaymentPurposePremiumSubscriptionBuilder {
    inner: StorePaymentPurposePremiumSubscription,
}

#[deprecated]
pub type RTDStorePaymentPurposePremiumSubscriptionBuilder =
    StorePaymentPurposePremiumSubscriptionBuilder;

impl StorePaymentPurposePremiumSubscriptionBuilder {
    pub fn build(&self) -> StorePaymentPurposePremiumSubscription {
        self.inner.clone()
    }

    pub fn is_restore(&mut self, is_restore: bool) -> &mut Self {
        self.inner.is_restore = is_restore;
        self
    }

    pub fn is_upgrade(&mut self, is_upgrade: bool) -> &mut Self {
        self.inner.is_upgrade = is_upgrade;
        self
    }
}

impl AsRef<StorePaymentPurposePremiumSubscription> for StorePaymentPurposePremiumSubscription {
    fn as_ref(&self) -> &StorePaymentPurposePremiumSubscription {
        self
    }
}

impl AsRef<StorePaymentPurposePremiumSubscription>
    for StorePaymentPurposePremiumSubscriptionBuilder
{
    fn as_ref(&self) -> &StorePaymentPurposePremiumSubscription {
        &self.inner
    }
}
