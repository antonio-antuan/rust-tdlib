use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Edits the text of an inline text or game message sent via a bot; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditInlineMessageText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Inline message identifier
    inline_message_id: String,
    /// The new message reply markup

    #[serde(skip_serializing_if = "ReplyMarkup::_is_default")]
    reply_markup: ReplyMarkup,
    /// New text content of the message. Should be of type InputMessageText

    #[serde(skip_serializing_if = "InputMessageContent::_is_default")]
    input_message_content: InputMessageContent,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditInlineMessageText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditInlineMessageText {}

impl EditInlineMessageText {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDEditInlineMessageTextBuilder {
        let mut inner = EditInlineMessageText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editInlineMessageText".to_string();

        RTDEditInlineMessageTextBuilder { inner }
    }

    pub fn inline_message_id(&self) -> &String {
        &self.inline_message_id
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDEditInlineMessageTextBuilder {
    inner: EditInlineMessageText,
}

impl RTDEditInlineMessageTextBuilder {
    pub fn build(&self) -> EditInlineMessageText {
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

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<EditInlineMessageText> for EditInlineMessageText {
    fn as_ref(&self) -> &EditInlineMessageText {
        self
    }
}

impl AsRef<EditInlineMessageText> for RTDEditInlineMessageTextBuilder {
    fn as_ref(&self) -> &EditInlineMessageText {
        &self.inner
    }
}
