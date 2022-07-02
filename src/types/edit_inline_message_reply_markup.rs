use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Edits the reply markup of an inline message sent via a bot; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditInlineMessageReplyMarkup {
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

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditInlineMessageReplyMarkup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditInlineMessageReplyMarkup {}

impl EditInlineMessageReplyMarkup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EditInlineMessageReplyMarkupBuilder {
        let mut inner = EditInlineMessageReplyMarkup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editInlineMessageReplyMarkup".to_string();

        EditInlineMessageReplyMarkupBuilder { inner }
    }

    pub fn inline_message_id(&self) -> &String {
        &self.inline_message_id
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }
}

#[doc(hidden)]
pub struct EditInlineMessageReplyMarkupBuilder {
    inner: EditInlineMessageReplyMarkup,
}

#[deprecated]
pub type RTDEditInlineMessageReplyMarkupBuilder = EditInlineMessageReplyMarkupBuilder;

impl EditInlineMessageReplyMarkupBuilder {
    pub fn build(&self) -> EditInlineMessageReplyMarkup {
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
}

impl AsRef<EditInlineMessageReplyMarkup> for EditInlineMessageReplyMarkup {
    fn as_ref(&self) -> &EditInlineMessageReplyMarkup {
        self
    }
}

impl AsRef<EditInlineMessageReplyMarkup> for EditInlineMessageReplyMarkupBuilder {
    fn as_ref(&self) -> &EditInlineMessageReplyMarkup {
        &self.inner
    }
}
