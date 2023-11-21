use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of message viewers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageViewers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of message viewers

    #[serde(default)]
    viewers: Vec<MessageViewer>,
}

impl RObject for MessageViewers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageViewers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageViewersBuilder {
        let mut inner = MessageViewers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageViewersBuilder { inner }
    }

    pub fn viewers(&self) -> &Vec<MessageViewer> {
        &self.viewers
    }
}

#[doc(hidden)]
pub struct MessageViewersBuilder {
    inner: MessageViewers,
}

#[deprecated]
pub type RTDMessageViewersBuilder = MessageViewersBuilder;

impl MessageViewersBuilder {
    pub fn build(&self) -> MessageViewers {
        self.inner.clone()
    }

    pub fn viewers(&mut self, viewers: Vec<MessageViewer>) -> &mut Self {
        self.inner.viewers = viewers;
        self
    }
}

impl AsRef<MessageViewers> for MessageViewers {
    fn as_ref(&self) -> &MessageViewers {
        self
    }
}

impl AsRef<MessageViewers> for MessageViewersBuilder {
    fn as_ref(&self) -> &MessageViewers {
        &self.inner
    }
}
