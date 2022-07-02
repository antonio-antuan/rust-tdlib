use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that the message content has been opened (e.g., the user has opened a photo, video, document, location or venue, or has listened to an audio file or voice note message). An updateMessageContentOpened update will be generated if something has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenMessageContent {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier of the message

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message with the opened content

    #[serde(default)]
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for OpenMessageContent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for OpenMessageContent {}

impl OpenMessageContent {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> OpenMessageContentBuilder {
        let mut inner = OpenMessageContent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "openMessageContent".to_string();

        OpenMessageContentBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct OpenMessageContentBuilder {
    inner: OpenMessageContent,
}

#[deprecated]
pub type RTDOpenMessageContentBuilder = OpenMessageContentBuilder;

impl OpenMessageContentBuilder {
    pub fn build(&self) -> OpenMessageContent {
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
}

impl AsRef<OpenMessageContent> for OpenMessageContent {
    fn as_ref(&self) -> &OpenMessageContent {
        self
    }
}

impl AsRef<OpenMessageContent> for OpenMessageContentBuilder {
    fn as_ref(&self) -> &OpenMessageContent {
        &self.inner
    }
}
