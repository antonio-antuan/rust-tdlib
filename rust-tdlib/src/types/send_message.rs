use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends a message. Returns the sent message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Target chat
    chat_id: i64,
    /// If not 0, a message thread identifier in which the message will be sent
    message_thread_id: i64,
    /// Identifier of the message to reply to or 0
    reply_to_message_id: i64,
    /// Options to be used to send the message
    options: MessageSendOptions,
    /// Markup for replying to the message; for bots only

    #[serde(skip_serializing_if = "ReplyMarkup::_is_default")]
    reply_markup: ReplyMarkup,
    /// The content of the message to be sent

    #[serde(skip_serializing_if = "InputMessageContent::_is_default")]
    input_message_content: InputMessageContent,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendMessage {}

impl SendMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendMessageBuilder {
        let mut inner = SendMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendMessage".to_string();

        RTDSendMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn reply_to_message_id(&self) -> i64 {
        self.reply_to_message_id
    }

    pub fn options(&self) -> &MessageSendOptions {
        &self.options
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDSendMessageBuilder {
    inner: SendMessage,
}

impl RTDSendMessageBuilder {
    pub fn build(&self) -> SendMessage {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }

    pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
        self.inner.reply_to_message_id = reply_to_message_id;
        self
    }

    pub fn options<T: AsRef<MessageSendOptions>>(&mut self, options: T) -> &mut Self {
        self.inner.options = options.as_ref().clone();
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

impl AsRef<SendMessage> for SendMessage {
    fn as_ref(&self) -> &SendMessage {
        self
    }
}

impl AsRef<SendMessage> for RTDSendMessageBuilder {
    fn as_ref(&self) -> &SendMessage {
        &self.inner
    }
}
