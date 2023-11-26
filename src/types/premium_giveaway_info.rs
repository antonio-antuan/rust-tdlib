use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about Telegram Premium giveaway
pub trait TDPremiumGiveawayInfo: Debug + RObject {}

/// Contains information about Telegram Premium giveaway
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum PremiumGiveawayInfo {
    #[doc(hidden)]
    #[default]
    _Default,
    /// Returns information about a Telegram Premium giveaway
    #[serde(rename = "getPremiumGiveawayInfo")]
    GetPremiumGiveawayInfo(GetPremiumGiveawayInfo),
    /// Describes a completed giveaway
    #[serde(rename = "premiumGiveawayInfoCompleted")]
    Completed(PremiumGiveawayInfoCompleted),
    /// Describes an ongoing giveaway
    #[serde(rename = "premiumGiveawayInfoOngoing")]
    Ongoing(PremiumGiveawayInfoOngoing),
}

impl RObject for PremiumGiveawayInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PremiumGiveawayInfo::GetPremiumGiveawayInfo(t) => t.extra(),
            PremiumGiveawayInfo::Completed(t) => t.extra(),
            PremiumGiveawayInfo::Ongoing(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PremiumGiveawayInfo::GetPremiumGiveawayInfo(t) => t.client_id(),
            PremiumGiveawayInfo::Completed(t) => t.client_id(),
            PremiumGiveawayInfo::Ongoing(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PremiumGiveawayInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PremiumGiveawayInfo::_Default)
    }
}

impl AsRef<PremiumGiveawayInfo> for PremiumGiveawayInfo {
    fn as_ref(&self) -> &PremiumGiveawayInfo {
        self
    }
}

/// Describes a completed giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiveawayInfoCompleted {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Point in time (Unix timestamp) when the giveaway was created

    #[serde(default)]
    creation_date: i32,
    /// Point in time (Unix timestamp) when the winners were selected. May be bigger than winners selection date specified in parameters of the giveaway

    #[serde(default)]
    actual_winners_selection_date: i32,
    /// True, if the giveaway was canceled and was fully refunded

    #[serde(default)]
    was_refunded: bool,
    /// Number of winners in the giveaway

    #[serde(default)]
    winner_count: i32,
    /// Number of winners, which activated their gift codes

    #[serde(default)]
    activation_count: i32,
    /// Telegram Premium gift code that was received by the current user; empty if the user isn't a winner in the giveaway

    #[serde(default)]
    gift_code: String,
}

impl RObject for PremiumGiveawayInfoCompleted {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumGiveawayInfo for PremiumGiveawayInfoCompleted {}

impl PremiumGiveawayInfoCompleted {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumGiveawayInfoCompletedBuilder {
        let mut inner = PremiumGiveawayInfoCompleted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumGiveawayInfoCompletedBuilder { inner }
    }

    pub fn creation_date(&self) -> i32 {
        self.creation_date
    }

    pub fn actual_winners_selection_date(&self) -> i32 {
        self.actual_winners_selection_date
    }

    pub fn was_refunded(&self) -> bool {
        self.was_refunded
    }

    pub fn winner_count(&self) -> i32 {
        self.winner_count
    }

    pub fn activation_count(&self) -> i32 {
        self.activation_count
    }

    pub fn gift_code(&self) -> &String {
        &self.gift_code
    }
}

#[doc(hidden)]
pub struct PremiumGiveawayInfoCompletedBuilder {
    inner: PremiumGiveawayInfoCompleted,
}

#[deprecated]
pub type RTDPremiumGiveawayInfoCompletedBuilder = PremiumGiveawayInfoCompletedBuilder;

impl PremiumGiveawayInfoCompletedBuilder {
    pub fn build(&self) -> PremiumGiveawayInfoCompleted {
        self.inner.clone()
    }

    pub fn creation_date(&mut self, creation_date: i32) -> &mut Self {
        self.inner.creation_date = creation_date;
        self
    }

    pub fn actual_winners_selection_date(
        &mut self,
        actual_winners_selection_date: i32,
    ) -> &mut Self {
        self.inner.actual_winners_selection_date = actual_winners_selection_date;
        self
    }

    pub fn was_refunded(&mut self, was_refunded: bool) -> &mut Self {
        self.inner.was_refunded = was_refunded;
        self
    }

    pub fn winner_count(&mut self, winner_count: i32) -> &mut Self {
        self.inner.winner_count = winner_count;
        self
    }

    pub fn activation_count(&mut self, activation_count: i32) -> &mut Self {
        self.inner.activation_count = activation_count;
        self
    }

    pub fn gift_code<T: AsRef<str>>(&mut self, gift_code: T) -> &mut Self {
        self.inner.gift_code = gift_code.as_ref().to_string();
        self
    }
}

impl AsRef<PremiumGiveawayInfoCompleted> for PremiumGiveawayInfoCompleted {
    fn as_ref(&self) -> &PremiumGiveawayInfoCompleted {
        self
    }
}

impl AsRef<PremiumGiveawayInfoCompleted> for PremiumGiveawayInfoCompletedBuilder {
    fn as_ref(&self) -> &PremiumGiveawayInfoCompleted {
        &self.inner
    }
}

/// Describes an ongoing giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiveawayInfoOngoing {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Point in time (Unix timestamp) when the giveaway was created

    #[serde(default)]
    creation_date: i32,
    /// Status of the current user in the giveaway

    #[serde(skip_serializing_if = "PremiumGiveawayParticipantStatus::_is_default")]
    status: PremiumGiveawayParticipantStatus,
    /// True, if the giveaway has ended and results are being prepared

    #[serde(default)]
    is_ended: bool,
}

impl RObject for PremiumGiveawayInfoOngoing {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumGiveawayInfo for PremiumGiveawayInfoOngoing {}

impl PremiumGiveawayInfoOngoing {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumGiveawayInfoOngoingBuilder {
        let mut inner = PremiumGiveawayInfoOngoing::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumGiveawayInfoOngoingBuilder { inner }
    }

    pub fn creation_date(&self) -> i32 {
        self.creation_date
    }

    pub fn status(&self) -> &PremiumGiveawayParticipantStatus {
        &self.status
    }

    pub fn is_ended(&self) -> bool {
        self.is_ended
    }
}

#[doc(hidden)]
pub struct PremiumGiveawayInfoOngoingBuilder {
    inner: PremiumGiveawayInfoOngoing,
}

#[deprecated]
pub type RTDPremiumGiveawayInfoOngoingBuilder = PremiumGiveawayInfoOngoingBuilder;

impl PremiumGiveawayInfoOngoingBuilder {
    pub fn build(&self) -> PremiumGiveawayInfoOngoing {
        self.inner.clone()
    }

    pub fn creation_date(&mut self, creation_date: i32) -> &mut Self {
        self.inner.creation_date = creation_date;
        self
    }

    pub fn status<T: AsRef<PremiumGiveawayParticipantStatus>>(&mut self, status: T) -> &mut Self {
        self.inner.status = status.as_ref().clone();
        self
    }

    pub fn is_ended(&mut self, is_ended: bool) -> &mut Self {
        self.inner.is_ended = is_ended;
        self
    }
}

impl AsRef<PremiumGiveawayInfoOngoing> for PremiumGiveawayInfoOngoing {
    fn as_ref(&self) -> &PremiumGiveawayInfoOngoing {
        self
    }
}

impl AsRef<PremiumGiveawayInfoOngoing> for PremiumGiveawayInfoOngoingBuilder {
    fn as_ref(&self) -> &PremiumGiveawayInfoOngoing {
        &self.inner
    }
}
