use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes user privacy settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetUserPrivacySettingRules {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The privacy setting

    #[serde(skip_serializing_if = "UserPrivacySetting::_is_default")]
    setting: UserPrivacySetting,
    /// The new privacy rules
    rules: UserPrivacySettingRules,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetUserPrivacySettingRules {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetUserPrivacySettingRules {}

impl SetUserPrivacySettingRules {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetUserPrivacySettingRulesBuilder {
        let mut inner = SetUserPrivacySettingRules::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setUserPrivacySettingRules".to_string();

        RTDSetUserPrivacySettingRulesBuilder { inner }
    }

    pub fn setting(&self) -> &UserPrivacySetting {
        &self.setting
    }

    pub fn rules(&self) -> &UserPrivacySettingRules {
        &self.rules
    }
}

#[doc(hidden)]
pub struct RTDSetUserPrivacySettingRulesBuilder {
    inner: SetUserPrivacySettingRules,
}

impl RTDSetUserPrivacySettingRulesBuilder {
    pub fn build(&self) -> SetUserPrivacySettingRules {
        self.inner.clone()
    }

    pub fn setting<T: AsRef<UserPrivacySetting>>(&mut self, setting: T) -> &mut Self {
        self.inner.setting = setting.as_ref().clone();
        self
    }

    pub fn rules<T: AsRef<UserPrivacySettingRules>>(&mut self, rules: T) -> &mut Self {
        self.inner.rules = rules.as_ref().clone();
        self
    }
}

impl AsRef<SetUserPrivacySettingRules> for SetUserPrivacySettingRules {
    fn as_ref(&self) -> &SetUserPrivacySettingRules {
        self
    }
}

impl AsRef<SetUserPrivacySettingRules> for RTDSetUserPrivacySettingRulesBuilder {
    fn as_ref(&self) -> &SetUserPrivacySettingRules {
        &self.inner
    }
}
