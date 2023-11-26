use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Launches a prepaid Telegram Premium giveaway for subscribers of channel chats; requires can_post_messages rights in the channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaunchPrepaidPremiumGiveaway {
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
    giveaway_id: i64,
    /// Giveaway parameters
    parameters: PremiumGiveawayParameters,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for LaunchPrepaidPremiumGiveaway {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for LaunchPrepaidPremiumGiveaway {}

impl LaunchPrepaidPremiumGiveaway {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LaunchPrepaidPremiumGiveawayBuilder {
        let mut inner = LaunchPrepaidPremiumGiveaway::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "launchPrepaidPremiumGiveaway".to_string();

        LaunchPrepaidPremiumGiveawayBuilder { inner }
    }

    pub fn giveaway_id(&self) -> i64 {
        self.giveaway_id
    }

    pub fn parameters(&self) -> &PremiumGiveawayParameters {
        &self.parameters
    }
}

#[doc(hidden)]
pub struct LaunchPrepaidPremiumGiveawayBuilder {
    inner: LaunchPrepaidPremiumGiveaway,
}

#[deprecated]
pub type RTDLaunchPrepaidPremiumGiveawayBuilder = LaunchPrepaidPremiumGiveawayBuilder;

impl LaunchPrepaidPremiumGiveawayBuilder {
    pub fn build(&self) -> LaunchPrepaidPremiumGiveaway {
        self.inner.clone()
    }

    pub fn giveaway_id(&mut self, giveaway_id: i64) -> &mut Self {
        self.inner.giveaway_id = giveaway_id;
        self
    }

    pub fn parameters<T: AsRef<PremiumGiveawayParameters>>(&mut self, parameters: T) -> &mut Self {
        self.inner.parameters = parameters.as_ref().clone();
        self
    }
}

impl AsRef<LaunchPrepaidPremiumGiveaway> for LaunchPrepaidPremiumGiveaway {
    fn as_ref(&self) -> &LaunchPrepaidPremiumGiveaway {
        self
    }
}

impl AsRef<LaunchPrepaidPremiumGiveaway> for LaunchPrepaidPremiumGiveawayBuilder {
    fn as_ref(&self) -> &LaunchPrepaidPremiumGiveaway {
        &self.inner
    }
}
