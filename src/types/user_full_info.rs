
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains full information about a user (except the full list of profile photos)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the user is blacklisted by the current user
  is_blocked: bool,
  /// True, if the user can be called
  can_be_called: bool,
  /// True, if the user can't be called due to their privacy settings
  has_private_calls: bool,
  /// True, if the current user needs to explicitly allow to share their phone number with the user when the method addContact is used
  need_phone_number_privacy_exception: bool,
  /// A short user bio
  bio: String,
  /// For bots, the text that is included with the link when users share the bot
  share_text: String,
  /// Number of group chats where both the other user and the current user are a member; 0 for the current user
  group_in_common_count: i64,
  /// If the user is a bot, information about the bot; may be null
  bot_info: Option<BotInfo>,
  
}

impl RObject for UserFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userFullInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl UserFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserFullInfoBuilder {
    let mut inner = UserFullInfo::default();
    inner.td_name = "userFullInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUserFullInfoBuilder { inner }
  }

  pub fn is_blocked(&self) -> bool { self.is_blocked }

  pub fn can_be_called(&self) -> bool { self.can_be_called }

  pub fn has_private_calls(&self) -> bool { self.has_private_calls }

  pub fn need_phone_number_privacy_exception(&self) -> bool { self.need_phone_number_privacy_exception }

  pub fn bio(&self) -> &String { &self.bio }

  pub fn share_text(&self) -> &String { &self.share_text }

  pub fn group_in_common_count(&self) -> i64 { self.group_in_common_count }

  pub fn bot_info(&self) -> &Option<BotInfo> { &self.bot_info }

}

#[doc(hidden)]
pub struct RTDUserFullInfoBuilder {
  inner: UserFullInfo
}

impl RTDUserFullInfoBuilder {
  pub fn build(&self) -> UserFullInfo { self.inner.clone() }

   
  pub fn is_blocked(&mut self, is_blocked: bool) -> &mut Self {
    self.inner.is_blocked = is_blocked;
    self
  }

   
  pub fn can_be_called(&mut self, can_be_called: bool) -> &mut Self {
    self.inner.can_be_called = can_be_called;
    self
  }

   
  pub fn has_private_calls(&mut self, has_private_calls: bool) -> &mut Self {
    self.inner.has_private_calls = has_private_calls;
    self
  }

   
  pub fn need_phone_number_privacy_exception(&mut self, need_phone_number_privacy_exception: bool) -> &mut Self {
    self.inner.need_phone_number_privacy_exception = need_phone_number_privacy_exception;
    self
  }

   
  pub fn bio<T: AsRef<str>>(&mut self, bio: T) -> &mut Self {
    self.inner.bio = bio.as_ref().to_string();
    self
  }

   
  pub fn share_text<T: AsRef<str>>(&mut self, share_text: T) -> &mut Self {
    self.inner.share_text = share_text.as_ref().to_string();
    self
  }

   
  pub fn group_in_common_count(&mut self, group_in_common_count: i64) -> &mut Self {
    self.inner.group_in_common_count = group_in_common_count;
    self
  }

   
  pub fn bot_info<T: AsRef<BotInfo>>(&mut self, bot_info: T) -> &mut Self {
    self.inner.bot_info = Some(bot_info.as_ref().clone());
    self
  }

}

impl AsRef<UserFullInfo> for UserFullInfo {
  fn as_ref(&self) -> &UserFullInfo { self }
}

impl AsRef<UserFullInfo> for RTDUserFullInfoBuilder {
  fn as_ref(&self) -> &UserFullInfo { &self.inner }
}



