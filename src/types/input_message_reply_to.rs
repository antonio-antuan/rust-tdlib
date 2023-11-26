use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about the message or the story to be replied
pub trait TDInputMessageReplyTo: Debug + RObject {}

/// Contains information about the message or the story to be replied
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum InputMessageReplyTo {
    #[doc(hidden)]
    #[default]
    _Default,
    /// Describes a message to be replied
    #[serde(rename = "inputMessageReplyToMessage")]
    Message(InputMessageReplyToMessage),
    /// Describes a story to be replied
    #[serde(rename = "inputMessageReplyToStory")]
    Story(InputMessageReplyToStory),
}

impl RObject for InputMessageReplyTo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InputMessageReplyTo::Message(t) => t.extra(),
            InputMessageReplyTo::Story(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputMessageReplyTo::Message(t) => t.client_id(),
            InputMessageReplyTo::Story(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputMessageReplyTo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputMessageReplyTo::_Default)
    }
}

impl AsRef<InputMessageReplyTo> for InputMessageReplyTo {
    fn as_ref(&self) -> &InputMessageReplyTo {
        self
    }
}

/// Describes a message to be replied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageReplyToMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the chat to which the message to be replied belongs; pass 0 if the message to be replied is in the same chat. Must always be 0 for replies in secret chats. A message can be replied in another chat or topic only if message.can_be_replied_in_another_chat

    #[serde(default)]
    chat_id: i64,
    /// The identifier of the message to be replied in the same or the specified chat

    #[serde(default)]
    message_id: i64,
    /// Manually chosen quote from the message to be replied; pass null if none; 0-getOption("message_reply_quote_length_max") characters. Must always be null for replies in secret chats. Only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities are allowed to be kept and must be kept in the quote
    quote: FormattedText,
}

impl RObject for InputMessageReplyToMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageReplyTo for InputMessageReplyToMessage {}

impl InputMessageReplyToMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageReplyToMessageBuilder {
        let mut inner = InputMessageReplyToMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageReplyToMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn quote(&self) -> &FormattedText {
        &self.quote
    }
}

#[doc(hidden)]
pub struct InputMessageReplyToMessageBuilder {
    inner: InputMessageReplyToMessage,
}

#[deprecated]
pub type RTDInputMessageReplyToMessageBuilder = InputMessageReplyToMessageBuilder;

impl InputMessageReplyToMessageBuilder {
    pub fn build(&self) -> InputMessageReplyToMessage {
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
        self.inner.quote = quote.as_ref().clone();
        self
    }
}

impl AsRef<InputMessageReplyToMessage> for InputMessageReplyToMessage {
    fn as_ref(&self) -> &InputMessageReplyToMessage {
        self
    }
}

impl AsRef<InputMessageReplyToMessage> for InputMessageReplyToMessageBuilder {
    fn as_ref(&self) -> &InputMessageReplyToMessage {
        &self.inner
    }
}

/// Describes a story to be replied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageReplyToStory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the sender of the story. Currently, stories can be replied only in the sender's chat

    #[serde(default)]
    story_sender_chat_id: i64,
    /// The identifier of the story

    #[serde(default)]
    story_id: i32,
}

impl RObject for InputMessageReplyToStory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageReplyTo for InputMessageReplyToStory {}

impl InputMessageReplyToStory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageReplyToStoryBuilder {
        let mut inner = InputMessageReplyToStory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageReplyToStoryBuilder { inner }
    }

    pub fn story_sender_chat_id(&self) -> i64 {
        self.story_sender_chat_id
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }
}

#[doc(hidden)]
pub struct InputMessageReplyToStoryBuilder {
    inner: InputMessageReplyToStory,
}

#[deprecated]
pub type RTDInputMessageReplyToStoryBuilder = InputMessageReplyToStoryBuilder;

impl InputMessageReplyToStoryBuilder {
    pub fn build(&self) -> InputMessageReplyToStory {
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

impl AsRef<InputMessageReplyToStory> for InputMessageReplyToStory {
    fn as_ref(&self) -> &InputMessageReplyToStory {
        self
    }
}

impl AsRef<InputMessageReplyToStory> for InputMessageReplyToStoryBuilder {
    fn as_ref(&self) -> &InputMessageReplyToStory {
        &self.inner
    }
}
