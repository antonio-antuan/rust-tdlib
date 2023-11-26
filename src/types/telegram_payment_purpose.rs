use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a purpose of a payment toward Telegram
pub trait TDTelegramPaymentPurpose: Debug + RObject {}

/// Describes a purpose of a payment toward Telegram
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum TelegramPaymentPurpose {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The user creating Telegram Premium gift codes for other users
    #[serde(rename = "telegramPaymentPurposePremiumGiftCodes")]
    PremiumGiftCodes(TelegramPaymentPurposePremiumGiftCodes),
    /// The user creating a Telegram Premium giveaway for subscribers of channel chats; requires can_post_messages rights in the channels
    #[serde(rename = "telegramPaymentPurposePremiumGiveaway")]
    PremiumGiveaway(TelegramPaymentPurposePremiumGiveaway),
}

impl RObject for TelegramPaymentPurpose {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            TelegramPaymentPurpose::PremiumGiftCodes(t) => t.extra(),
            TelegramPaymentPurpose::PremiumGiveaway(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            TelegramPaymentPurpose::PremiumGiftCodes(t) => t.client_id(),
            TelegramPaymentPurpose::PremiumGiveaway(t) => t.client_id(),

            _ => None,
        }
    }
}

impl TelegramPaymentPurpose {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, TelegramPaymentPurpose::_Default)
    }
}

impl AsRef<TelegramPaymentPurpose> for TelegramPaymentPurpose {
    fn as_ref(&self) -> &TelegramPaymentPurpose {
        self
    }
}

/// The user creating Telegram Premium gift codes for other users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelegramPaymentPurposePremiumGiftCodes {
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
    /// Number of month the Telegram Premium subscription will be active for the users

    #[serde(default)]
    month_count: i32,
}

impl RObject for TelegramPaymentPurposePremiumGiftCodes {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTelegramPaymentPurpose for TelegramPaymentPurposePremiumGiftCodes {}

impl TelegramPaymentPurposePremiumGiftCodes {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TelegramPaymentPurposePremiumGiftCodesBuilder {
        let mut inner = TelegramPaymentPurposePremiumGiftCodes::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TelegramPaymentPurposePremiumGiftCodesBuilder { inner }
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

    pub fn month_count(&self) -> i32 {
        self.month_count
    }
}

#[doc(hidden)]
pub struct TelegramPaymentPurposePremiumGiftCodesBuilder {
    inner: TelegramPaymentPurposePremiumGiftCodes,
}

#[deprecated]
pub type RTDTelegramPaymentPurposePremiumGiftCodesBuilder =
    TelegramPaymentPurposePremiumGiftCodesBuilder;

impl TelegramPaymentPurposePremiumGiftCodesBuilder {
    pub fn build(&self) -> TelegramPaymentPurposePremiumGiftCodes {
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

    pub fn month_count(&mut self, month_count: i32) -> &mut Self {
        self.inner.month_count = month_count;
        self
    }
}

impl AsRef<TelegramPaymentPurposePremiumGiftCodes> for TelegramPaymentPurposePremiumGiftCodes {
    fn as_ref(&self) -> &TelegramPaymentPurposePremiumGiftCodes {
        self
    }
}

impl AsRef<TelegramPaymentPurposePremiumGiftCodes>
    for TelegramPaymentPurposePremiumGiftCodesBuilder
{
    fn as_ref(&self) -> &TelegramPaymentPurposePremiumGiftCodes {
        &self.inner
    }
}

/// The user creating a Telegram Premium giveaway for subscribers of channel chats; requires can_post_messages rights in the channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelegramPaymentPurposePremiumGiveaway {
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
    /// Number of users which will be able to activate the gift codes

    #[serde(default)]
    winner_count: i32,
    /// Number of month the Telegram Premium subscription will be active for the users

    #[serde(default)]
    month_count: i32,
}

impl RObject for TelegramPaymentPurposePremiumGiveaway {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTelegramPaymentPurpose for TelegramPaymentPurposePremiumGiveaway {}

impl TelegramPaymentPurposePremiumGiveaway {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TelegramPaymentPurposePremiumGiveawayBuilder {
        let mut inner = TelegramPaymentPurposePremiumGiveaway::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TelegramPaymentPurposePremiumGiveawayBuilder { inner }
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

    pub fn winner_count(&self) -> i32 {
        self.winner_count
    }

    pub fn month_count(&self) -> i32 {
        self.month_count
    }
}

#[doc(hidden)]
pub struct TelegramPaymentPurposePremiumGiveawayBuilder {
    inner: TelegramPaymentPurposePremiumGiveaway,
}

#[deprecated]
pub type RTDTelegramPaymentPurposePremiumGiveawayBuilder =
    TelegramPaymentPurposePremiumGiveawayBuilder;

impl TelegramPaymentPurposePremiumGiveawayBuilder {
    pub fn build(&self) -> TelegramPaymentPurposePremiumGiveaway {
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

    pub fn winner_count(&mut self, winner_count: i32) -> &mut Self {
        self.inner.winner_count = winner_count;
        self
    }

    pub fn month_count(&mut self, month_count: i32) -> &mut Self {
        self.inner.month_count = month_count;
        self
    }
}

impl AsRef<TelegramPaymentPurposePremiumGiveaway> for TelegramPaymentPurposePremiumGiveaway {
    fn as_ref(&self) -> &TelegramPaymentPurposePremiumGiveaway {
        self
    }
}

impl AsRef<TelegramPaymentPurposePremiumGiveaway> for TelegramPaymentPurposePremiumGiveawayBuilder {
    fn as_ref(&self) -> &TelegramPaymentPurposePremiumGiveaway {
        &self.inner
    }
}
