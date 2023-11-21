use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether aggressive anti-spam checks are enabled in the supergroup. Can be called only if supergroupFullInfo.can_toggle_aggressive_anti_spam == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSupergroupHasAggressiveAntiSpamEnabled {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the supergroup, which isn't a broadcast group

    #[serde(default)]
    supergroup_id: i64,
    /// The new value of has_aggressive_anti_spam_enabled

    #[serde(default)]
    has_aggressive_anti_spam_enabled: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleSupergroupHasAggressiveAntiSpamEnabled {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleSupergroupHasAggressiveAntiSpamEnabled {}

impl ToggleSupergroupHasAggressiveAntiSpamEnabled {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleSupergroupHasAggressiveAntiSpamEnabledBuilder {
        let mut inner = ToggleSupergroupHasAggressiveAntiSpamEnabled::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleSupergroupHasAggressiveAntiSpamEnabled".to_string();

        ToggleSupergroupHasAggressiveAntiSpamEnabledBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn has_aggressive_anti_spam_enabled(&self) -> bool {
        self.has_aggressive_anti_spam_enabled
    }
}

#[doc(hidden)]
pub struct ToggleSupergroupHasAggressiveAntiSpamEnabledBuilder {
    inner: ToggleSupergroupHasAggressiveAntiSpamEnabled,
}

#[deprecated]
pub type RTDToggleSupergroupHasAggressiveAntiSpamEnabledBuilder =
    ToggleSupergroupHasAggressiveAntiSpamEnabledBuilder;

impl ToggleSupergroupHasAggressiveAntiSpamEnabledBuilder {
    pub fn build(&self) -> ToggleSupergroupHasAggressiveAntiSpamEnabled {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn has_aggressive_anti_spam_enabled(
        &mut self,
        has_aggressive_anti_spam_enabled: bool,
    ) -> &mut Self {
        self.inner.has_aggressive_anti_spam_enabled = has_aggressive_anti_spam_enabled;
        self
    }
}

impl AsRef<ToggleSupergroupHasAggressiveAntiSpamEnabled>
    for ToggleSupergroupHasAggressiveAntiSpamEnabled
{
    fn as_ref(&self) -> &ToggleSupergroupHasAggressiveAntiSpamEnabled {
        self
    }
}

impl AsRef<ToggleSupergroupHasAggressiveAntiSpamEnabled>
    for ToggleSupergroupHasAggressiveAntiSpamEnabledBuilder
{
    fn as_ref(&self) -> &ToggleSupergroupHasAggressiveAntiSpamEnabled {
        &self.inner
    }
}
