use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes type of a message sponsor
pub trait TDMessageSponsorType: Debug + RObject {}

/// Describes type of a message sponsor
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum MessageSponsorType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The sponsor is a bot
    #[serde(rename = "messageSponsorTypeBot")]
    Bot(MessageSponsorTypeBot),
    /// The sponsor is a private channel chat
    #[serde(rename = "messageSponsorTypePrivateChannel")]
    PrivateChannel(MessageSponsorTypePrivateChannel),
    /// The sponsor is a public channel chat
    #[serde(rename = "messageSponsorTypePublicChannel")]
    PublicChannel(MessageSponsorTypePublicChannel),
    /// The sponsor is a website
    #[serde(rename = "messageSponsorTypeWebsite")]
    Website(MessageSponsorTypeWebsite),
}

impl RObject for MessageSponsorType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            MessageSponsorType::Bot(t) => t.extra(),
            MessageSponsorType::PrivateChannel(t) => t.extra(),
            MessageSponsorType::PublicChannel(t) => t.extra(),
            MessageSponsorType::Website(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            MessageSponsorType::Bot(t) => t.client_id(),
            MessageSponsorType::PrivateChannel(t) => t.client_id(),
            MessageSponsorType::PublicChannel(t) => t.client_id(),
            MessageSponsorType::Website(t) => t.client_id(),

            _ => None,
        }
    }
}

impl MessageSponsorType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MessageSponsorType::_Default)
    }
}

impl AsRef<MessageSponsorType> for MessageSponsorType {
    fn as_ref(&self) -> &MessageSponsorType {
        self
    }
}

/// The sponsor is a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSponsorTypeBot {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier of the bot

    #[serde(default)]
    bot_user_id: i64,
    /// An internal link to be opened when the sponsored message is clicked

    #[serde(skip_serializing_if = "InternalLinkType::_is_default")]
    link: InternalLinkType,
}

impl RObject for MessageSponsorTypeBot {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSponsorType for MessageSponsorTypeBot {}

impl MessageSponsorTypeBot {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSponsorTypeBotBuilder {
        let mut inner = MessageSponsorTypeBot::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSponsorTypeBotBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn link(&self) -> &InternalLinkType {
        &self.link
    }
}

#[doc(hidden)]
pub struct MessageSponsorTypeBotBuilder {
    inner: MessageSponsorTypeBot,
}

#[deprecated]
pub type RTDMessageSponsorTypeBotBuilder = MessageSponsorTypeBotBuilder;

impl MessageSponsorTypeBotBuilder {
    pub fn build(&self) -> MessageSponsorTypeBot {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn link<T: AsRef<InternalLinkType>>(&mut self, link: T) -> &mut Self {
        self.inner.link = link.as_ref().clone();
        self
    }
}

impl AsRef<MessageSponsorTypeBot> for MessageSponsorTypeBot {
    fn as_ref(&self) -> &MessageSponsorTypeBot {
        self
    }
}

impl AsRef<MessageSponsorTypeBot> for MessageSponsorTypeBotBuilder {
    fn as_ref(&self) -> &MessageSponsorTypeBot {
        &self.inner
    }
}

/// The sponsor is a private channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSponsorTypePrivateChannel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Title of the chat

    #[serde(default)]
    title: String,
    /// Invite link for the channel

    #[serde(default)]
    invite_link: String,
}

impl RObject for MessageSponsorTypePrivateChannel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSponsorType for MessageSponsorTypePrivateChannel {}

impl MessageSponsorTypePrivateChannel {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSponsorTypePrivateChannelBuilder {
        let mut inner = MessageSponsorTypePrivateChannel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSponsorTypePrivateChannelBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }
}

#[doc(hidden)]
pub struct MessageSponsorTypePrivateChannelBuilder {
    inner: MessageSponsorTypePrivateChannel,
}

#[deprecated]
pub type RTDMessageSponsorTypePrivateChannelBuilder = MessageSponsorTypePrivateChannelBuilder;

impl MessageSponsorTypePrivateChannelBuilder {
    pub fn build(&self) -> MessageSponsorTypePrivateChannel {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }
}

impl AsRef<MessageSponsorTypePrivateChannel> for MessageSponsorTypePrivateChannel {
    fn as_ref(&self) -> &MessageSponsorTypePrivateChannel {
        self
    }
}

impl AsRef<MessageSponsorTypePrivateChannel> for MessageSponsorTypePrivateChannelBuilder {
    fn as_ref(&self) -> &MessageSponsorTypePrivateChannel {
        &self.inner
    }
}

/// The sponsor is a public channel chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSponsorTypePublicChannel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sponsor chat identifier

    #[serde(default)]
    chat_id: i64,
    /// An internal link to be opened when the sponsored message is clicked; may be null if the sponsor chat needs to be opened instead
    link: Option<InternalLinkType>,
}

impl RObject for MessageSponsorTypePublicChannel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSponsorType for MessageSponsorTypePublicChannel {}

impl MessageSponsorTypePublicChannel {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSponsorTypePublicChannelBuilder {
        let mut inner = MessageSponsorTypePublicChannel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSponsorTypePublicChannelBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn link(&self) -> &Option<InternalLinkType> {
        &self.link
    }
}

#[doc(hidden)]
pub struct MessageSponsorTypePublicChannelBuilder {
    inner: MessageSponsorTypePublicChannel,
}

#[deprecated]
pub type RTDMessageSponsorTypePublicChannelBuilder = MessageSponsorTypePublicChannelBuilder;

impl MessageSponsorTypePublicChannelBuilder {
    pub fn build(&self) -> MessageSponsorTypePublicChannel {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn link<T: AsRef<InternalLinkType>>(&mut self, link: T) -> &mut Self {
        self.inner.link = Some(link.as_ref().clone());
        self
    }
}

impl AsRef<MessageSponsorTypePublicChannel> for MessageSponsorTypePublicChannel {
    fn as_ref(&self) -> &MessageSponsorTypePublicChannel {
        self
    }
}

impl AsRef<MessageSponsorTypePublicChannel> for MessageSponsorTypePublicChannelBuilder {
    fn as_ref(&self) -> &MessageSponsorTypePublicChannel {
        &self.inner
    }
}

/// The sponsor is a website
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSponsorTypeWebsite {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// URL of the website

    #[serde(default)]
    url: String,
    /// Name of the website

    #[serde(default)]
    name: String,
}

impl RObject for MessageSponsorTypeWebsite {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSponsorType for MessageSponsorTypeWebsite {}

impl MessageSponsorTypeWebsite {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSponsorTypeWebsiteBuilder {
        let mut inner = MessageSponsorTypeWebsite::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSponsorTypeWebsiteBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct MessageSponsorTypeWebsiteBuilder {
    inner: MessageSponsorTypeWebsite,
}

#[deprecated]
pub type RTDMessageSponsorTypeWebsiteBuilder = MessageSponsorTypeWebsiteBuilder;

impl MessageSponsorTypeWebsiteBuilder {
    pub fn build(&self) -> MessageSponsorTypeWebsite {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }
}

impl AsRef<MessageSponsorTypeWebsite> for MessageSponsorTypeWebsite {
    fn as_ref(&self) -> &MessageSponsorTypeWebsite {
        self
    }
}

impl AsRef<MessageSponsorTypeWebsite> for MessageSponsorTypeWebsiteBuilder {
    fn as_ref(&self) -> &MessageSponsorTypeWebsite {
        &self.inner
    }
}
