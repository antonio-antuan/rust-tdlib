use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Edits the message reply markup; for bots only. Returns the edited message after the edit is completed on the server side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditMessageReplyMarkup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat the message belongs to

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message

    #[serde(default)]
    message_id: i64,
    /// The new message reply markup; pass null if none

    #[serde(skip_serializing_if = "ReplyMarkup::_is_default")]
    reply_markup: ReplyMarkup,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditMessageReplyMarkup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditMessageReplyMarkup {}

impl EditMessageReplyMarkup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EditMessageReplyMarkupBuilder {
        let mut inner = EditMessageReplyMarkup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editMessageReplyMarkup".to_string();

        EditMessageReplyMarkupBuilder { inner }
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
}

#[doc(hidden)]
pub struct EditMessageReplyMarkupBuilder {
    inner: EditMessageReplyMarkup,
}

#[deprecated]
pub type RTDEditMessageReplyMarkupBuilder = EditMessageReplyMarkupBuilder;

impl EditMessageReplyMarkupBuilder {
    pub fn build(&self) -> EditMessageReplyMarkup {
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
}

impl AsRef<EditMessageReplyMarkup> for EditMessageReplyMarkup {
    fn as_ref(&self) -> &EditMessageReplyMarkup {
        self
    }
}

impl AsRef<EditMessageReplyMarkup> for EditMessageReplyMarkupBuilder {
    fn as_ref(&self) -> &EditMessageReplyMarkup {
        &self.inner
    }
}
