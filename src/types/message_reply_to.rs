use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about the message or the story a message is replying to
pub trait TDMessageReplyTo: Debug + RObject {}

/// Contains information about the message or the story a message is replying to
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum MessageReplyTo {
    #[doc(hidden)]
    #[default]
    _Default,
    /// Describes a message replied by a given message
    #[serde(rename = "messageReplyToMessage")]
    Message(MessageReplyToMessage),
    /// Describes a story replied by a given message
    #[serde(rename = "messageReplyToStory")]
    Story(MessageReplyToStory),
}

impl RObject for MessageReplyTo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            MessageReplyTo::Message(t) => t.extra(),
            MessageReplyTo::Story(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            MessageReplyTo::Message(t) => t.client_id(),
            MessageReplyTo::Story(t) => t.client_id(),

            _ => None,
        }
    }
}

impl MessageReplyTo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MessageReplyTo::_Default)
    }
}

impl AsRef<MessageReplyTo> for MessageReplyTo {
    fn as_ref(&self) -> &MessageReplyTo {
        self
    }
}

/// Describes a message replied by a given message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReplyToMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the chat to which the message belongs; may be 0 if the replied message is in unknown chat

    #[serde(default)]
    chat_id: i64,
    /// The identifier of the message; may be 0 if the replied message is in unknown chat

    #[serde(default)]
    message_id: i64,
    /// Manually or automatically chosen quote from the replied message; may be null if none. Only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities can be present in the quote
    quote: Option<FormattedText>,
    /// True, if the quote was manually chosen by the message sender

    #[serde(default)]
    is_quote_manual: bool,
    /// Information about origin of the message if the message was from another chat or topic; may be null for messages from the same chat
    origin: Option<MessageOrigin>,
    /// Point in time (Unix timestamp) when the message was sent if the message was from another chat or topic; 0 for messages from the same chat

    #[serde(default)]
    origin_send_date: i32,
    /// Media content of the message if the message was from another chat or topic; may be null for messages from the same chat and messages without media. Can be only one of the following types: messageAnimation, messageAudio, messageContact, messageDice, messageDocument, messageGame, messageInvoice, messageLocation, messagePhoto, messagePoll, messagePremiumGiveaway, messageSticker, messageStory, messageText (for link preview), messageVenue, messageVideo, messageVideoNote, or messageVoiceNote
    content: Option<MessageContent>,
}

impl RObject for MessageReplyToMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageReplyTo for MessageReplyToMessage {}

impl MessageReplyToMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageReplyToMessageBuilder {
        let mut inner = MessageReplyToMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageReplyToMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn quote(&self) -> &Option<FormattedText> {
        &self.quote
    }

    pub fn is_quote_manual(&self) -> bool {
        self.is_quote_manual
    }

    pub fn origin(&self) -> &Option<MessageOrigin> {
        &self.origin
    }

    pub fn origin_send_date(&self) -> i32 {
        self.origin_send_date
    }

    pub fn content(&self) -> &Option<MessageContent> {
        &self.content
    }
}

#[doc(hidden)]
pub struct MessageReplyToMessageBuilder {
    inner: MessageReplyToMessage,
}

#[deprecated]
pub type RTDMessageReplyToMessageBuilder = MessageReplyToMessageBuilder;

impl MessageReplyToMessageBuilder {
    pub fn build(&self) -> MessageReplyToMessage {
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

    pub fn quote<T: AsRef<FormattedText>>(&mut self, quote: T) -> &mut Self {
        self.inner.quote = Some(quote.as_ref().clone());
        self
    }

    pub fn is_quote_manual(&mut self, is_quote_manual: bool) -> &mut Self {
        self.inner.is_quote_manual = is_quote_manual;
        self
    }

    pub fn origin<T: AsRef<MessageOrigin>>(&mut self, origin: T) -> &mut Self {
        self.inner.origin = Some(origin.as_ref().clone());
        self
    }

    pub fn origin_send_date(&mut self, origin_send_date: i32) -> &mut Self {
        self.inner.origin_send_date = origin_send_date;
        self
    }

    pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
        self.inner.content = Some(content.as_ref().clone());
        self
    }
}

impl AsRef<MessageReplyToMessage> for MessageReplyToMessage {
    fn as_ref(&self) -> &MessageReplyToMessage {
        self
    }
}

impl AsRef<MessageReplyToMessage> for MessageReplyToMessageBuilder {
    fn as_ref(&self) -> &MessageReplyToMessage {
        &self.inner
    }
}

/// Describes a story replied by a given message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReplyToStory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the sender of the story

    #[serde(default)]
    story_sender_chat_id: i64,
    /// The identifier of the story

    #[serde(default)]
    story_id: i32,
}

impl RObject for MessageReplyToStory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageReplyTo for MessageReplyToStory {}

impl MessageReplyToStory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageReplyToStoryBuilder {
        let mut inner = MessageReplyToStory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageReplyToStoryBuilder { inner }
    }

    pub fn story_sender_chat_id(&self) -> i64 {
        self.story_sender_chat_id
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }
}

#[doc(hidden)]
pub struct MessageReplyToStoryBuilder {
    inner: MessageReplyToStory,
}

#[deprecated]
pub type RTDMessageReplyToStoryBuilder = MessageReplyToStoryBuilder;

impl MessageReplyToStoryBuilder {
    pub fn build(&self) -> MessageReplyToStory {
        self.inner.clone()
    }

    pub fn story_sender_chat_id(&mut self, story_sender_chat_id: i64) -> &mut Self {
        self.inner.story_sender_chat_id = story_sender_chat_id;
        self
    }

    pub fn story_id(&mut self, story_id: i32) -> &mut Self {
        self.inner.story_id = story_id;
        self
    }
}

impl AsRef<MessageReplyToStory> for MessageReplyToStory {
    fn as_ref(&self) -> &MessageReplyToStory {
        self
    }
}

impl AsRef<MessageReplyToStory> for MessageReplyToStoryBuilder {
    fn as_ref(&self) -> &MessageReplyToStory {
        &self.inner
    }
}
