
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a link to a message in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the link is a public link for a message in a chat
  is_public: bool,
  /// If found, identifier of the chat to which the message belongs, 0 otherwise
  chat_id: i64,
  /// If found, the linked message; may be null
  message: Option<Message>,
  /// True, if the whole media album to which the message belongs is linked
  for_album: bool,
  
}

impl RObject for MessageLinkInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageLinkInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl MessageLinkInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageLinkInfoBuilder {
    let mut inner = MessageLinkInfo::default();
    inner.td_name = "messageLinkInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageLinkInfoBuilder { inner }
  }

  pub fn is_public(&self) -> bool { self.is_public }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message(&self) -> &Option<Message> { &self.message }

  pub fn for_album(&self) -> bool { self.for_album }

}

#[doc(hidden)]
pub struct RTDMessageLinkInfoBuilder {
  inner: MessageLinkInfo
}

impl RTDMessageLinkInfoBuilder {
  pub fn build(&self) -> MessageLinkInfo { self.inner.clone() }

   
  pub fn is_public(&mut self, is_public: bool) -> &mut Self {
    self.inner.is_public = is_public;
    self
  }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = Some(message.as_ref().clone());
    self
  }

   
  pub fn for_album(&mut self, for_album: bool) -> &mut Self {
    self.inner.for_album = for_album;
    self
  }

}

impl AsRef<MessageLinkInfo> for MessageLinkInfo {
  fn as_ref(&self) -> &MessageLinkInfo { self }
}

impl AsRef<MessageLinkInfo> for RTDMessageLinkInfoBuilder {
  fn as_ref(&self) -> &MessageLinkInfo { &self.inner }
}



