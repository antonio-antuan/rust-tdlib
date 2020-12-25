
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a forwarded message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Origin of a forwarded message
  origin: MessageForwardOrigin,
  /// Point in time (Unix timestamp) when the message was originally sent
  date: i64,
  /// For messages forwarded to the chat with the current user (Saved Messages) or to the channel's discussion group, the identifier of the chat from which the message was forwarded last time; 0 if unknown
  from_chat_id: i64,
  /// For messages forwarded to the chat with the current user (Saved Messages) or to the channel's discussion group, the identifier of the original message from which the new message was forwarded last time; 0 if unknown
  from_message_id: i64,
  
}

impl RObject for MessageForwardInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageForwardInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl MessageForwardInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageForwardInfoBuilder {
    let mut inner = MessageForwardInfo::default();
    inner.td_name = "messageForwardInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageForwardInfoBuilder { inner }
  }

  pub fn origin(&self) -> &MessageForwardOrigin { &self.origin }

  pub fn date(&self) -> i64 { self.date }

  pub fn from_chat_id(&self) -> i64 { self.from_chat_id }

  pub fn from_message_id(&self) -> i64 { self.from_message_id }

}

#[doc(hidden)]
pub struct RTDMessageForwardInfoBuilder {
  inner: MessageForwardInfo
}

impl RTDMessageForwardInfoBuilder {
  pub fn build(&self) -> MessageForwardInfo { self.inner.clone() }

   
  pub fn origin<T: AsRef<MessageForwardOrigin>>(&mut self, origin: T) -> &mut Self {
    self.inner.origin = origin.as_ref().clone();
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

   
  pub fn from_chat_id(&mut self, from_chat_id: i64) -> &mut Self {
    self.inner.from_chat_id = from_chat_id;
    self
  }

   
  pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self {
    self.inner.from_message_id = from_message_id;
    self
  }

}

impl AsRef<MessageForwardInfo> for MessageForwardInfo {
  fn as_ref(&self) -> &MessageForwardInfo { self }
}

impl AsRef<MessageForwardInfo> for RTDMessageForwardInfoBuilder {
  fn as_ref(&self) -> &MessageForwardInfo { &self.inner }
}



