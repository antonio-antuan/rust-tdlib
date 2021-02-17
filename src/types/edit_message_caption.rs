use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Edits the message content caption. Returns the edited message after the edit is completed on the server side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditMessageCaption {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat the message belongs to
    chat_id: i64,
    /// Identifier of the message
    message_id: i64,
    /// The new message reply markup; for bots only

    #[serde(skip_serializing_if = "ReplyMarkup::_is_default")]
    reply_markup: ReplyMarkup,
    /// New message content caption; 0-GetOption("message_caption_length_max") characters
    caption: FormattedText,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditMessageCaption {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditMessageCaption {}

impl EditMessageCaption {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDEditMessageCaptionBuilder {
        let mut inner = EditMessageCaption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editMessageCaption".to_string();

        RTDEditMessageCaptionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }
}

#[doc(hidden)]
pub struct RTDEditMessageCaptionBuilder {
    inner: EditMessageCaption,
}

impl RTDEditMessageCaptionBuilder {
    pub fn build(&self) -> EditMessageCaption {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<EditMessageCaption> for EditMessageCaption {
    fn as_ref(&self) -> &EditMessageCaption {
        self
    }
}

impl AsRef<EditMessageCaption> for RTDEditMessageCaptionBuilder {
    fn as_ref(&self) -> &EditMessageCaption {
        &self.inner
    }
}
