
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a chat invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat invite link
  invite_link: String,
  
}

impl RObject for ChatInviteLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatInviteLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatInviteLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatInviteLinkBuilder {
    let mut inner = ChatInviteLink::default();
    inner.td_name = "chatInviteLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatInviteLinkBuilder { inner }
  }

  pub fn invite_link(&self) -> &String { &self.invite_link }

}

#[doc(hidden)]
pub struct RTDChatInviteLinkBuilder {
  inner: ChatInviteLink
}

impl RTDChatInviteLinkBuilder {
  pub fn build(&self) -> ChatInviteLink { self.inner.clone() }

   
  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }

}

impl AsRef<ChatInviteLink> for ChatInviteLink {
  fn as_ref(&self) -> &ChatInviteLink { self }
}

impl AsRef<ChatInviteLink> for RTDChatInviteLinkBuilder {
  fn as_ref(&self) -> &ChatInviteLink { &self.inner }
}



