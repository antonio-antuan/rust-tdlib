
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a chat invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier of the invite link; 0 if the user is not a member of this chat
  chat_id: i64,
  /// Contains information about the type of the chat
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: ChatType,
  /// Title of the chat
  title: String,
  /// Chat photo; may be null
  photo: Option<ChatPhoto>,
  /// Number of members
  member_count: i64,
  /// User identifiers of some chat members that may be known to the current user
  member_user_ids: Vec<i64>,
  /// True, if the chat is a public supergroup or channel, i.e. it has a username or it is a location-based supergroup
  is_public: bool,
  
}

impl RObject for ChatInviteLinkInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatInviteLinkInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatInviteLinkInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatInviteLinkInfoBuilder {
    let mut inner = ChatInviteLinkInfo::default();
    inner.td_name = "chatInviteLinkInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatInviteLinkInfoBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn type_(&self) -> &ChatType { &self.type_ }

  pub fn title(&self) -> &String { &self.title }

  pub fn photo(&self) -> &Option<ChatPhoto> { &self.photo }

  pub fn member_count(&self) -> i64 { self.member_count }

  pub fn member_user_ids(&self) -> &Vec<i64> { &self.member_user_ids }

  pub fn is_public(&self) -> bool { self.is_public }

}

#[doc(hidden)]
pub struct RTDChatInviteLinkInfoBuilder {
  inner: ChatInviteLinkInfo
}

impl RTDChatInviteLinkInfoBuilder {
  pub fn build(&self) -> ChatInviteLinkInfo { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn type_<T: AsRef<ChatType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

   
  pub fn member_count(&mut self, member_count: i64) -> &mut Self {
    self.inner.member_count = member_count;
    self
  }

   
  pub fn member_user_ids(&mut self, member_user_ids: Vec<i64>) -> &mut Self {
    self.inner.member_user_ids = member_user_ids;
    self
  }

   
  pub fn is_public(&mut self, is_public: bool) -> &mut Self {
    self.inner.is_public = is_public;
    self
  }

}

impl AsRef<ChatInviteLinkInfo> for ChatInviteLinkInfo {
  fn as_ref(&self) -> &ChatInviteLinkInfo { self }
}

impl AsRef<ChatInviteLinkInfo> for RTDChatInviteLinkInfoBuilder {
  fn as_ref(&self) -> &ChatInviteLinkInfo { &self.inner }
}



