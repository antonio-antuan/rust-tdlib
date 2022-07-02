use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTPS link to a message in a chat. Available only for already sent messages in supergroups and channels, or if message.can_get_media_timestamp_links and a media timestamp link is generated. This is an offline request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the message belongs

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message

    #[serde(default)]
    message_id: i64,
    /// If not 0, timestamp from which the video/audio/video note/voice note playing must start, in seconds. The media can be in the message content or in its web page preview

    #[serde(default)]
    media_timestamp: i32,
    /// Pass true to create a link for the whole media album

    #[serde(default)]
    for_album: bool,
    /// Pass true to create a link to the message as a channel post comment, or from a message thread

    #[serde(default)]
    for_comment: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessageLink {}

impl GetMessageLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetMessageLinkBuilder {
        let mut inner = GetMessageLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageLink".to_string();

        GetMessageLinkBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn media_timestamp(&self) -> i32 {
        self.media_timestamp
    }

    pub fn for_album(&self) -> bool {
        self.for_album
    }

    pub fn for_comment(&self) -> bool {
        self.for_comment
    }
}

#[doc(hidden)]
pub struct GetMessageLinkBuilder {
    inner: GetMessageLink,
}

#[deprecated]
pub type RTDGetMessageLinkBuilder = GetMessageLinkBuilder;

impl GetMessageLinkBuilder {
    pub fn build(&self) -> GetMessageLink {
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

    pub fn media_timestamp(&mut self, media_timestamp: i32) -> &mut Self {
        self.inner.media_timestamp = media_timestamp;
        self
    }

    pub fn for_album(&mut self, for_album: bool) -> &mut Self {
        self.inner.for_album = for_album;
        self
    }

    pub fn for_comment(&mut self, for_comment: bool) -> &mut Self {
        self.inner.for_comment = for_comment;
        self
    }
}

impl AsRef<GetMessageLink> for GetMessageLink {
    fn as_ref(&self) -> &GetMessageLink {
        self
    }
}

impl AsRef<GetMessageLink> for GetMessageLinkBuilder {
    fn as_ref(&self) -> &GetMessageLink {
        &self.inner
    }
}
