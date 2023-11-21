use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes parameters of a Telegram Premium giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiveawayParameters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the channel chat, which will be automatically boosted by the winners of the giveaway for duration of the Premium subscription

    #[serde(default)]
    boosted_chat_id: i64,
    /// Identifiers of other channel chats that must be subscribed by the users to be eligible for the giveaway. There can be up to getOption("giveaway_additional_chat_count_max") additional chats

    #[serde(default)]
    additional_chat_ids: Vec<i64>,
    /// Point in time (Unix timestamp) when the giveaway is expected to be performed; must be 60-getOption("giveaway_duration_max") seconds in the future in scheduled giveaways

    #[serde(default)]
    winners_selection_date: i32,
    /// True, if only new subscribers of the chats will be eligible for the giveaway

    #[serde(default)]
    only_new_members: bool,
    /// The list of two-letter ISO 3166-1 alpha-2 codes of countries, users from which will be eligible for the giveaway. If empty, then all users can participate in the giveaway. There can be up to getOption("giveaway_country_count_max") chosen countries. Users with phone number that was bought on Fragment can participate in any giveaway and the country code "FT" must not be specified in the list

    #[serde(default)]
    country_codes: Vec<String>,
}

impl RObject for PremiumGiveawayParameters {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PremiumGiveawayParameters {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumGiveawayParametersBuilder {
        let mut inner = PremiumGiveawayParameters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumGiveawayParametersBuilder { inner }
    }

    pub fn boosted_chat_id(&self) -> i64 {
        self.boosted_chat_id
    }

    pub fn additional_chat_ids(&self) -> &Vec<i64> {
        &self.additional_chat_ids
    }

    pub fn winners_selection_date(&self) -> i32 {
        self.winners_selection_date
    }

    pub fn only_new_members(&self) -> bool {
        self.only_new_members
    }

    pub fn country_codes(&self) -> &Vec<String> {
        &self.country_codes
    }
}

#[doc(hidden)]
pub struct PremiumGiveawayParametersBuilder {
    inner: PremiumGiveawayParameters,
}

#[deprecated]
pub type RTDPremiumGiveawayParametersBuilder = PremiumGiveawayParametersBuilder;

impl PremiumGiveawayParametersBuilder {
    pub fn build(&self) -> PremiumGiveawayParameters {
        self.inner.clone()
    }

    pub fn boosted_chat_id(&mut self, boosted_chat_id: i64) -> &mut Self {
        self.inner.boosted_chat_id = boosted_chat_id;
        self
    }

    pub fn additional_chat_ids(&mut self, additional_chat_ids: Vec<i64>) -> &mut Self {
        self.inner.additional_chat_ids = additional_chat_ids;
        self
    }

    pub fn winners_selection_date(&mut self, winners_selection_date: i32) -> &mut Self {
        self.inner.winners_selection_date = winners_selection_date;
        self
    }

    pub fn only_new_members(&mut self, only_new_members: bool) -> &mut Self {
        self.inner.only_new_members = only_new_members;
        self
    }

    pub fn country_codes(&mut self, country_codes: Vec<String>) -> &mut Self {
        self.inner.country_codes = country_codes;
        self
    }
}

impl AsRef<PremiumGiveawayParameters> for PremiumGiveawayParameters {
    fn as_ref(&self) -> &PremiumGiveawayParameters {
        self
    }
}

impl AsRef<PremiumGiveawayParameters> for PremiumGiveawayParametersBuilder {
    fn as_ref(&self) -> &PremiumGiveawayParameters {
        &self.inner
    }
}
