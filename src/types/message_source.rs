use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes source of a message
pub trait TDMessageSource: Debug + RObject {}

/// Describes source of a message
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum MessageSource {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The message is from a chat event log
    #[serde(rename = "messageSourceChatEventLog")]
    ChatEventLog(MessageSourceChatEventLog),
    /// The message is from a chat history
    #[serde(rename = "messageSourceChatHistory")]
    ChatHistory(MessageSourceChatHistory),
    /// The message is from a chat list or a forum topic list
    #[serde(rename = "messageSourceChatList")]
    ChatList(MessageSourceChatList),
    /// The message is from a forum topic history
    #[serde(rename = "messageSourceForumTopicHistory")]
    ForumTopicHistory(MessageSourceForumTopicHistory),
    /// The message is from chat, message thread or forum topic history preview
    #[serde(rename = "messageSourceHistoryPreview")]
    HistoryPreview(MessageSourceHistoryPreview),
    /// The message is from a message thread history
    #[serde(rename = "messageSourceMessageThreadHistory")]
    MessageThreadHistory(MessageSourceMessageThreadHistory),
    /// The message is from a notification
    #[serde(rename = "messageSourceNotification")]
    Notification(MessageSourceNotification),
    /// The message is from some other source
    #[serde(rename = "messageSourceOther")]
    Other(MessageSourceOther),
    /// The message was screenshotted; the source must be used only if the message content was visible during the screenshot
    #[serde(rename = "messageSourceScreenshot")]
    Screenshot(MessageSourceScreenshot),
    /// The message is from search results, including file downloads, local file list, outgoing document messages, calendar
    #[serde(rename = "messageSourceSearch")]
    Search(MessageSourceSearch),
}

impl RObject for MessageSource {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            MessageSource::ChatEventLog(t) => t.extra(),
            MessageSource::ChatHistory(t) => t.extra(),
            MessageSource::ChatList(t) => t.extra(),
            MessageSource::ForumTopicHistory(t) => t.extra(),
            MessageSource::HistoryPreview(t) => t.extra(),
            MessageSource::MessageThreadHistory(t) => t.extra(),
            MessageSource::Notification(t) => t.extra(),
            MessageSource::Other(t) => t.extra(),
            MessageSource::Screenshot(t) => t.extra(),
            MessageSource::Search(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            MessageSource::ChatEventLog(t) => t.client_id(),
            MessageSource::ChatHistory(t) => t.client_id(),
            MessageSource::ChatList(t) => t.client_id(),
            MessageSource::ForumTopicHistory(t) => t.client_id(),
            MessageSource::HistoryPreview(t) => t.client_id(),
            MessageSource::MessageThreadHistory(t) => t.client_id(),
            MessageSource::Notification(t) => t.client_id(),
            MessageSource::Other(t) => t.client_id(),
            MessageSource::Screenshot(t) => t.client_id(),
            MessageSource::Search(t) => t.client_id(),

            _ => None,
        }
    }
}

impl MessageSource {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MessageSource::_Default)
    }
}

impl AsRef<MessageSource> for MessageSource {
    fn as_ref(&self) -> &MessageSource {
        self
    }
}

