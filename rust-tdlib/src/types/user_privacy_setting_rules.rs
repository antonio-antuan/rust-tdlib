use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRules {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A list of rules
    rules: Vec<UserPrivacySettingRule>,
}

impl RObject for UserPrivacySettingRules {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl UserPrivacySettingRules {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRulesBuilder {
        let mut inner = UserPrivacySettingRules::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserPrivacySettingRulesBuilder { inner }
    }

    pub fn rules(&self) -> &Vec<UserPrivacySettingRule> {
        &self.rules
    }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRulesBuilder {
    inner: UserPrivacySettingRules,
}

impl RTDUserPrivacySettingRulesBuilder {
    pub fn build(&self) -> UserPrivacySettingRules {
        self.inner.clone()
    }

    pub fn rules(&mut self, rules: Vec<UserPrivacySettingRule>) -> &mut Self {
        self.inner.rules = rules;
        self
    }
}

impl AsRef<UserPrivacySettingRules> for UserPrivacySettingRules {
    fn as_ref(&self) -> &UserPrivacySettingRules {
        self
    }
}

impl AsRef<UserPrivacySettingRules> for RTDUserPrivacySettingRulesBuilder {
    fn as_ref(&self) -> &UserPrivacySettingRules {
        &self.inner
    }
}
