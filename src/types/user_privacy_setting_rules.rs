
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRules {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A list of rules
  rules: Vec<UserPrivacySettingRule>,
  
}

impl RObject for UserPrivacySettingRules {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRules" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl UserPrivacySettingRules {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingRulesBuilder {
    let mut inner = UserPrivacySettingRules::default();
    inner.td_name = "userPrivacySettingRules".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUserPrivacySettingRulesBuilder { inner }
  }

  pub fn rules(&self) -> &Vec<UserPrivacySettingRule> { &self.rules }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRulesBuilder {
  inner: UserPrivacySettingRules
}

impl RTDUserPrivacySettingRulesBuilder {
  pub fn build(&self) -> UserPrivacySettingRules { self.inner.clone() }

   
  pub fn rules(&mut self, rules: Vec<UserPrivacySettingRule>) -> &mut Self {
    self.inner.rules = rules;
    self
  }

}

impl AsRef<UserPrivacySettingRules> for UserPrivacySettingRules {
  fn as_ref(&self) -> &UserPrivacySettingRules { self }
}

impl AsRef<UserPrivacySettingRules> for RTDUserPrivacySettingRulesBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRules { &self.inner }
}