/// The message is from a chat event log
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceChatEventLog {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageSourceChatEventLog {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSource for MessageSourceChatEventLog {}

impl MessageSourceChatEventLog {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSourceChatEventLogBuilder {
        let mut inner = MessageSourceChatEventLog::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSourceChatEventLogBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MessageSourceChatEventLogBuilder {
    inner: MessageSourceChatEventLog,
}

#[deprecated]
pub type RTDMessageSourceChatEventLogBuilder = MessageSourceChatEventLogBuilder;

impl MessageSourceChatEventLogBuilder {
    pub fn build(&self) -> MessageSourceChatEventLog {
        self.inner.clone()
    }
}

impl AsRef<MessageSourceChatEventLog> for MessageSourceChatEventLog {
    fn as_ref(&self) -> &MessageSourceChatEventLog {
        self
    }
}

impl AsRef<MessageSourceChatEventLog> for MessageSourceChatEventLogBuilder {
    fn as_ref(&self) -> &MessageSourceChatEventLog {
        &self.inner
    }
}

/// The message is from a chat history
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceChatHistory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageSourceChatHistory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSource for MessageSourceChatHistory {}

impl MessageSourceChatHistory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSourceChatHistoryBuilder {
        let mut inner = MessageSourceChatHistory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSourceChatHistoryBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MessageSourceChatHistoryBuilder {
    inner: MessageSourceChatHistory,
}

#[deprecated]
pub type RTDMessageSourceChatHistoryBuilder = MessageSourceChatHistoryBuilder;

impl MessageSourceChatHistoryBuilder {
    pub fn build(&self) -> MessageSourceChatHistory {
        self.inner.clone()
    }
}

impl AsRef<MessageSourceChatHistory> for MessageSourceChatHistory {
    fn as_ref(&self) -> &MessageSourceChatHistory {
        self
    }
}

impl AsRef<MessageSourceChatHistory> for MessageSourceChatHistoryBuilder {
    fn as_ref(&self) -> &MessageSourceChatHistory {
        &self.inner
    }
}

/// The message is from a chat list or a forum topic list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceChatList {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageSourceChatList {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSource for MessageSourceChatList {}

impl MessageSourceChatList {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSourceChatListBuilder {
        let mut inner = MessageSourceChatList::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSourceChatListBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MessageSourceChatListBuilder {
    inner: MessageSourceChatList,
}

#[deprecated]
pub type RTDMessageSourceChatListBuilder = MessageSourceChatListBuilder;

impl MessageSourceChatListBuilder {
    pub fn build(&self) -> MessageSourceChatList {
        self.inner.clone()
    }
}

impl AsRef<MessageSourceChatList> for MessageSourceChatList {
    fn as_ref(&self) -> &MessageSourceChatList {
        self
    }
}

impl AsRef<MessageSourceChatList> for MessageSourceChatListBuilder {
    fn as_ref(&self) -> &MessageSourceChatList {
        &self.inner
    }
}

/// The message is from a forum topic history
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceForumTopicHistory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageSourceForumTopicHistory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSource for MessageSourceForumTopicHistory {}

impl MessageSourceForumTopicHistory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSourceForumTopicHistoryBuilder {
        let mut inner = MessageSourceForumTopicHistory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSourceForumTopicHistoryBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MessageSourceForumTopicHistoryBuilder {
    inner: MessageSourceForumTopicHistory,
}

#[deprecated]
pub type RTDMessageSourceForumTopicHistoryBuilder = MessageSourceForumTopicHistoryBuilder;

impl MessageSourceForumTopicHistoryBuilder {
    pub fn build(&self) -> MessageSourceForumTopicHistory {
        self.inner.clone()
    }
}

impl AsRef<MessageSourceForumTopicHistory> for MessageSourceForumTopicHistory {
    fn as_ref(&self) -> &MessageSourceForumTopicHistory {
        self
    }
}

impl AsRef<MessageSourceForumTopicHistory> for MessageSourceForumTopicHistoryBuilder {
    fn as_ref(&self) -> &MessageSourceForumTopicHistory {
        &self.inner
    }
}

/// The message is from chat, message thread or forum topic history preview
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceHistoryPreview {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageSourceHistoryPreview {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSource for MessageSourceHistoryPreview {}

impl MessageSourceHistoryPreview {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSourceHistoryPreviewBuilder {
        let mut inner = MessageSourceHistoryPreview::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSourceHistoryPreviewBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MessageSourceHistoryPreviewBuilder {
    inner: MessageSourceHistoryPreview,
}

#[deprecated]
pub type RTDMessageSourceHistoryPreviewBuilder = MessageSourceHistoryPreviewBuilder;

impl MessageSourceHistoryPreviewBuilder {
    pub fn build(&self) -> MessageSourceHistoryPreview {
        self.inner.clone()
    }
}

impl AsRef<MessageSourceHistoryPreview> for MessageSourceHistoryPreview {
    fn as_ref(&self) -> &MessageSourceHistoryPreview {
        self
    }
}

impl AsRef<MessageSourceHistoryPreview> for MessageSourceHistoryPreviewBuilder {
    fn as_ref(&self) -> &MessageSourceHistoryPreview {
        &self.inner
    }
}

/// The message is from a message thread history
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceMessageThreadHistory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageSourceMessageThreadHistory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSource for MessageSourceMessageThreadHistory {}

impl MessageSourceMessageThreadHistory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSourceMessageThreadHistoryBuilder {
        let mut inner = MessageSourceMessageThreadHistory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSourceMessageThreadHistoryBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MessageSourceMessageThreadHistoryBuilder {
    inner: MessageSourceMessageThreadHistory,
}

#[deprecated]
pub type RTDMessageSourceMessageThreadHistoryBuilder = MessageSourceMessageThreadHistoryBuilder;

impl MessageSourceMessageThreadHistoryBuilder {
    pub fn build(&self) -> MessageSourceMessageThreadHistory {
        self.inner.clone()
    }
}

impl AsRef<MessageSourceMessageThreadHistory> for MessageSourceMessageThreadHistory {
    fn as_ref(&self) -> &MessageSourceMessageThreadHistory {
        self
    }
}

impl AsRef<MessageSourceMessageThreadHistory> for MessageSourceMessageThreadHistoryBuilder {
    fn as_ref(&self) -> &MessageSourceMessageThreadHistory {
        &self.inner
    }
}

/// The message is from a notification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceNotification {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageSourceNotification {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSource for MessageSourceNotification {}

impl MessageSourceNotification {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSourceNotificationBuilder {
        let mut inner = MessageSourceNotification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSourceNotificationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MessageSourceNotificationBuilder {
    inner: MessageSourceNotification,
}

#[deprecated]
pub type RTDMessageSourceNotificationBuilder = MessageSourceNotificationBuilder;

impl MessageSourceNotificationBuilder {
    pub fn build(&self) -> MessageSourceNotification {
        self.inner.clone()
    }
}

impl AsRef<MessageSourceNotification> for MessageSourceNotification {
    fn as_ref(&self) -> &MessageSourceNotification {
        self
    }
}

impl AsRef<MessageSourceNotification> for MessageSourceNotificationBuilder {
    fn as_ref(&self) -> &MessageSourceNotification {
        &self.inner
    }
}

/// The message is from some other source
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceOther {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageSourceOther {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSource for MessageSourceOther {}

impl MessageSourceOther {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSourceOtherBuilder {
        let mut inner = MessageSourceOther::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSourceOtherBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MessageSourceOtherBuilder {
    inner: MessageSourceOther,
}

#[deprecated]
pub type RTDMessageSourceOtherBuilder = MessageSourceOtherBuilder;

impl MessageSourceOtherBuilder {
    pub fn build(&self) -> MessageSourceOther {
        self.inner.clone()
    }
}

impl AsRef<MessageSourceOther> for MessageSourceOther {
    fn as_ref(&self) -> &MessageSourceOther {
        self
    }
}

impl AsRef<MessageSourceOther> for MessageSourceOtherBuilder {
    fn as_ref(&self) -> &MessageSourceOther {
        &self.inner
    }
}

/// The message was screenshotted; the source must be used only if the message content was visible during the screenshot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceScreenshot {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageSourceScreenshot {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSource for MessageSourceScreenshot {}

impl MessageSourceScreenshot {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSourceScreenshotBuilder {
        let mut inner = MessageSourceScreenshot::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSourceScreenshotBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MessageSourceScreenshotBuilder {
    inner: MessageSourceScreenshot,
}

#[deprecated]
pub type RTDMessageSourceScreenshotBuilder = MessageSourceScreenshotBuilder;

impl MessageSourceScreenshotBuilder {
    pub fn build(&self) -> MessageSourceScreenshot {
        self.inner.clone()
    }
}

impl AsRef<MessageSourceScreenshot> for MessageSourceScreenshot {
    fn as_ref(&self) -> &MessageSourceScreenshot {
        self
    }
}

impl AsRef<MessageSourceScreenshot> for MessageSourceScreenshotBuilder {
    fn as_ref(&self) -> &MessageSourceScreenshot {
        &self.inner
    }
}

/// The message is from search results, including file downloads, local file list, outgoing document messages, calendar
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSourceSearch {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageSourceSearch {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageSource for MessageSourceSearch {}

impl MessageSourceSearch {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSourceSearchBuilder {
        let mut inner = MessageSourceSearch::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSourceSearchBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MessageSourceSearchBuilder {
    inner: MessageSourceSearch,
}

#[deprecated]
pub type RTDMessageSourceSearchBuilder = MessageSourceSearchBuilder;

impl MessageSourceSearchBuilder {
    pub fn build(&self) -> MessageSourceSearch {
        self.inner.clone()
    }
}

impl AsRef<MessageSourceSearch> for MessageSourceSearch {
    fn as_ref(&self) -> &MessageSourceSearch {
        self
    }
}

impl AsRef<MessageSourceSearch> for MessageSourceSearchBuilder {
    fn as_ref(&self) -> &MessageSourceSearch {
        &self.inner
    }
}
