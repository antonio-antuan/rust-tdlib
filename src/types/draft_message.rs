
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a message draft
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DraftMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the message to reply to; 0 if none
  reply_to_message_id: i64,
  /// Content of the message draft; this should always be of type inputMessageText
  input_message_text: InputMessageContent,
  
}

impl RObject for DraftMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "draftMessage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl DraftMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDraftMessageBuilder {
    let mut inner = DraftMessage::default();
    inner.td_name = "draftMessage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDDraftMessageBuilder { inner }
  }

  pub fn reply_to_message_id(&self) -> i64 { self.reply_to_message_id }

  pub fn input_message_text(&self) -> &InputMessageContent { &self.input_message_text }

}

#[doc(hidden)]
pub struct RTDDraftMessageBuilder {
  inner: DraftMessage
}

impl RTDDraftMessageBuilder {
  pub fn build(&self) -> DraftMessage { self.inner.clone() }

   
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
    self.inner.reply_to_message_id = reply_to_message_id;
    self
  }

   
  pub fn input_message_text<T: AsRef<InputMessageContent>>(&mut self, input_message_text: T) -> &mut Self {
    self.inner.input_message_text = input_message_text.as_ref().clone();
    self
  }

}

impl AsRef<DraftMessage> for DraftMessage {
  fn as_ref(&self) -> &DraftMessage { self }
}

impl AsRef<DraftMessage> for RTDDraftMessageBuilder {
  fn as_ref(&self) -> &DraftMessage { &self.inner }
}



