use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Edits the caption of an inline message sent via a bot; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditInlineMessageCaption {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Inline message identifier

    #[serde(default)]
    inline_message_id: String,
    /// The new message reply markup; pass null if none

    #[serde(skip_serializing_if = "ReplyMarkup::_is_default")]
    reply_markup: ReplyMarkup,
    /// New message content caption; pass null to remove caption; 0-GetOption("message_caption_length_max") characters
    caption: FormattedText,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditInlineMessageCaption {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditInlineMessageCaption {}

impl EditInlineMessageCaption {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EditInlineMessageCaptionBuilder {
        let mut inner = EditInlineMessageCaption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editInlineMessageCaption".to_string();

        EditInlineMessageCaptionBuilder { inner }
    }

    pub fn inline_message_id(&self) -> &String {
        &self.inline_message_id
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }
}

#[doc(hidden)]
pub struct EditInlineMessageCaptionBuilder {
    inner: EditInlineMessageCaption,
}

#[deprecated]
pub type RTDEditInlineMessageCaptionBuilder = EditInlineMessageCaptionBuilder;

impl EditInlineMessageCaptionBuilder {
    pub fn build(&self) -> EditInlineMessageCaption {
        self.inner.clone()
    }

    pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
        self.inner.inline_message_id = inline_message_id.as_ref().to_string();
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

impl AsRef<EditInlineMessageCaption> for EditInlineMessageCaption {
    fn as_ref(&self) -> &EditInlineMessageCaption {
        self
    }
}

impl AsRef<EditInlineMessageCaption> for EditInlineMessageCaptionBuilder {
    fn as_ref(&self) -> &EditInlineMessageCaption {
        &self.inner
    }
}
