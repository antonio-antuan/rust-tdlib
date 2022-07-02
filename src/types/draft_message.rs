use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a message draft
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DraftMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the message to reply to; 0 if none

    #[serde(default)]
    reply_to_message_id: i64,
    /// Point in time (Unix timestamp) when the draft was created

    #[serde(default)]
    date: i32,
    /// Content of the message draft; must be of the type inputMessageText

    #[serde(skip_serializing_if = "InputMessageContent::_is_default")]
    input_message_text: InputMessageContent,
}

impl RObject for DraftMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl DraftMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DraftMessageBuilder {
        let mut inner = DraftMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DraftMessageBuilder { inner }
    }

    pub fn reply_to_message_id(&self) -> i64 {
        self.reply_to_message_id
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn input_message_text(&self) -> &InputMessageContent {
        &self.input_message_text
    }
}

#[doc(hidden)]
pub struct DraftMessageBuilder {
    inner: DraftMessage,
}

#[deprecated]
pub type RTDDraftMessageBuilder = DraftMessageBuilder;

impl DraftMessageBuilder {
    pub fn build(&self) -> DraftMessage {
        self.inner.clone()
    }

    pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
        self.inner.reply_to_message_id = reply_to_message_id;
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn input_message_text<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_text: T,
    ) -> &mut Self {
        self.inner.input_message_text = input_message_text.as_ref().clone();
        self
    }
}

impl AsRef<DraftMessage> for DraftMessage {
    fn as_ref(&self) -> &DraftMessage {
        self
    }
}

impl AsRef<DraftMessage> for DraftMessageBuilder {
    fn as_ref(&self) -> &DraftMessage {
        &self.inner
    }
}
