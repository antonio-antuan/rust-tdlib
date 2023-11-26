use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a sponsored message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SponsoredMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message identifier; unique for the chat to which the sponsored message belongs among both ordinary and sponsored messages

    #[serde(default)]
    message_id: i64,
    /// True, if the message needs to be labeled as "recommended" instead of "sponsored"

    #[serde(default)]
    is_recommended: bool,
    /// Content of the message. Currently, can be only of the type messageText

    #[serde(skip_serializing_if = "MessageContent::_is_default")]
    content: MessageContent,
    /// Information about the sponsor of the message
    sponsor: MessageSponsor,
    /// If non-empty, additional information about the sponsored message to be shown along with the message

    #[serde(default)]
    additional_info: String,
}

impl RObject for SponsoredMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl SponsoredMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SponsoredMessageBuilder {
        let mut inner = SponsoredMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SponsoredMessageBuilder { inner }
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn is_recommended(&self) -> bool {
        self.is_recommended
    }

    pub fn content(&self) -> &MessageContent {
        &self.content
    }

    pub fn sponsor(&self) -> &MessageSponsor {
        &self.sponsor
    }

    pub fn additional_info(&self) -> &String {
        &self.additional_info
    }
}

#[doc(hidden)]
pub struct SponsoredMessageBuilder {
    inner: SponsoredMessage,
}

#[deprecated]
pub type RTDSponsoredMessageBuilder = SponsoredMessageBuilder;

impl SponsoredMessageBuilder {
    pub fn build(&self) -> SponsoredMessage {
        self.inner.clone()
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn is_recommended(&mut self, is_recommended: bool) -> &mut Self {
        self.inner.is_recommended = is_recommended;
        self
    }

    pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
        self.inner.content = content.as_ref().clone();
        self
    }

    pub fn sponsor<T: AsRef<MessageSponsor>>(&mut self, sponsor: T) -> &mut Self {
        self.inner.sponsor = sponsor.as_ref().clone();
        self
    }

    pub fn additional_info<T: AsRef<str>>(&mut self, additional_info: T) -> &mut Self {
        self.inner.additional_info = additional_info.as_ref().to_string();
        self
    }
}

impl AsRef<SponsoredMessage> for SponsoredMessage {
    fn as_ref(&self) -> &SponsoredMessage {
        self
    }
}

impl AsRef<SponsoredMessage> for SponsoredMessageBuilder {
    fn as_ref(&self) -> &SponsoredMessage {
        &self.inner
    }
}
