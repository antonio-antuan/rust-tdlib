use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTML code for embedding the message. Available only for messages in supergroups and channels with a username
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageEmbeddingCode {
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
    /// Pass true to return an HTML code for embedding of the whole media album

    #[serde(default)]
    for_album: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageEmbeddingCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessageEmbeddingCode {}

impl GetMessageEmbeddingCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetMessageEmbeddingCodeBuilder {
        let mut inner = GetMessageEmbeddingCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageEmbeddingCode".to_string();

        GetMessageEmbeddingCodeBuilder { inner }
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
}

#[doc(hidden)]
pub struct GetMessageEmbeddingCodeBuilder {
    inner: GetMessageEmbeddingCode,
}

#[deprecated]
pub type RTDGetMessageEmbeddingCodeBuilder = GetMessageEmbeddingCodeBuilder;

impl GetMessageEmbeddingCodeBuilder {
    pub fn build(&self) -> GetMessageEmbeddingCode {
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
}

impl AsRef<GetMessageEmbeddingCode> for GetMessageEmbeddingCode {
    fn as_ref(&self) -> &GetMessageEmbeddingCode {
        self
    }
}

impl AsRef<GetMessageEmbeddingCode> for GetMessageEmbeddingCodeBuilder {
    fn as_ref(&self) -> &GetMessageEmbeddingCode {
        &self.inner
    }
}
