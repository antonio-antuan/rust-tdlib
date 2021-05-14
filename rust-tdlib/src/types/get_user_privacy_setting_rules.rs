use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns the current privacy settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserPrivacySettingRules {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The privacy setting

    #[serde(skip_serializing_if = "UserPrivacySetting::_is_default")]
    setting: UserPrivacySetting,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetUserPrivacySettingRules {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetUserPrivacySettingRules {}

impl GetUserPrivacySettingRules {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetUserPrivacySettingRulesBuilder {
        let mut inner = GetUserPrivacySettingRules::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getUserPrivacySettingRules".to_string();

        RTDGetUserPrivacySettingRulesBuilder { inner }
    }

    pub fn setting(&self) -> &UserPrivacySetting {
        &self.setting
    }
}

#[doc(hidden)]
pub struct RTDGetUserPrivacySettingRulesBuilder {
    inner: GetUserPrivacySettingRules,
}

impl RTDGetUserPrivacySettingRulesBuilder {
    pub fn build(&self) -> GetUserPrivacySettingRules {
        self.inner.clone()
    }

    pub fn setting<T: AsRef<UserPrivacySetting>>(&mut self, setting: T) -> &mut Self {
        self.inner.setting = setting.as_ref().clone();
        self
    }
}

impl AsRef<GetUserPrivacySettingRules> for GetUserPrivacySettingRules {
    fn as_ref(&self) -> &GetUserPrivacySettingRules {
        self
    }
}

impl AsRef<GetUserPrivacySettingRules> for RTDGetUserPrivacySettingRulesBuilder {
    fn as_ref(&self) -> &GetUserPrivacySettingRules {
        &self.inner
    }
}
