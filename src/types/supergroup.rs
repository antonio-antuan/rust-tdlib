
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a supergroup or channel with zero or more members (subscribers in the case of channels). From the point of view of the system, a channel is a special kind of a supergroup: only administrators can post and see the list of members, and posts from all administrators use the name and photo of the channel instead of individual names and profile photos. Unlike supergroups, channels can have an unlimited number of subscribers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Supergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Supergroup or channel identifier
  id: i64,
  /// Username of the supergroup or channel; empty for private supergroups or channels
  username: String,
  /// Point in time (Unix timestamp) when the current user joined, or the point in time when the supergroup or channel was created, in case the user is not a member
  date: i64,
  /// Status of the current user in the supergroup or channel; custom title will be always empty
  status: ChatMemberStatus,
  /// Member count; 0 if unknown. Currently it is guaranteed to be known only if the supergroup or channel was found through SearchPublicChats
  member_count: i64,
  /// True, if the channel has a discussion group, or the supergroup is the designated discussion group for a channel
  has_linked_chat: bool,
  /// True, if the supergroup is connected to a location, i.e. the supergroup is a location-based supergroup
  has_location: bool,
  /// True, if messages sent to the channel should contain information about the sender. This field is only applicable to channels
  sign_messages: bool,
  /// True, if the slow mode is enabled in the supergroup
  is_slow_mode_enabled: bool,
  /// True, if the supergroup is a channel
  is_channel: bool,
  /// True, if the supergroup or channel is verified
  is_verified: bool,
  /// If non-empty, contains a human-readable description of the reason why access to this supergroup or channel must be restricted
  restriction_reason: String,
  /// True, if many users reported this supergroup as a scam
  is_scam: bool,
  
}

impl RObject for Supergroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "supergroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Supergroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSupergroupBuilder {
    let mut inner = Supergroup::default();
    inner.td_name = "supergroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSupergroupBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

  pub fn username(&self) -> &String { &self.username }

  pub fn date(&self) -> i64 { self.date }

  pub fn status(&self) -> &ChatMemberStatus { &self.status }

  pub fn member_count(&self) -> i64 { self.member_count }

  pub fn has_linked_chat(&self) -> bool { self.has_linked_chat }

  pub fn has_location(&self) -> bool { self.has_location }

  pub fn sign_messages(&self) -> bool { self.sign_messages }

  pub fn is_slow_mode_enabled(&self) -> bool { self.is_slow_mode_enabled }

  pub fn is_channel(&self) -> bool { self.is_channel }

  pub fn is_verified(&self) -> bool { self.is_verified }

  pub fn restriction_reason(&self) -> &String { &self.restriction_reason }

  pub fn is_scam(&self) -> bool { self.is_scam }

}

#[doc(hidden)]
pub struct RTDSupergroupBuilder {
  inner: Supergroup
}

impl RTDSupergroupBuilder {
  pub fn build(&self) -> Supergroup { self.inner.clone() }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
    self.inner.username = username.as_ref().to_string();
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

   
  pub fn status<T: AsRef<ChatMemberStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }

   
  pub fn member_count(&mut self, member_count: i64) -> &mut Self {
    self.inner.member_count = member_count;
    self
  }

   
  pub fn has_linked_chat(&mut self, has_linked_chat: bool) -> &mut Self {
    self.inner.has_linked_chat = has_linked_chat;
    self
  }

   
  pub fn has_location(&mut self, has_location: bool) -> &mut Self {
    self.inner.has_location = has_location;
    self
  }

   
  pub fn sign_messages(&mut self, sign_messages: bool) -> &mut Self {
    self.inner.sign_messages = sign_messages;
    self
  }

   
  pub fn is_slow_mode_enabled(&mut self, is_slow_mode_enabled: bool) -> &mut Self {
    self.inner.is_slow_mode_enabled = is_slow_mode_enabled;
    self
  }

   
  pub fn is_channel(&mut self, is_channel: bool) -> &mut Self {
    self.inner.is_channel = is_channel;
    self
  }

   
  pub fn is_verified(&mut self, is_verified: bool) -> &mut Self {
    self.inner.is_verified = is_verified;
    self
  }

   
  pub fn restriction_reason<T: AsRef<str>>(&mut self, restriction_reason: T) -> &mut Self {
    self.inner.restriction_reason = restriction_reason.as_ref().to_string();
    self
  }

   
  pub fn is_scam(&mut self, is_scam: bool) -> &mut Self {
    self.inner.is_scam = is_scam;
    self
  }

}

impl AsRef<Supergroup> for Supergroup {
  fn as_ref(&self) -> &Supergroup { self }
}

impl AsRef<Supergroup> for RTDSupergroupBuilder {
  fn as_ref(&self) -> &Supergroup { &self.inner }
}



