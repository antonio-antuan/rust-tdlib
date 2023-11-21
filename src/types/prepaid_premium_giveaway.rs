use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a prepaid Telegram Premium giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrepaidPremiumGiveaway {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the prepaid giveaway

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// Number of users which will receive Telegram Premium subscription gift codes

    #[serde(default)]
    winner_count: i32,
    /// Number of month the Telegram Premium subscription will be active after code activation

    #[serde(default)]
    month_count: i32,
    /// Point in time (Unix timestamp) when the giveaway was paid

    #[serde(default)]
    payment_date: i32,
}

impl RObject for PrepaidPremiumGiveaway {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PrepaidPremiumGiveaway {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PrepaidPremiumGiveawayBuilder {
        let mut inner = PrepaidPremiumGiveaway::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PrepaidPremiumGiveawayBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn winner_count(&self) -> i32 {
        self.winner_count
    }

    pub fn month_count(&self) -> i32 {
        self.month_count
    }

    pub fn payment_date(&self) -> i32 {
        self.payment_date
    }
}

#[doc(hidden)]
pub struct PrepaidPremiumGiveawayBuilder {
    inner: PrepaidPremiumGiveaway,
}

#[deprecated]
pub type RTDPrepaidPremiumGiveawayBuilder = PrepaidPremiumGiveawayBuilder;

impl PrepaidPremiumGiveawayBuilder {
    pub fn build(&self) -> PrepaidPremiumGiveaway {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
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

    pub fn payment_date(&mut self, payment_date: i32) -> &mut Self {
        self.inner.payment_date = payment_date;
        self
    }
}

impl AsRef<PrepaidPremiumGiveaway> for PrepaidPremiumGiveaway {
    fn as_ref(&self) -> &PrepaidPremiumGiveaway {
        self
    }
}

impl AsRef<PrepaidPremiumGiveaway> for PrepaidPremiumGiveawayBuilder {
    fn as_ref(&self) -> &PrepaidPremiumGiveaway {
        &self.inner
    }
}
