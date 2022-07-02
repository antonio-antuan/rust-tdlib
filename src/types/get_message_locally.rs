use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a message, if it is available locally without sending network request. This is an offline request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageLocally {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat the message belongs to

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message to get

    #[serde(default)]
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageLocally {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessageLocally {}

impl GetMessageLocally {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetMessageLocallyBuilder {
        let mut inner = GetMessageLocally::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageLocally".to_string();

        GetMessageLocallyBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct GetMessageLocallyBuilder {
    inner: GetMessageLocally,
}

#[deprecated]
pub type RTDGetMessageLocallyBuilder = GetMessageLocallyBuilder;

impl GetMessageLocallyBuilder {
    pub fn build(&self) -> GetMessageLocally {
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

impl AsRef<GetMessageLocally> for GetMessageLocally {
    fn as_ref(&self) -> &GetMessageLocally {
        self
    }
}

impl AsRef<GetMessageLocally> for GetMessageLocallyBuilder {
    fn as_ref(&self) -> &GetMessageLocally {
        &self.inner
    }
}
