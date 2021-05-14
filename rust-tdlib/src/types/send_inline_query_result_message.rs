use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends the result of an inline query as a message. Returns the sent message. Always clears a chat draft message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendInlineQueryResultMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Target chat
    chat_id: i64,
    /// If not 0, a message thread identifier in which the message will be sent
    message_thread_id: i64,
    /// Identifier of a message to reply to or 0
    reply_to_message_id: i64,
    /// Options to be used to send the message
    options: MessageSendOptions,
    /// Identifier of the inline query

    #[serde(deserialize_with = "super::_common::number_from_string")]
    query_id: i64,
    /// Identifier of the inline result
    result_id: String,
    /// If true, there will be no mention of a bot, via which the message is sent. Can be used only for bots GetOption("animation_search_bot_username"), GetOption("photo_search_bot_username") and GetOption("venue_search_bot_username")
    hide_via_bot: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendInlineQueryResultMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendInlineQueryResultMessage {}

impl SendInlineQueryResultMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendInlineQueryResultMessageBuilder {
        let mut inner = SendInlineQueryResultMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendInlineQueryResultMessage".to_string();

        RTDSendInlineQueryResultMessageBuilder { inner }
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

    pub fn query_id(&self) -> i64 {
        self.query_id
    }

    pub fn result_id(&self) -> &String {
        &self.result_id
    }

    pub fn hide_via_bot(&self) -> bool {
        self.hide_via_bot
    }
}

#[doc(hidden)]
pub struct RTDSendInlineQueryResultMessageBuilder {
    inner: SendInlineQueryResultMessage,
}

impl RTDSendInlineQueryResultMessageBuilder {
    pub fn build(&self) -> SendInlineQueryResultMessage {
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

    pub fn query_id(&mut self, query_id: i64) -> &mut Self {
        self.inner.query_id = query_id;
        self
    }

    pub fn result_id<T: AsRef<str>>(&mut self, result_id: T) -> &mut Self {
        self.inner.result_id = result_id.as_ref().to_string();
        self
    }

    pub fn hide_via_bot(&mut self, hide_via_bot: bool) -> &mut Self {
        self.inner.hide_via_bot = hide_via_bot;
        self
    }
}

impl AsRef<SendInlineQueryResultMessage> for SendInlineQueryResultMessage {
    fn as_ref(&self) -> &SendInlineQueryResultMessage {
        self
    }
}

impl AsRef<SendInlineQueryResultMessage> for RTDSendInlineQueryResultMessageBuilder {
    fn as_ref(&self) -> &SendInlineQueryResultMessage {
        &self.inner
    }
}
