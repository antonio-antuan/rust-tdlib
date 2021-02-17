use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTPS link to a message in a chat. Available only for already sent messages in supergroups and channels. This is an offline request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the message belongs
    chat_id: i64,
    /// Identifier of the message
    message_id: i64,
    /// Pass true to create a link for the whole media album
    for_album: bool,
    /// Pass true to create a link to the message as a channel post comment, or from a message thread
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetMessageLinkBuilder {
        let mut inner = GetMessageLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageLink".to_string();

        RTDGetMessageLinkBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn for_album(&self) -> bool {
        self.for_album
    }

    pub fn for_comment(&self) -> bool {
        self.for_comment
    }
}

#[doc(hidden)]
pub struct RTDGetMessageLinkBuilder {
    inner: GetMessageLink,
}

impl RTDGetMessageLinkBuilder {
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

impl AsRef<GetMessageLink> for RTDGetMessageLinkBuilder {
    fn as_ref(&self) -> &GetMessageLink {
        &self.inner
    }
}
